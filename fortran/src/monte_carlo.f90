! module monte_carlo
!     implicit none
!     private
!     public :: init_simulation, run_simulation, get_scatter_data

!     integer, parameter :: dp = kind(1.0d0)
!     integer, parameter :: max_electron = 100000

!     real(dp) :: sample_thickness
!     real(dp) :: beam_energy
!     real(dp) :: scattering_angle_stddev

!     real(dp), allocatable :: scatter_positions(:,:)
!     integer :: num_electrons  ! Declare globally
module monte_carlo
    use iso_c_binding
    implicit none

    ! Make module variables visible to other modules
    public :: init_simulation, run_simulation, get_scatter_data
    public :: scatter_positions, num_electrons

    integer, parameter :: dp = kind(1.0d0)
    integer, parameter :: max_electron = 100000
    real(dp) :: beam_energy, sample_thickness, scattering_angle_stddev
    integer :: num_electrons
    real(dp), allocatable :: scatter_positions(:,:)


contains
    subroutine init_simulation(energy, thickness, angle_stddev, n_electrons)
        real(dp), intent(in) :: energy, thickness, angle_stddev
        integer, intent(in) :: n_electrons

        beam_energy = energy
        sample_thickness = thickness
        scattering_angle_stddev = angle_stddev
        num_electrons = min(n_electrons, max_electron)

        allocate(scatter_positions(3, num_electrons))
        scatter_positions = 0.0_dp

    end subroutine init_simulation

    subroutine run_simulation()
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
    end subroutine run_simulation
    
    function get_scatter_data() result(data)
        real(dp), allocatable :: data(:,:)  ! Make data allocatable to return values
        allocate(data(3, size(scatter_positions, 2)))  ! Allocate data dynamically
        data = scatter_positions  ! Return the data
    end function get_scatter_data
end module monte_carlo
