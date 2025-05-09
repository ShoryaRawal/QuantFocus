module beam
  use iso_fortran_env, only: dp => real64
  implicit none
  private
  public :: initialize_beam, get_beam_energy, get_beam_direction, get_beam_position

  real(dp) :: beam_energy        ! in keV
  real(dp), dimension(3) :: beam_direction ! Unit vector
  real(dp), dimension(3) :: beam_position  ! Initial position (nm)

contains

  subroutine initialize_beam(energy, direction, position)
    ! Initializes the beam with specified parameters
    real(dp), intent(in) :: energy
    real(dp), dimension(3), intent(in) :: direction, position

    if (energy <= 0.0_dp) then
      print *, 'Error: Beam energy must be positive.'
      stop
    end if

    if (norm2(direction) == 0.0_dp) then
      print *, 'Error: Beam direction vector must not be zero.'
      stop
    end if

    beam_energy = energy
    beam_direction = direction / norm2(direction)  ! Normalize direction vector
    beam_position = position
  end subroutine initialize_beam

  function get_beam_energy() result(energy)
    ! Returns the beam energy in keV
    real(dp) :: energy
    energy = beam_energy
  end function get_beam_energy

  function get_beam_direction() result(direction)
    ! Returns the normalized beam direction vector
    real(dp), dimension(3) :: direction
    direction = beam_direction
  end function get_beam_direction

  function get_beam_position() result(position)
    ! Returns the beam's initial position
    real(dp), dimension(3) :: position
    position = beam_position
  end function get_beam_position

  function norm2(v) result(norm)
    ! Computes the Euclidean norm of a 3D vector
    real(dp), dimension(3), intent(in) :: v
    real(dp) :: norm
    norm = sqrt(sum(v**2))
  end function norm2

end module beam
