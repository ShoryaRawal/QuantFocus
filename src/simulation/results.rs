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
        // Get the 2D image data from Fortran
        let (image_data, width, height) = crate::ffi::wrapper::get_image_data();
        
        println!("Raw image data dimensions: {}×{}", width, height);
        
        // Apply image formation (normalize to [0,255], gamma=1.0 by default)
        let image_buffer = formation::to_grayscale_bytes(
            &image_data,
            height,
            width,
            /* gamma */ 1.0,
            /* lut */ None,
        ).0;  // Only take the buffer, dimensions are already known

        println!("Final image dimensions: {}×{}", width, height);
        
        SimulationResult {
            params: params.clone(),
            scatter,
            image_buffer,
            width,
            height,
        }
    }

    /// Save the result image to a PNG file with embedded metadata.
    pub fn save_png(&self, path: &str) -> Result<(), image::ImageError> {
        println!("Saving PNG with dimensions: {}×{}", self.width, self.height);
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
