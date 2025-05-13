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
    assert_eq!(data.len(), rows * cols, "Data length does not match rows*cols");

    const MAX_DIM: usize = 16384;

    // Find data range
    let (min, max) = data.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(min, max), &v| (f64::min(min, v), f64::max(max, v)),
    );
    let range = max - min;

    // Convert values to grayscale u8
    let mut grayscale = Vec::with_capacity(rows * cols);
    for &val in data {
        let norm = if range > 0.0 { (val - min) / range } else { 0.0 };
        let corrected = norm.powf(1.0 / gamma);
        let mut byte = (corrected * 255.0).round().clamp(0.0, 255.0) as u8;
        if let Some(table) = lut {
            byte = table[byte as usize];
        }
        grayscale.push(byte);
    }

    // Compute downscaling factors
    let scale_row = if rows > MAX_DIM {
        (rows as f64 / MAX_DIM as f64).ceil() as usize
    } else {
        1
    };
    let scale_col = if cols > MAX_DIM {
        (cols as f64 / MAX_DIM as f64).ceil() as usize
    } else {
        1
    };

    if scale_row == 1 && scale_col == 1 {
        return grayscale;
    }

    let new_rows = rows / scale_row;
    let new_cols = cols / scale_col;

    println!(
        "⚠️ Image resized from {}×{} to {}×{} to fit SDL limits",
        rows, cols, new_rows, new_cols
    );

    // Downsample (nearest-neighbor)
    let mut downscaled = Vec::with_capacity(new_rows * new_cols);
    for r in 0..new_rows {
        for c in 0..new_cols {
            let orig_r = r * scale_row;
            let orig_c = c * scale_col;
            let idx = orig_r * cols + orig_c;
            downscaled.push(grayscale[idx]);
        }
    }

    downscaled
}



// pub fn to_grayscale_bytes(
//     data: &[f64],
//     rows: usize,
//     cols: usize,
//     gamma: f64,
//     lut: Option<&Lut>,
// ) -> Vec<u8> {
//     assert!(gamma > 0.0, "Gamma must be greater than zero");
//     let total = data.len();
//     assert_eq!(total, rows * cols, "Data length does not match rows*cols");

//     // Find data range
//     let (min, max) = data.iter().fold(
//         (f64::INFINITY, f64::NEG_INFINITY),
//         |(min, max), &v| (f64::min(min, v), f64::max(max, v)),
//     );
//     let range = max - min;

//     // Convert each value
//     let mut buf = Vec::with_capacity(total);
//     for &val in data {
//         // Normalize value to [0,1]
//         let norm = if range > 0.0 { (val - min) / range } else { 0.0 };
//         // Apply gamma correction
//         let corrected = norm.powf(1.0 / gamma);
//         // Scale to [0,255]
//         let mut byte = (corrected * 255.0).round().clamp(0.0, 255.0) as u8;
//         // Apply LUT if provided
//         if let Some(table) = lut {
//             byte = table[byte as usize];
//         }
//         buf.push(byte);
//     }

//     buf
// }