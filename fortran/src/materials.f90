module materials
    implicit none
    private
    public :: define_material, get_atomic_number, get_density, get_mean_free_path

    integer, parameter :: dp = kind(1.0d0)
    integer, parameter :: max_materials = 100

    type :: material
        character(len=32) :: name
        real(dp) :: density
        integer :: Z
        real :: mean_free_path
        real(dp) :: get_mean_free_path
    end type material

    type(material), dimension(max_materials) :: material_list
    integer :: material_count = 0

contains
    subroutine define_material(name, Z, density, mean_free_path)
        character(len=*), intent(in) :: name
        integer, intent(in) :: Z
        real(dp), intent(in) :: density, mean_free_path
        
        if(material_count >= max_materials) then
            print *, 'ERROR: Maximum number of materials reached.'
            stop
        end if

        material_count = material_count + 1
        material_list(material_count)%name = name
        material_list(material_count)%Z = Z
        material_list(material_count)%density = density
        material_list(material_count)%mean_free_path = mean_free_path
    end subroutine define_material

    function get_atomic_number(index) result(Z)
        integer, intent(in) :: index
        integer :: Z
        
        if(index > 0 .and. index <= material_count) then
            Z = material_list(index)%Z
        else
            Z = -1
        end if
    end function get_atomic_number

    function get_density(index) result(rho)
    ! Get density of the indexed material
    integer, intent(in) :: index
    real(dp) :: rho

    if (index > 0 .and. index <= material_count) then
      rho = material_list(index)%density
    else
      rho = -1.0_dp
    end if
  end function get_density

  function get_mean_free_path(index) result(mfp)
    ! Get mean free path of the indexed material
    integer, intent(in) :: index
    real(dp) :: mfp

    if (index > 0 .and. index <= material_count) then
      mfp = material_list(index)%mean_free_path
    else
      mfp = -1.0_dp
    end if
  end function get_mean_free_path

end module materials