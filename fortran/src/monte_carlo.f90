module monte_carlo
    use iso_c_binding
    use iso_fortran_env, only: dp => real64
    implicit none

    ! Make module variables visible to other modules
    public :: f_init_simulation, f_run_simulation, f_get_scatter_data, f_get_image_data
    public :: f_run_line_scan, f_get_line_data
    public :: scatter_positions, num_electrons, line_scan_data

    ! Physical constants
    real(dp), parameter :: ELECTRON_MASS = 9.10938356e-31_dp  ! kg
    real(dp), parameter :: ELECTRON_CHARGE = 1.60217662e-19_dp ! C
    real(dp), parameter :: VACUUM_PERMITTIVITY = 8.8541878128e-12_dp ! F/m
    real(dp), parameter :: PLANCK_CONSTANT = 6.62607015e-34_dp ! J⋅s
    real(dp), parameter :: SPEED_OF_LIGHT = 2.99792458e8_dp ! m/s
    real(dp), parameter :: PI = 3.141592653589793_dp
    real(dp), parameter :: AVOGADRO = 6.02214076e23_dp ! mol⁻¹
    real(dp), parameter :: BOHR_RADIUS = 5.29177210903e-11_dp ! m
    real(dp), parameter :: FINE_STRUCTURE = 1.0_dp/137.035999084_dp
    real(dp), parameter :: REST_MASS_ENERGY = 511.0_dp ! keV

    ! Simulation parameters
    integer, parameter :: MAX_ELECTRONS = 100000
    integer :: num_electrons
    real(dp), allocatable, target :: scatter_positions(:,:)  ! (x,y,z,energy) for each electron
    real(dp), allocatable :: surface_heights(:,:)           ! Surface topography
    real(dp), allocatable :: material_properties(:,:,:)     ! Composition and crystal orientation
    real(dp), allocatable, target :: line_scan_data(:,:)   ! Line scan intensity data
    real(dp), allocatable, target :: image_buffer(:,:)  ! 2D image buffer
    integer :: image_width, image_height
    
    ! Sample properties (Iron Oxide - Fe2O3)
    real(dp), parameter :: FE_ATOMIC_NUMBER = 26.0_dp
    real(dp), parameter :: O_ATOMIC_NUMBER = 8.0_dp
    real(dp), parameter :: DENSITY = 5.24_dp  ! g/cm³
    real(dp), parameter :: CRYSTAL_SIZE = 50.0_dp  ! nm
    real(dp), parameter :: SURFACE_ROUGHNESS = 10.0_dp  ! nm
    real(dp), parameter :: MEAN_IONIZATION_POTENTIAL = 286.0_dp  ! eV (Fe2O3)
    
    ! Beam parameters
    real(dp) :: beam_energy           ! keV
    real(dp) :: beam_current         ! nA
    real(dp) :: spot_size           ! nm
    real(dp) :: working_distance    ! mm
    real(dp) :: scan_resolution     ! pixels
    real(dp) :: dwell_time         ! μs
    logical :: is_line_scan = .false. ! Mode switch

contains
    subroutine f_init_simulation(energy, current, resolution, distance) bind(C, name="f_init_simulation")
        real(c_double), value :: energy    ! Beam energy in keV
        real(c_double), value :: current   ! Beam current in nA
        integer(c_int), value :: resolution ! Image resolution in pixels
        real(c_double), value :: distance  ! Working distance in mm
        integer :: i, j
        real(dp) :: rand

        beam_energy = energy
        beam_current = current
        working_distance = distance
        scan_resolution = real(resolution, dp)
        is_line_scan = .false.
        
        ! Calculate number of electrons based on beam current and dwell time
        dwell_time = 1.0e-6_dp  ! 1 microsecond default dwell time
        num_electrons = min(int(beam_current * 6.242e9_dp * dwell_time), MAX_ELECTRONS)
        
        ! Initialize arrays
        if (allocated(scatter_positions)) deallocate(scatter_positions)
        if (allocated(surface_heights)) deallocate(surface_heights)
        if (allocated(material_properties)) deallocate(material_properties)
        if (allocated(line_scan_data)) deallocate(line_scan_data)
        
        allocate(scatter_positions(4, num_electrons))
        allocate(surface_heights(resolution, resolution))
        allocate(material_properties(resolution, resolution, 3))
        
        ! Generate realistic surface topography
        do i = 1, resolution
            do j = 1, resolution
                call random_number(rand)
                surface_heights(i,j) = generate_surface_feature(i, j, resolution)
            end do
        end do
        
        ! Initialize material properties with crystalline structure
        call initialize_crystal_structure(resolution)

        ! Initialize image buffer
        image_width = resolution
        image_height = resolution
        if (allocated(image_buffer)) deallocate(image_buffer)
        allocate(image_buffer(image_width, image_height))
        image_buffer = 0.0_dp
    end subroutine f_init_simulation

    function generate_surface_feature(x, y, size) result(height)
        integer, intent(in) :: x, y, size
        real(dp) :: height, dx, dy, dist
        real(dp) :: rand
        integer :: num_crystals, i
        
        height = 0.0_dp
        num_crystals = size/10  ! Approximate number of crystal grains
        
        ! Add multiple crystalline features
        do i = 1, num_crystals
            dx = x - size/2
            dy = y - size/2
            dist = sqrt(dx*dx + dy*dy)
            
            call random_number(rand)
            height = height + CRYSTAL_SIZE * exp(-dist/SURFACE_ROUGHNESS) * &
                    (1.0_dp + 0.2_dp * (rand - 0.5_dp))
        end do
    end function generate_surface_feature

    subroutine initialize_crystal_structure(size)
        integer, intent(in) :: size
        integer :: i, j
        real(dp) :: rand, orientation
        
        do i = 1, size
            do j = 1, size
                ! Set iron and oxygen concentrations
                material_properties(i,j,1) = 0.4_dp  ! Fe concentration
                material_properties(i,j,2) = 0.6_dp  ! O concentration
                
                ! Set crystal orientation (0 to 2π)
                call random_number(rand)
                orientation = 2.0_dp * PI * rand
                material_properties(i,j,3) = orientation
            end do
        end do
    end subroutine initialize_crystal_structure

    subroutine f_run_simulation() bind(C, name="f_run_simulation")
        integer :: i, j, num_collisions, pixel_x, pixel_y
        real(dp) :: energy, path_length, mfp
        real(dp) :: x, y, z, dx, dy, dz
        real(dp) :: theta, phi, energy_loss
        real(dp) :: se_yield, pixel_size
        real(dp) :: scan_x, scan_y
        logical :: generate_se

        ! Clear image buffer
        image_buffer = 0.0_dp
        
        ! Calculate pixel size based on a typical 10μm field of view
        pixel_size = 10000.0_dp / image_width  ! nm per pixel
        
        ! Scan over the surface
        do j = 1, image_height
            do i = 1, image_width
                ! Calculate beam position
                scan_x = (i - image_width/2) * pixel_size
                scan_y = (j - image_height/2) * pixel_size
                
                ! Run multiple electrons per pixel
                do k = 1, num_electrons/image_width/image_height
                    ! Initialize electron at surface with beam position
                    energy = beam_energy
                    x = scan_x
                    y = scan_y
                    z = 0.0_dp
                    dx = 0.0_dp
                    dy = 0.0_dp
                    dz = 1.0_dp  ! Initial direction along z-axis
                    
                    ! Add initial beam spread
                    call beam_spread(dx, dy, dz)
                    
                    ! Track electron until it's absorbed or escapes
                    do while (z >= 0.0_dp .and. energy > 0.1_dp)
                        ! Calculate mean free path (including both elastic and inelastic)
                        mfp = calculate_mfp(energy)
                        
                        ! Sample path length (exponential distribution)
                        call random_number(path_length)
                        path_length = -mfp * log(path_length)
                        
                        ! Move electron
                        x = x + path_length * dx
                        y = y + path_length * dy
                        z = z + path_length * dz
                        
                        ! Determine if this collision generates SE
                        call random_number(se_yield)
                        generate_se = se_yield < calculate_se_yield(energy)
                        
                        ! Calculate scattering angles using screened Rutherford
                        call calculate_scatter_angles(energy, theta, phi)
                        
                        ! Update direction
                        call update_direction(dx, dy, dz, theta, phi)
                        
                        ! Calculate energy loss (Bethe formula with straggling)
                        energy_loss = calculate_energy_loss(energy, path_length)
                        energy = energy - energy_loss
                        
                        ! If electron escapes surface (backscattered)
                        if (z < 0.0_dp) then
                            ! Add to image intensity with distance-based weighting
                            pixel_x = nint((x + (image_width/2) * pixel_size) / pixel_size)
                            pixel_y = nint((y + (image_height/2) * pixel_size) / pixel_size)
                            
                            if (pixel_x >= 1 .and. pixel_x <= image_width .and. &
                                pixel_y >= 1 .and. pixel_y <= image_height) then
                                image_buffer(pixel_x, pixel_y) = &
                                    image_buffer(pixel_x, pixel_y) + energy/beam_energy
                            end if
                        end if
                    end do
                end do
            end do
        end do
        
        ! Normalize image
        image_buffer = image_buffer / maxval(image_buffer)
    end subroutine f_run_simulation
    
    ! Helper functions
    
    subroutine beam_spread(dx, dy, dz)
        real(dp), intent(inout) :: dx, dy, dz
        real(dp) :: angle_x, angle_y, norm
        
        call random_number(angle_x)
        call random_number(angle_y)
        
        ! Gaussian beam profile
        angle_x = sqrt(-2.0_dp * log(angle_x)) * cos(2.0_dp * PI * angle_y) * spot_size
        angle_y = sqrt(-2.0_dp * log(angle_x)) * sin(2.0_dp * PI * angle_y) * spot_size
        
        dx = sin(angle_x)
        dy = sin(angle_y)
        dz = sqrt(1.0_dp - dx*dx - dy*dy)  ! Normalize
        
        ! Ensure normalization
        norm = sqrt(dx*dx + dy*dy + dz*dz)
        dx = dx/norm
        dy = dy/norm
        dz = dz/norm
    end subroutine beam_spread
    
    function calculate_mfp(energy) result(mfp)
        real(dp), intent(in) :: energy
        real(dp) :: mfp, cross_section, elastic_cs, inelastic_cs
        
        ! Elastic cross-section (screened Rutherford)
        elastic_cs = 5.21e-21_dp * (FE_ATOMIC_NUMBER**2) / (energy**2) * &
                    (1.0_dp + energy/REST_MASS_ENERGY)**(-2)  ! Relativistic correction
        
        ! Inelastic cross-section (Møller)
        inelastic_cs = 2.0_dp * PI * (BOHR_RADIUS**2) * FINE_STRUCTURE * &
                      (FE_ATOMIC_NUMBER/energy) * (log(energy/MEAN_IONIZATION_POTENTIAL) + 0.198_dp)
        
        ! Total cross-section
        cross_section = elastic_cs + inelastic_cs
        
        ! Mean free path
        mfp = 1.0_dp / (DENSITY * AVOGADRO * cross_section / (FE_ATOMIC_NUMBER*1.0_dp + O_ATOMIC_NUMBER*1.0_dp))
        mfp = mfp * 1.0e9_dp  ! Convert to nm
    end function calculate_mfp
    
    subroutine calculate_scatter_angles(energy, theta, phi)
        real(dp), intent(in) :: energy
        real(dp), intent(out) :: theta, phi
        real(dp) :: rand, eta, screening_parameter
        
        ! TF screening parameter
        screening_parameter = 0.885_dp * BOHR_RADIUS * FE_ATOMIC_NUMBER**(-1.0_dp/3.0_dp)
        eta = 2.0_dp * energy * screening_parameter / (FINE_STRUCTURE * BOHR_RADIUS)
        
        ! Sample theta from screened Rutherford
        call random_number(rand)
        theta = acos(1.0_dp - 2.0_dp * rand/(1.0_dp + eta * (1.0_dp - rand)))
        
        ! Uniform phi
        call random_number(rand)
        phi = 2.0_dp * PI * rand
    end subroutine calculate_scatter_angles
    
    function calculate_se_yield(energy) result(yield)
        real(dp), intent(in) :: energy
        real(dp) :: yield, E_max
        
        ! Maximum SE yield energy (typically 1-2 keV for most materials)
        E_max = 1.5_dp  ! keV for Si
        
        ! Empirical SE yield formula (Sternglass)
        yield = 1.36_dp * (energy/E_max) * exp(-2.3_dp * (energy/E_max)**0.5_dp)
    end function calculate_se_yield
    
    function calculate_energy_loss(energy, path_length) result(energy_loss)
        real(dp), intent(in) :: energy, path_length
        real(dp) :: energy_loss, stopping_power
        real(dp) :: beta2, gamma, rand
        
        ! Relativistic factors
        beta2 = 1.0_dp - 1.0_dp/(1.0_dp + energy/REST_MASS_ENERGY)**2
        gamma = 1.0_dp/sqrt(1.0_dp - beta2)
        
        ! Bethe formula with density effect correction
        stopping_power = 0.1536_dp * (DENSITY/FE_ATOMIC_NUMBER) * (FE_ATOMIC_NUMBER/beta2) * &
                        (log(2.0_dp*ELECTRON_MASS*beta2*gamma**2/MEAN_IONIZATION_POTENTIAL) - &
                         beta2 - calculate_density_effect(beta2))
        
        ! Add energy straggling (Landau-Vavilov)
        call random_number(rand)
        energy_loss = stopping_power * path_length * (1.0_dp + 0.1_dp * (2.0_dp * rand - 1.0_dp))
        
        ! Convert to keV
        energy_loss = energy_loss * 1.0e-3_dp
    end function calculate_energy_loss
    
    function calculate_density_effect(beta2) result(delta)
        real(dp), intent(in) :: beta2
        real(dp) :: delta, x
        
        ! Simplified Sternheimer formula
        x = log10(sqrt(beta2/(1.0_dp - beta2)))
        if (x < 0.0_dp) then
            delta = 0.0_dp
        else
            delta = 4.606_dp * x - 1.0_dp
        end if
    end function calculate_density_effect

    function f_get_scatter_data() result(data)
        real(dp), pointer :: data(:,:)
        data => scatter_positions
    end function f_get_scatter_data

    subroutine f_run_line_scan(start_x, end_x, num_points) bind(C, name="f_run_line_scan")
        real(c_double), value :: start_x, end_x  ! Line scan start and end positions in nm
        integer(c_int), value :: num_points      ! Number of points in the line scan
        integer :: i
        real(dp) :: x, step_size
        
        is_line_scan = .true.
        
        ! Allocate line scan data array (position, intensity)
        if (allocated(line_scan_data)) deallocate(line_scan_data)
        allocate(line_scan_data(2, num_points))
        
        step_size = (end_x - start_x) / (num_points - 1)
        
        ! Perform line scan
        do i = 1, num_points
            x = start_x + (i-1) * step_size
            line_scan_data(1, i) = x  ! Position
            
            ! Run simulation at this point
            call simulate_point(x, 0.0_dp)  ! y=0 for line scan
            
            ! Calculate intensity from scattered electrons
            line_scan_data(2, i) = calculate_intensity()
        end do
    end subroutine f_run_line_scan

    function f_get_line_data() result(data)
        real(dp), pointer :: data(:,:)
        data => line_scan_data
    end function f_get_line_data

    subroutine simulate_point(x, y)
        real(dp), intent(in) :: x, y
        ! ... implementation of single point simulation ...
        ! This will be similar to f_run_simulation but for a single point
    end subroutine simulate_point

    function calculate_intensity() result(intensity)
        real(dp) :: intensity
        integer :: i
        
        intensity = 0.0_dp
        do i = 1, num_electrons
            if (scatter_positions(3,i) < 0.0_dp) then  ! Backscattered electron
                intensity = intensity + 1.0_dp
            end if
        end do
        intensity = intensity / num_electrons
    end function calculate_intensity

    function f_get_image_data() result(data)
        real(dp), pointer :: data(:,:)
        data => image_buffer
    end function f_get_image_data
end module monte_carlo
