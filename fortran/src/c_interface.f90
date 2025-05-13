module c_interface
  use iso_c_binding
  !use monte_carlo, only: init_simulation, run_simulation, get_scatter_data
  implicit none

  ! Persistent buffers
  real(c_double), allocatable, target, save :: scatter_temp(:,:)
  real(c_double), pointer,    save  :: scatter_flat(:)

contains

  subroutine c_init_simulation(energy, thickness, angle_stddev, n_electrons) bind(C, name="c_init_simulation")
    use monte_carlo, only: f_init_simulation
    real(c_double), value :: energy, thickness, angle_stddev
    integer(c_int),     value :: n_electrons
    call f_init_simulation(energy, thickness, angle_stddev, n_electrons)
  end subroutine

  subroutine c_run_simulation() bind(C, name="c_run_simulation")
    use monte_carlo, only: f_run_simulation
    call f_run_simulation()
  end subroutine

  subroutine c_get_scatter_data(data_ptr, rows, cols) bind(C, name="c_get_scatter_data")
    use monte_carlo, only: f_get_scatter_data
    ! C interface uses TYPE(C_PTR) instead of POINTER dummy
    type(c_ptr), intent(out) :: data_ptr
    integer(c_int), intent(out) :: rows, cols
    integer :: total

    ! Get the 2D scatter data
    scatter_temp = f_get_scatter_data()

    rows = size(scatter_temp, 1)    ! should be 3
    cols = size(scatter_temp, 2)    ! number of electrons
    total = rows * cols

    ! Create a Fortran pointer to the contiguous data
    call c_f_pointer(c_loc(scatter_temp), scatter_flat, [total])

    ! Return its C address
    data_ptr = c_loc(scatter_flat)
  end subroutine

end module c_interface
