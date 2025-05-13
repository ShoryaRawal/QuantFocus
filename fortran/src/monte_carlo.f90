module monte_carlo
    use iso_c_binding
    implicit none

    ! Make module variables visible to other modules
    public :: f_init_simulation, f_run_simulation, f_get_scatter_data
    public :: scatter_positions, num_electrons

    integer, parameter :: dp = kind(1.0d0)
    integer, parameter :: max_electron = 100000
    real(dp) :: beam_energy, sample_thickness, scattering_angle_stddev
    integer :: num_electrons
    real(dp), allocatable, target, save :: scatter_positions(:,:)

contains
    subroutine f_init_simulation(energy, thickness, angle_stddev, n_electrons) bind(C, name="f_init_simulation")
        use iso_c_binding
        real(c_double), value :: energy, thickness, angle_stddev
        integer(c_int), value :: n_electrons

        beam_energy = energy
        sample_thickness = thickness
        scattering_angle_stddev = angle_stddev
        num_electrons = min(n_electrons, max_electron)

        allocate(scatter_positions(3, num_electrons))
        scatter_positions = 0.0_c_double
    end subroutine f_init_simulation

    subroutine f_run_simulation() bind(C, name="f_run_simulation")
        integer :: i
        real(dp) :: x, y, z, angle_x, angle_y
        
        ! Declare num_electrons locally since it's used only here
        !integer :: num_electrons

        do i = 1, num_electrons
            call random_number(angle_x)
            call random_number(angle_y)
        
            angle_x = (angle_x - 0.5_dp) * 2 * scattering_angle_stddev
            angle_y = (angle_y - 0.5_dp) * 2 * scattering_angle_stddev
            
            x = sample_thickness * tan(angle_x)  
            y = sample_thickness * tan(angle_y)  
            z = sample_thickness
            scatter_positions(:,i) = (/ x, y, z /)
        end do
    end subroutine f_run_simulation
    
    function f_get_scatter_data() result(data)
        real(dp), pointer :: data(:,:)  ! Use pointer, not allocatable
        data => scatter_positions       ! Just return the persistent memory
    end function f_get_scatter_data
end module monte_carlo
