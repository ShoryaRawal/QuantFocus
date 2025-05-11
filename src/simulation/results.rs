//! Defines the result of a SEM simulation, including raw scatter data and derived image.

use crate::simulation::parameters::SimulationParameters;
use crate::imaging::formation;
use crate::imaging::export;
use crate::ffi::wrapper::ScatterData;

/// A complete simulation result, tying parameters to output data and images.
pub struct SimulationResult {
    pub params: SimulationParameters,
    pub scatter: ScatterData,
    pub image_buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl SimulationResult {
    /// Converts raw scatter data into a SimulationResult, applying gamma and LUT.
    pub fn from_scatter(scatter: ScatterData, params: &SimulationParameters) -> Self {
        let rows = scatter.rows;
        let cols = scatter.cols;
        
        // Apply image formation (normalize to [0,255], gamma=1.0 by default)
        let image_buffer = formation::to_grayscale_bytes(
            &scatter.data,
            rows,
            cols,
            /* gamma */ 1.0,
            /* lut */ None,
        );

        // export to disk or embed metadata
        // let filename = format!("sim_{:.1}keV_{}e.png", params.energy_kev, params.num_electrons);
        // export::save_png(&filename, &image_buffer, cols as u32, rows as u32)
        
        SimulationResult {
            params: params.clone(),
            scatter,
            image_buffer,
            width: cols,
            height: rows,
        }
    }

    /// Save the result image to a PNG file with embedded metadata.
    pub fn save_png(&self, path: &str) -> Result<(), image::ImageError> {
        export::save_png_with_metadata(
            path,
            &self.image_buffer,
            self.width as u32,
            self.height as u32,
            &self.params,
        )
        .map_err(image::ImageError::IoError)
    }
    
}
