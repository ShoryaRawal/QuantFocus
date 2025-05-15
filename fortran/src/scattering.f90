! scattering.f90
! Implements electron scattering models in the SEM simulation

module scattering
  use iso_fortran_env, only: dp => real64
  implicit none
  private
  public :: elastic_scatter, inelastic_scatter, generate_secondaries

  ! Physical constants
  real(dp), parameter :: PI = 3.141592653589793_dp
  real(dp), parameter :: ELECTRON_MASS = 9.10938356e-31_dp ! kg
  real(dp), parameter :: ELECTRON_CHARGE = 1.60217662e-19_dp ! C
  real(dp), parameter :: BOHR_RADIUS = 5.29177210903e-11_dp ! m
  real(dp), parameter :: RYDBERG_ENERGY = 13.605693122994_dp ! eV
  real(dp), parameter :: FINE_STRUCTURE = 1.0_dp/137.035999084_dp
  real(dp), parameter :: SPEED_OF_LIGHT = 2.99792458e8_dp ! m/s
  integer, parameter :: MAX_SE = 10 ! Maximum number of secondary electrons

contains

  function elastic_scatter(energy_in, atomic_number) result(scatter_angle)
    ! Simulates an elastic scattering angle using Mott cross-section approximation
    real(dp), intent(in) :: energy_in
    integer, intent(in) :: atomic_number
    real(dp) :: scatter_angle
    real(dp) :: energy, beta2, gamma, screening_param, eta, rand
    
    ! Work with local copy of energy
    energy = energy_in * 1000.0_dp  ! Convert to eV
    
    ! Relativistic factors
    beta2 = 1.0_dp - 1.0_dp/(1.0_dp + energy/(ELECTRON_MASS * SPEED_OF_LIGHT**2))**2
    gamma = 1.0_dp/sqrt(1.0_dp - beta2)
    
    ! Screening parameter (TF model)
    screening_param = 0.885_dp * BOHR_RADIUS * atomic_number**(-1.0_dp/3.0_dp)
    eta = 2.0_dp * energy * screening_param / (FINE_STRUCTURE * BOHR_RADIUS)
    
    ! Sample from screened Rutherford distribution
    call random_number(rand)
    scatter_angle = acos(1.0_dp - 2.0_dp * rand/(1.0_dp + eta * (1.0_dp - rand)))
  end function elastic_scatter

  function inelastic_scatter(energy_in, atomic_number) result(energy_loss)
    ! Simulates inelastic scattering using Bethe formula with straggling
    real(dp), intent(in) :: energy_in
    integer, intent(in) :: atomic_number
    real(dp) :: energy_loss
    real(dp) :: energy, mean_loss, straggling, rand
    
    ! Work with local copy of energy
    energy = energy_in * 1000.0_dp  ! Convert to eV
    
    ! Mean energy loss (simplified Bethe formula)
    mean_loss = (78500.0_dp * atomic_number) / energy * log(1.166_dp * energy/RYDBERG_ENERGY)
    
    ! Add energy straggling (simplified model)
    call random_number(rand)
    straggling = mean_loss * 0.1_dp * (2.0_dp * rand - 1.0_dp)
    energy_loss = mean_loss + straggling
    
    ! Convert back to keV
    energy_loss = energy_loss * 0.001_dp
  end function inelastic_scatter

  function generate_secondaries(primary_energy) result(num_secondaries)
    ! Generates secondary electrons based on primary electron energy
    real(dp), intent(in) :: primary_energy
    integer :: num_secondaries
    real(dp) :: rand, yield
    
    ! Calculate SE yield using empirical formula
    yield = 1.36_dp * (primary_energy/1.5_dp) * exp(-2.3_dp * sqrt(primary_energy/1.5_dp))
    
    ! Sample number of secondaries
    call random_number(rand)
    num_secondaries = min(int(rand * yield), MAX_SE)
  end function generate_secondaries

end module scattering
