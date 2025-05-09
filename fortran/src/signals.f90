module signals
  use iso_fortran_env, only: dp => real64
  implicit none
  private
  public :: generate_signal, apply_detector_response

  real(dp), parameter :: detector_efficiency = 0.85_dp
  real(dp), parameter :: noise_level = 0.02_dp ! standard deviation for Gaussian noise

contains

  function generate_signal(secondary_electrons, backscattered_electrons) result(signal_intensity)
    ! Generates signal intensity from SE and BSE counts
    real(dp), intent(in) :: secondary_electrons
    real(dp), intent(in) :: backscattered_electrons
    real(dp) :: signal_intensity

    ! Linear combination of SE and BSE weighted by empirical factors
    signal_intensity = 0.6_dp * secondary_electrons + 0.4_dp * backscattered_electrons
  end function generate_signal

  function apply_detector_response(signal_intensity) result(adjusted_signal)
    ! Simulates detector's response including efficiency and noise
    real(dp), intent(in) :: signal_intensity
    real(dp) :: adjusted_signal
    real(dp) :: noise, rand

    call random_number(rand)
    noise = noise_level * signal_intensity * (2.0_dp * rand - 1.0_dp) ! symmetric noise

    adjusted_signal = detector_efficiency * signal_intensity + noise

    ! Ensure non-negative output
    if (adjusted_signal < 0.0_dp) adjusted_signal = 0.0_dp
  end function apply_detector_response

end module signals
