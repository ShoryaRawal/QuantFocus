! scattering.f90
! Implements electron scattering models in the SEM simulation

module scattering
  use iso_fortran_env, only: dp => real64
  implicit none
  private
  public :: elastic_scatter, inelastic_scatter

  real(dp), parameter :: pi = 3.141592653589793_dp
  real(dp), parameter :: electron_mass = 9.10938356e-31_dp ! in kg
  real(dp), parameter :: electron_charge = 1.60217662e-19_dp ! in C

contains

  function elastic_scatter(energy, atomic_number) result(scatter_angle)
    ! Simulates an elastic scattering angle using Rutherford model approximation
    real(dp), intent(in) :: energy      ! Beam energy in keV
    integer, intent(in) :: atomic_number
    real(dp) :: scatter_angle          ! Scattering angle in radians
    real(dp) :: theta_min, theta_max, theta, prob, rand

    call random_number(rand)

    ! Set angular range (avoid division by zero at 0 degrees)
    theta_min = 1.0e-3_dp   ! in radians
    theta_max = pi / 2.0_dp

    ! Use inverse transform sampling on Rutherford distribution
    prob = rand * (1.0_dp / tan(theta_min/2.0_dp)**2 - 1.0_dp / tan(theta_max/2.0_dp)**2)
    theta = 2.0_dp * atan(sqrt(1.0_dp / prob + 1.0_dp / tan(theta_max/2.0_dp)**2))

    scatter_angle = min(max(theta, theta_min), theta_max)
  end function elastic_scatter

  function inelastic_scatter(energy, mean_free_path) result(energy_loss)
    ! Simulates an inelastic scattering event resulting in energy loss
    real(dp), intent(in) :: energy          ! Initial beam energy in keV
    real(dp), intent(in) :: mean_free_path  ! Mean free path in nm
    real(dp) :: energy_loss                ! Energy lost during scattering in keV
    real(dp) :: rand

    call random_number(rand)

    ! Sample energy loss using exponential decay model
    energy_loss = -mean_free_path * log(1.0_dp - rand) * 0.01_dp  ! Loss scaled arbitrarily

    ! Limit energy loss to a fraction of the total energy
    if (energy_loss > 0.2_dp * energy) energy_loss = 0.2_dp * energy
  end function inelastic_scatter

end module scattering
