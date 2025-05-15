module c_interface
  use iso_c_binding
  use iso_fortran_env, only: dp => real64
  use monte_carlo, only: f_init_simulation, f_run_simulation, f_get_scatter_data, &
                        f_run_line_scan, f_get_line_data, f_get_image_data, image_width, image_height
  !use monte_carlo, only: init_simulation, run_simulation, get_scatter_data
  implicit none

  ! Persistent buffers
  real(c_double), allocatable, target, save :: scatter_temp(:,:)
  real(c_double), pointer,    save  :: scatter_flat(:)
  real(c_double), allocatable, target, save :: line_temp(:,:)
  real(c_double), pointer,    save  :: line_flat(:)
  real(dp), pointer :: line_data(:,:)

contains

  subroutine init_simulation(energy, current, resolution, distance) bind(C, name="init_simulation")
    real(c_double), value :: energy    ! Beam energy in keV
    real(c_double), value :: current   ! Beam current in nA
    integer(c_int), value :: resolution ! Image resolution in pixels
    real(c_double), value :: distance  ! Working distance in mm
    
    call f_init_simulation(energy, current, resolution, distance)
  end subroutine init_simulation

  subroutine run_simulation() bind(C, name="run_simulation")
    call f_run_simulation()
  end subroutine run_simulation

  subroutine run_line_scan(start_x, end_x, num_points) bind(C, name="run_line_scan")
    real(c_double), value :: start_x, end_x
    integer(c_int), value :: num_points
    
    call f_run_line_scan(start_x, end_x, num_points)
  end subroutine run_line_scan

  function get_scatter_data() result(data) bind(C, name="get_scatter_data")
    type(c_ptr) :: data
    data = c_loc(f_get_scatter_data())
  end function get_scatter_data

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

  subroutine get_line_data(data_ptr, points) bind(C, name="get_line_data")
    type(c_ptr), intent(out) :: data_ptr
    integer(c_int), intent(out) :: points
    integer :: total
    
    ! Get the line scan data as a Fortran pointer
    line_data => f_get_line_data()
    
    ! Copy to our temporary buffer
    if (allocated(line_temp)) deallocate(line_temp)
    allocate(line_temp(size(line_data,1), size(line_data,2)))
    line_temp = line_data
    
    points = size(line_temp, 2)
    total = size(line_temp)
    
    ! Create a Fortran pointer to the contiguous data
    if (associated(line_flat)) nullify(line_flat)
    call c_f_pointer(c_loc(line_temp), line_flat, [total])
    
    ! Return its C address
    data_ptr = c_loc(line_flat)
  end subroutine get_line_data

  function c_get_image_data(data_ptr, width, height) bind(C, name="fortran_get_image_data")
    use iso_c_binding
    use monte_carlo, only: f_get_image_data
    implicit none
    type(c_ptr), intent(out) :: data_ptr
    integer(c_int), intent(out) :: width, height
    real(c_double), pointer :: fortran_array(:,:)
    integer :: c_get_image_data

    fortran_array => f_get_image_data()
    data_ptr = c_loc(fortran_array(1,1))
    width = image_width
    height = image_height
    c_get_image_data = 0
  end function c_get_image_data

end module c_interface
