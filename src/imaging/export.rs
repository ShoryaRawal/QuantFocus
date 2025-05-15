//! Image export utilities for SEM simulator.

use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;

use image::{ImageBuffer, ImageError, Luma};
use png::{Encoder, ColorType, BitDepth};

use crate::simulation::parameters::SimulationParameters;

/// Save a raw 8-bit grayscale buffer as a PNG file at the given path.
///
/// # Arguments
/// - `path`: File path to save the PNG.
/// - `buffer`: Flat slice of length `width * height` of grayscale values.
/// - `width`: Image width in pixels.
/// - `height`: Image height in pixels.
///
/// # Errors
/// Returns `ImageError` if the underlying image crate fails.
pub fn save_png(
    path: &str,
    buffer: &[u8],
    width: u32,
    height: u32,
) -> Result<(), ImageError> {
    // Create an ImageBuffer from the raw buffer
    let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buffer.to_vec())
        .expect("Buffer length does not match width*height");

    // Save using the image crate
    img.save(path)
}

/// Save a raw 8-bit grayscale buffer as a PNG file with embedded metadata tEXt chunks.
///
/// Embeds simulation parameters (energy, thickness, angle_stddev, num_electrons) as text.
///
/// # Arguments
/// - `path`: File path to save the PNG.
/// - `buffer`: Flat slice of length `width * height` of grayscale values.
/// - `width`: Image width in pixels.
/// - `height`: Image height in pixels.
/// - `params`: Simulation parameters to embed in metadata.
///
/// # Errors
/// Returns `std::io::Error` if writing fails.
pub fn save_png_with_metadata(
    path: &str,
    buffer: &[u8],
    width: u32,
    height: u32,
    params: &SimulationParameters,
) -> Result<(), io::Error> {
    // Prepare file and encoder
    let file = File::create(path)?;
    let w = BufWriter::new(file);
    let mut encoder = Encoder::new(w, width, height);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::Eight);

    // Add tEXt chunks for metadata
    encoder.add_text_chunk("Energy_keV".into(), params.energy_kev.to_string())?;
    encoder.add_text_chunk("Current_nA".into(), params.current_na.to_string())?;
    encoder.add_text_chunk("Resolution".into(), params.resolution.to_string())?;
    encoder.add_text_chunk("Distance_mm".into(), params.distance_mm.to_string())?;

    let mut writer = encoder.write_header()?;

    // Write image data (row-major)
    writer.write_image_data(buffer)?;

    Ok(())
}
