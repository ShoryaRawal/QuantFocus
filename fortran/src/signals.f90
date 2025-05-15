module signals
  use iso_fortran_env, only: dp => real64
  implicit none
  private
  public :: generate_signal, apply_detector_response, setup_detector

  ! Physical constants
  real(dp), parameter :: PI = 3.141592653589793_dp

  ! Detector parameters
  real(dp), parameter :: SE_EFFICIENCY = 0.85_dp     ! Secondary electron detection efficiency
  real(dp), parameter :: BSE_EFFICIENCY = 0.90_dp    ! Backscattered electron detection efficiency
  real(dp), parameter :: NOISE_LEVEL = 0.02_dp       ! Base noise level (standard deviation)
  real(dp), parameter :: DARK_CURRENT = 0.005_dp     ! Dark current contribution
  real(dp), parameter :: GAIN = 1000.0_dp            ! Detector gain
  real(dp), parameter :: TIME_CONSTANT = 100.0e-9_dp ! Detector response time (s)

  ! Detector geometry
  real(dp) :: detector_distance = 10.0_dp  ! mm
  real(dp) :: detector_angle = 45.0_dp     ! degrees
  real(dp) :: detector_solid_angle         ! steradians

  ! Image formation parameters
  real(dp), parameter :: EDGE_ENHANCEMENT = 1.2_dp   ! Edge brightness enhancement factor
  real(dp), parameter :: DEPTH_SENSITIVITY = 0.8_dp  ! Sensitivity to topographical features
  real(dp), parameter :: CRYSTAL_CONTRAST = 0.15_dp  ! Contrast between crystal orientations

contains

  subroutine setup_detector(distance_mm, angle_deg)
    ! Configure detector geometry
    real(dp), intent(in) :: distance_mm, angle_deg
    
    detector_distance = distance_mm
    detector_angle = angle_deg
    
    ! Calculate detector solid angle (simplified circular detector)
    detector_solid_angle = 2.0_dp * PI * (1.0_dp - cos(0.1_dp))  ! Assumes 0.1 rad acceptance
  end subroutine setup_detector

  function generate_signal(position, energy, surface_normal, crystal_orientation) result(signal_intensity)
    ! Generates signal intensity from SE and BSE with energy and angular dependence
    real(dp), intent(in) :: position(3)           ! (x,y,z) coordinates
    real(dp), intent(in) :: energy                ! Electron energy in keV
    real(dp), intent(in) :: surface_normal(3)     ! Local surface normal vector
    real(dp), intent(in) :: crystal_orientation   ! Local crystal orientation
    real(dp) :: signal_intensity

    real(dp) :: topo_factor, edge_factor, crystal_factor
    real(dp) :: detector_vector(3), cos_angle
    real(dp) :: local_height, gradient(2)
    
    ! Calculate topographical contrast
    detector_vector = [sin(detector_angle), 0.0_dp, cos(detector_angle)]
    cos_angle = dot_product(surface_normal, detector_vector)
    topo_factor = (1.0_dp + DEPTH_SENSITIVITY * abs(cos_angle))

    ! Calculate edge enhancement
    local_height = position(3)
    gradient = [position(1), position(2)] - [0.0_dp, 0.0_dp]  ! Simplified gradient
    edge_factor = 1.0_dp + EDGE_ENHANCEMENT * exp(-norm2(gradient)**2 / 100.0_dp)

    ! Calculate crystal orientation contrast
    crystal_factor = 1.0_dp + CRYSTAL_CONTRAST * cos(crystal_orientation)

    ! Combine all contrast mechanisms
    signal_intensity = SE_EFFICIENCY * topo_factor * edge_factor * crystal_factor * &
                      exp(-norm2(position) / detector_distance) * &
                      (1.0_dp - exp(-energy/2.0_dp))  ! Energy-dependent yield
  end function generate_signal

  function apply_detector_response(signal_intensity, dwell_time) result(measured_signal)
    ! Simulates detector's response including various real-world effects
    real(dp), intent(in) :: signal_intensity
    real(dp), intent(in) :: dwell_time        ! Pixel dwell time in seconds
    real(dp) :: measured_signal
    real(dp) :: noise, shot_noise, thermal_noise
    real(dp) :: rand1, rand2, response_factor
    
    ! Time-dependent response
    response_factor = 1.0_dp - exp(-dwell_time/TIME_CONSTANT)
    
    ! Generate realistic noise
    call random_number(rand1)
    call random_number(rand2)
    
    thermal_noise = NOISE_LEVEL * sqrt(-2.0_dp * log(rand1)) * cos(2.0_dp * PI * rand2)
    shot_noise = sqrt(abs(signal_intensity)) * NOISE_LEVEL * &
                 sqrt(-2.0_dp * log(rand1)) * sin(2.0_dp * PI * rand2)
    
    ! Combine signal components
    measured_signal = response_factor * signal_intensity + &
                     shot_noise + thermal_noise + DARK_CURRENT
    
    ! Apply realistic dynamic range
    measured_signal = max(0.0_dp, min(measured_signal, 255.0_dp))
  end function apply_detector_response

  pure function dot_product(a, b) result(result)
    real(dp), dimension(3), intent(in) :: a, b
    real(dp) :: result
    result = a(1)*b(1) + a(2)*b(2) + a(3)*b(3)
  end function dot_product

end module signals
