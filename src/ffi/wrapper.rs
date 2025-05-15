use std::ptr;
use std::slice;

use crate::ffi::bindings;

/// Represents a 2D scattering data result from the simulation.
pub struct ScatterData {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

/// Initializes the SEM simulation with the specified parameters.
///
/// # Arguments
/// - `energy` – Beam energy in keV
/// - `current` – Beam current in nA 
/// - `resolution` – Image resolution in pixels
/// - `distance` – Working distance in mm
pub fn init_simulation(energy: f64, current: f64, resolution: i32, distance: f64) {
    println!("Initializing simulation with {}keV beam energy, {}nA current, {}px resolution", 
             energy, current, resolution);
    unsafe {
        bindings::c_init_simulation(energy, current, resolution, distance);
    }
}

/// Runs the Monte Carlo SEM simulation.
///
/// This executes the Fortran backend's scattering and detection loop.
pub fn run_simulation() {
    println!("Starting Monte Carlo simulation");
    unsafe {
        bindings::c_run_simulation();
    }
}

/// Retrieves the simulation's scattering data as a flattened array and dimensions.
///
/// Returns a `ScatterData` struct containing the raw values and grid shape.
pub fn get_scatter_data() -> ScatterData {
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    let mut raw_ptr: *mut f64 = ptr::null_mut();

    unsafe {
        bindings::c_get_scatter_data(&mut raw_ptr, &mut rows, &mut cols);
        assert!(!raw_ptr.is_null(), "Null pointer returned from Fortran");
        println!("Received data from Fortran with dimensions: {}×{}", rows, cols);

        let total = (rows * cols) as usize;
        let data_slice = slice::from_raw_parts(raw_ptr, total);
        let data_vec = data_slice.to_vec();

        // Ensure dimensions are positive before converting to usize
        if rows <= 0 || cols <= 0 {
            panic!("Invalid dimensions from Fortran: {}×{}", rows, cols);
        }

        let result = ScatterData {
            data: data_vec,
            rows: rows as usize,
            cols: cols as usize,
        };
        println!("Converted to ScatterData with dimensions: {}×{}", result.rows, result.cols);
        result
    }
}

/// Gets the 2D SEM image data from the simulation.
pub fn get_image_data() -> (Vec<f64>, usize, usize) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut raw_ptr: *mut f64 = ptr::null_mut();

    unsafe {
        bindings::c_get_image_data(&mut raw_ptr, &mut width, &mut height);
        assert!(!raw_ptr.is_null(), "Null pointer returned from Fortran");
        println!("Received image data from Fortran with dimensions: {}×{}", width, height);

        let total = (width * height) as usize;
        let data_slice = slice::from_raw_parts(raw_ptr, total);
        let data_vec = data_slice.to_vec();

        // Ensure dimensions are positive
        if width <= 0 || height <= 0 {
            panic!("Invalid dimensions from Fortran: {}×{}", width, height);
        }

        (data_vec, width as usize, height as usize)
    }
}
