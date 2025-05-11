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
/// - `thickness` – Sample thickness in micrometers
/// - `angle_stddev` – Standard deviation of beam angle (radians)
/// - `n_electrons` – Number of electrons to simulate
pub fn init_simulation(energy: f64, thickness: f64, angle_stddev: f64, n_electrons: i32) {
    unsafe {
        bindings::c_init_simulation(energy, thickness, angle_stddev, n_electrons);
    }
}

/// Runs the Monte Carlo SEM simulation.
///
/// This executes the Fortran backend's scattering and detection loop.
pub fn run_simulation() {
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

        let total = (rows * cols) as usize;
        let data_slice = slice::from_raw_parts(raw_ptr, total);
        let data_vec = data_slice.to_vec();

        ScatterData {
            data: data_vec,
            rows: rows as usize,
            cols: cols as usize,
        }
    }
}
