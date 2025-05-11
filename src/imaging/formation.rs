//! This module provides image formation from simulation data and export utilities.

use crate::imaging::Lut;

/// Map raw simulation data (floats) into an 8-bit grayscale image buffer.
///
/// # Arguments
/// - `data`: Flat slice of length `rows * cols` containing simulation values.
/// - `rows`: Number of rows in the image.
/// - `cols`: Number of columns in the image.
/// - `gamma`: Gamma correction factor (> 0). 1.0 means linear mapping.
/// - `lut`: Optional lookup table to remap final 0–255 values.
///
/// # Returns
/// A `Vec<u8>` of length `rows * cols`, row-major, representing grayscale intensities.
pub fn to_grayscale_bytes(
    data: &[f64],
    rows: usize,
    cols: usize,
    gamma: f64,
    lut: Option<&Lut>,
) -> Vec<u8> {
    assert!(gamma > 0.0, "Gamma must be greater than zero");
    let total = data.len();
    assert_eq!(total, rows * cols, "Data length does not match rows*cols");

    // Find data range
    let (min, max) = data.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(min, max), &v| (f64::min(min, v), f64::max(max, v)),
    );
    let range = max - min;

    // Convert each value
    let mut buf = Vec::with_capacity(total);
    for &val in data {
        // Normalize value to [0,1]
        let norm = if range > 0.0 { (val - min) / range } else { 0.0 };
        // Apply gamma correction
        let corrected = norm.powf(1.0 / gamma);
        // Scale to [0,255]
        let mut byte = (corrected * 255.0).round().clamp(0.0, 255.0) as u8;
        // Apply LUT if provided
        if let Some(table) = lut {
            byte = table[byte as usize];
        }
        buf.push(byte);
    }

    buf
}