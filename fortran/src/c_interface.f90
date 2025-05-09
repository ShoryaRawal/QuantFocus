module c_interface
    use iso_c_binding
    use monte_carlo, only: get_scatter_data, run_simulation, init_simulation
    implicit none

    ! Add these module-scope buffers for persistence
    real(c_double), allocatable, target, save :: scatter_temp(:,:)
    real(c_double), pointer, save :: scatter_flat(:)

contains


    subroutine c_init_simulation(energy, thickness, angle_stddev, n_electrons) bind(C, name="c_init_simulation")
        real(c_double), intent(in) :: energy, thickness, angle_stddev
        integer(c_int), intent(in) :: n_electrons
        call init_simulation(energy, thickness, angle_stddev, n_electrons)
    end subroutine

    subroutine c_run_simulation() bind(C, name="c_run_simulation")
        call run_simulation()
    end subroutine

    subroutine c_get_scatter_data(data, rows, cols) bind(C, name="c_get_scatter_data")
        use iso_c_binding
        implicit none
        real(c_double), pointer :: data(:)
        integer(c_int), intent(out) :: rows, cols
        integer :: total

        scatter_temp = get_scatter_data()
        rows = size(scatter_temp, 1)
        cols = size(scatter_temp, 2)
        total = rows * cols

        call c_f_pointer(c_loc(scatter_temp), scatter_flat, [total])
        data => scatter_flat
    end subroutine

end module c_interface
