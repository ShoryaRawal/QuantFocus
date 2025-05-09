# SEM Simulator Project Template

## Project Overview

This project involves building a Scanning Electron Microscope (SEM) simulator that realistically generates noisy SEM images by modeling electron beam interactions with sample materials. The core simulation uses Monte Carlo methods and established physical principles to model:

- Electron beam generation
- Beam-material interaction (elastic and inelastic scattering)
- Signal detection (secondary and backscattered electrons)
- Image formation
- Noise modeling

## Architecture

The system uses a hybrid architecture:
- **Fortran**: Core simulation engine (Monte Carlo physics)
- **Rust**: Interface layer, orchestration, and UI
- **Optional Python**: Alternative visualization interface

## File Structure

```
sem-simulator/
├── CMakeLists.txt                  # Main build configuration
├── README.md                       # Project documentation
├── Cargo.toml                      # Rust package configuration
├── build.rs                        # Rust build script for FFI bindings
│
├── src/                            # Rust source code
│   ├── main.rs                     # Application entry point
│   ├── lib.rs                      # Rust library exports
│   ├── ffi/                        # Foreign Function Interface
│   │   ├── bindings.rs             # Auto-generated Fortran bindings
│   │   └── wrapper.rs              # Safe Rust wrappers around FFI
│   ├── simulation/                 # Simulation orchestration
│   │   ├── mod.rs                  # Module definition
│   │   ├── parameters.rs           # Simulation parameter handling
│   │   └── results.rs              # Results processing
│   ├── materials/                  # Material definitions
│   │   ├── mod.rs                  # Module definition
│   │   ├── presets.rs              # Predefined materials
│   │   └── custom.rs               # Custom material creation
│   ├── imaging/                    # Image processing
│   │   ├── mod.rs                  # Module definition
│   │   ├── formation.rs            # Image formation from signals
│   │   └── export.rs               # Image export utilities
│   └── ui/                         # User interface
│       ├── mod.rs                  # Module definition
│       ├── app.rs                  # Main application UI
│       └── visualizer.rs           # Results visualization
│
├── fortran/                        # Fortran source code
│   ├── CMakeLists.txt              # Fortran build configuration
│   ├── include/                    # Header files for C interface
│   │   └── sem_sim_c.h             # C header for Fortran bindings
│   ├── src/                        # Source files
│   │   ├── beam.f90                # Electron beam generation
│   │   ├── monte_carlo.f90         # Monte Carlo interaction engine
│   │   ├── scattering.f90          # Scattering models
│   │   ├── signals.f90             # Signal detection
│   │   ├── materials.f90           # Material properties
│   │   ├── noise.f90               # Noise modeling
│   │   └── c_interface.f90         # ISO_C_BINDING interface
│   └── tests/                      # Fortran unit tests
│       ├── test_beam.f90           # Beam generation tests
│       └── test_monte_carlo.f90    # Monte Carlo engine tests
│
├── python/                         # Optional Python interface
│   ├── requirements.txt            # Python dependencies
│   ├── setup.py                    # Python package setup
│   ├── sem_simulator/              # Python package
│   │   ├── __init__.py             # Package initialization
│   │   ├── interface.py            # Interface to Rust/Fortran
│   │   └── visualization.py        # Visualization tools
│   └── notebooks/                  # Jupyter notebooks
│       └── example_simulation.ipynb # Example usage
│
├── tests/                          # Integration tests
│   ├── integration_test.rs         # Rust integration tests
│   └── test_data/                  # Test data
│
├── examples/                       # Example simulations
│   ├── copper_sample.rs            # Example with copper
│   └── silicon_device.rs           # Example with silicon
│
└── docs/                           # Documentation
    ├── physics.md                  # Physics model documentation
    ├── api.md                      # API documentation
    └── user_guide.md               # User guide
```

## Key Files Already Implemented

### sem_sim_c.h

This is the C header file that defines the interface between the Fortran core and the Rust/C layers. It includes:

- Error codes and handling mechanisms
- Data structures for all simulation parameters
- Function prototypes for the entire API
- Comprehensive documentation for each component

This header defines several important structures:
- `sem_material_t`: Material properties (atomic number, density, etc.)
- `sem_beam_params_t`: Electron beam parameters
- `sem_scan_params_t`: Image scan settings 
- `sem_mc_params_t`: Monte Carlo simulation settings
- `sem_detector_params_t`: Detector configuration
- `sem_image_params_t`: Image processing parameters
- `sem_sim_ctx_t`: Simulation context handle
- `sem_sample_t`: Sample definition
- `sem_image_t`: Image data structure
- `sem_sim_results_t`: Comprehensive simulation results

And critical functions like:
- Context creation/destruction
- Sample creation/manipulation
- Monte Carlo simulation execution
- Image generation and export

## Priority Implementation Order

The implementation should proceed in this order:

1. **monte_carlo.f90**: Core physics engine for electron interactions
2. **materials.f90**: Material property definitions and handling
3. **beam.f90**: Electron beam generation models
4. **scattering.f90**: Elastic and inelastic scattering implementation
5. **signals.f90**: Signal detection based on scattering
6. **c_interface.f90**: Fortran-to-C bindings that implement the sem_sim_c.h interface

## Fortran Module Details

The Fortran modules need detailed implementations. Here's what each should contain:

### monte_carlo.f90

The core physics simulation that tracks electrons through materials:

```fortran
module monte_carlo
  use iso_fortran_env
  implicit none
  
  ! Private module variables and types
  private
  
  ! Public procedures and types
  public :: initialize_monte_carlo
  public :: simulate_electron_trajectory
  public :: mc_run_pixel_simulation
  
  ! Constants for physics calculations
  real(real64), parameter :: ELECTRON_MASS = 9.1093837015e-31 ! kg
  real(real64), parameter :: ELEMENTARY_CHARGE = 1.602176634e-19 ! C
  real(real64), parameter :: VACUUM_PERMITTIVITY = 8.8541878128e-12 ! F/m
  
  ! Types for simulation data
  type :: electron_state
    real(real64) :: x, y, z    ! Position (nm)
    real(real64) :: dx, dy, dz ! Direction (unit vector)
    real(real64) :: energy     ! Energy (eV)
    integer :: collisions      ! Collision counter
    logical :: terminated      ! Electron terminated flag
  end type electron_state
  
  ! Additional needed type definitions...
  
contains
  ! Initialize Monte Carlo engine with parameters
  subroutine initialize_monte_carlo(num_electrons, max_collisions, min_energy, max_depth, track_secondaries)
    integer, intent(in) :: num_electrons, max_collisions
    real(real64), intent(in) :: min_energy, max_depth
    logical, intent(in) :: track_secondaries
    
    ! Implementation...
  end subroutine initialize_monte_carlo
  
  ! Simulate a single electron trajectory
  subroutine simulate_electron_trajectory(material_data, initial_energy, &
                                       x0, y0, z0, dx0, dy0, dz0, &
                                       bse_detected, se_count)
    ! Arguments...
    
    ! Implementation of electron path through material with scattering events
    ! This is the core Monte Carlo algorithm
  end subroutine simulate_electron_trajectory
  
  ! Run simulation for a single pixel
  function mc_run_pixel_simulation(material_data, beam_params, detector_params, x_pos, y_pos) result(pixel_value)
    ! Arguments...
    
    ! Implementation that calls simulate_electron_trajectory for multiple electrons
    ! and aggregates the results into a pixel value
  end function mc_run_pixel_simulation
  
  ! Private helper functions
  
  ! Calculate elastic scattering cross-section (Rutherford or Mott)
  function elastic_cross_section(atomic_number, energy) result(cross_section)
    ! Implementation...
  end function elastic_cross_section
  
  ! Calculate inelastic mean free path
  function inelastic_mean_free_path(material_data, energy) result(mfp)
    ! Implementation...
  end function inelastic_mean_free_path
  
  ! Calculate energy loss per unit path (Bethe formula)
  function energy_loss_rate(material_data, energy) result(loss_rate)
    ! Implementation...
  end function energy_loss_rate
  
  ! Sample scattering angle from distribution
  function sample_scattering_angle(energy, atomic_number) result(theta)
    ! Implementation...
  end function sample_scattering_angle
  
  ! Generate secondary electrons
  subroutine generate_secondaries(state, material_data, energy_loss, secondaries)
    ! Implementation...
  end subroutine generate_secondaries
  
end module monte_carlo
```

### materials.f90

Defines material properties and handling:

```fortran
module materials
  use iso_fortran_env
  implicit none
  
  private
  
  public :: material_type
  public :: create_material
  public :: create_layered_sample
  public :: get_material_at_position
  public :: get_predefined_material
  
  ! Material data type
  type :: material_type
    real(real64) :: atomic_number    ! Z
    real(real64) :: density          ! g/cm³
    real(real64) :: work_function    ! eV
    real(real64) :: mean_ionization  ! eV
    character(len=32) :: name        ! Material name
  end type material_type
  
  ! Sample definitions
  type :: sample_layer_type
    type(material_type) :: material
    real(real64) :: thickness        ! nm
    real(real64) :: z_start          ! nm
    real(real64) :: z_end            ! nm
  end type sample_layer_type
  
  type :: sample_type
    integer :: num_layers
    type(sample_layer_type), allocatable :: layers(:)
    real(real64) :: width            ! nm
    real(real64) :: height           ! nm
    real(real64) :: depth            ! nm
  end type sample_type
  
contains
  ! Create a new material
  function create_material(atomic_number, density, work_function, mean_ionization, name) result(material)
    ! Implementation...
  end function create_material
  
  ! Create a layered sample
  function create_layered_sample(materials, thicknesses, num_layers, width, height) result(sample)
    ! Implementation...
  end function create_layered_sample
  
  ! Get material at a specific position within sample
  function get_material_at_position(sample, x, y, z) result(material)
    ! Implementation...
  end function get_material_at_position
  
  ! Get predefined material by name
  function get_predefined_material(name) result(material)
    character(len=*), intent(in) :: name
    type(material_type) :: material
    
    ! Implementation with common materials like Cu, Si, C, etc.
  end function get_predefined_material
  
end module materials
```

### beam.f90

Models the electron beam generation:

```fortran
module beam
  use iso_fortran_env
  implicit none
  
  private
  
  public :: beam_type
  public :: create_beam
  public :: generate_electron
  
  ! Beam parameter type
  type :: beam_type
    real(real64) :: energy_kev         ! Beam energy in keV
    real(real64) :: current_na         ! Beam current in nA
    real(real64) :: working_distance   ! Working distance in mm
    real(real64) :: spot_size          ! Spot size in nm
    real(real64) :: convergence_angle  ! Beam convergence angle in radians
  end type beam_type
  
contains
  ! Create a new beam configuration
  function create_beam(energy_kev, current_na, working_distance, spot_size, convergence_angle) result(beam_config)
    ! Implementation...
  end function create_beam
  
  ! Generate a single electron from the beam at a specific position
  subroutine generate_electron(beam_config, x_pos, y_pos, x, y, z, dx, dy, dz, energy)
    ! Implementation that generates electron position and direction
    ! based on Gaussian beam profile
  end subroutine generate_electron
  
  ! Calculate electron density distribution (Gaussian)
  function electron_density(beam_config, r) result(density)
    ! Implementation of Gaussian beam profile
  end function electron_density
  
end module beam
```

### scattering.f90

Implements scattering models:

```fortran
module scattering
  use iso_fortran_env
  use materials, only: material_type
  implicit none
  
  private
  
  public :: elastic_scattering
  public :: inelastic_scattering
  public :: calculate_mean_free_path
  
contains
  ! Simulate elastic scattering event
  subroutine elastic_scattering(material, energy, dx, dy, dz, new_dx, new_dy, new_dz, energy_loss)
    ! Implementation of Rutherford or Mott scattering
  end subroutine elastic_scattering
  
  ! Simulate inelastic scattering event
  subroutine inelastic_scattering(material, energy, dx, dy, dz, new_dx, new_dy, new_dz, energy_loss, secondary_count)
    ! Implementation of inelastic collision with energy loss
  end subroutine inelastic_scattering
  
  ! Calculate mean free path for combined scattering processes
  function calculate_mean_free_path(material, energy) result(mfp)
    ! Implementation...
  end function calculate_mean_free_path
  
  ! Screened Rutherford cross-section
  function rutherford_cross_section(atomic_number, energy, angle) result(cross_section)
    ! Implementation...
  end function rutherford_cross_section
  
  ! Energy loss calculation (Bethe equation)
  function bethe_energy_loss(material, energy, path_length) result(energy_loss)
    ! Implementation...
  end function bethe_energy_loss
  
end module scattering
```

### signals.f90

Handles signal detection and processing:

```fortran
module signals
  use iso_fortran_env
  implicit none
  
  private
  
  public :: detect_signal
  public :: apply_noise
  
contains
  ! Detect signal from electron interactions
  function detect_signal(signal_type, collection_efficiency, energy_threshold, &
                        take_off_angle, azimuthal_angle, &
                        electron_energy, scattering_angle, exit_angle) result(detected)
    ! Implementation...
  end function detect_signal
  
  ! Apply noise model to signal
  function apply_noise(signal_value, noise_model, param1, param2) result(noisy_signal)
    ! Implementation of Poisson and/or Gaussian noise
  end function apply_noise
  
  ! Private helper functions
  
  ! Calculate detection probability based on geometry
  function detection_probability(take_off_angle, azimuthal_angle, exit_angle) result(prob)
    ! Implementation...
  end function detection_probability
  
  ! Generate Poisson noise
  function poisson_noise(mean_value) result(noisy_value)
    ! Implementation...
  end function poisson_noise
  
  ! Generate Gaussian noise
  function gaussian_noise(mean_value, std_dev) result(noisy_value)
    ! Implementation...
  end function gaussian_noise
  
end module signals
```

### c_interface.f90

Implements the C interface defined in sem_sim_c.h:

```fortran
module c_interface
  use iso_fortran_env
  use iso_c_binding
  use monte_carlo
  use materials
  use beam
  use scattering
  use signals
  implicit none
  
  private
  
  ! Public C-bound procedures from sem_sim_c.h
  public :: sem_initialize
  public :: sem_finalize
  public :: sem_get_version
  public :: sem_create_context
  public :: sem_destroy_context
  public :: sem_create_homogeneous_sample
  public :: sem_create_layered_sample
  public :: sem_destroy_sample
  public :: sem_get_predefined_material
  public :: sem_run_simulation
  public :: sem_cleanup_results
  public :: sem_save_image
  public :: sem_get_error_message
  public :: sem_get_default_params
  public :: sem_init_monte_carlo
  public :: sem_simulate_electron
  public :: sem_set_progress_callback
  
  ! Type definitions matching the C structures
  ! These must match exactly with sem_sim_c.h
  
  ! Internal data structures
  type :: sim_context_type
    integer :: id
    ! Internal simulation state
  end type sim_context_type
  
  ! Global state
  integer :: next_context_id = 1
  type(sim_context_type), allocatable, target :: contexts(:)
  
contains
  ! Implementation of all functions from sem_sim_c.h
  ! Each function must match the C prototype exactly
  
  function sem_initialize() bind(C, name="sem_initialize")
    integer(c_int) :: sem_initialize
    
    ! Implementation...
  end function sem_initialize
  
  function sem_finalize() bind(C, name="sem_finalize")
    integer(c_int) :: sem_finalize
    
    ! Implementation...
  end function sem_finalize
  
  ! And so on for all other functions in sem_sim_c.h...
  
  ! Helper functions for converting between Fortran and C types
  
  ! Convert C material to Fortran material
  function c_to_fortran_material(c_material) result(f_material)
    type(material_type) :: f_material
    ! Implementation...
  end function c_to_fortran_material
  
  ! Convert Fortran material to C material
  subroutine fortran_to_c_material(f_material, c_material)
    ! Implementation...
  end subroutine fortran_to_c_material
  
  ! Additional conversion functions for other types...
  
end module c_interface
```

## Implementation Notes

When implementing the Fortran modules:

1. **Start with core physics**: Focus on the Monte Carlo simulation engine first
2. **Ensure numerical stability**: Use appropriate precision for all calculations
3. **Memory management**: Be careful with allocation/deallocation of resources
4. **Thread safety**: Consider thread-safety for parallel execution
5. **Performance optimization**: Optimize inner loops and critical paths
6. **Validation**: Compare results with analytical models for simple cases

## C Interface Implementation Details

When implementing the Fortran-to-C interface:
1. Use ISO_C_BINDING consistently
2. Ensure proper memory handling across language boundaries
3. Implement clear error codes and status reporting
4. Create conversion functions for all complex types
5. Document all assumptions and limitations

The C interface (`sem_sim_c.h`) is already fully defined and provides the complete API. The Fortran implementation must fulfill all the promises made in this interface.

## Next Steps

1. Implement `monte_carlo.f90` as the core physics engine
2. Implement supporting modules (`materials.f90`, `beam.f90`, etc.)
3. Implement `c_interface.f90` to connect Fortran with C/Rust
4. Create the Rust FFI bindings
5. Build a minimal end-to-end pipeline

This project requires deep understanding of both electron microscopy physics and software engineering principles. Focus on correctness before optimization, but be aware that the Monte Carlo simulation will be computationally intensive.