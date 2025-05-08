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