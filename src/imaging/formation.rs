use crate::imaging::Lut;

/// Converts a floating-point simulation buffer into an 8-bit grayscale buffer,
/// downscaling if needed to fit SDL's texture limits.
pub fn to_grayscale_bytes(
    data: &[f64],
    height: usize,
    width: usize,
    gamma: f64,
    lut: Option<&Lut>,
) -> (Vec<u8>, usize, usize) {  // Return buffer and its dimensions
    assert!(gamma > 0.0, "Gamma must be greater than zero");
    assert_eq!(data.len(), width * height, "Data length does not match width*height");
    assert!(width > 0 && height > 0, "Dimensions must be greater than zero");

    // Constants for dimension limits
    const MAX_DIM: usize = 4096;  // Quarter of SDL's limit for safety
    const MIN_DIM: usize = 32;    // Minimum dimension to prevent degenerate cases

    println!("Converting image data with dimensions {}×{}", width, height);

    // Find data range for normalization
    let (min, max) = data.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(min, max), &v| (min.min(v), max.max(v)),
    );
    let range = max - min;

    // Convert to 8-bit grayscale with gamma correction
    let mut grayscale = vec![0u8; width * height];
    for (i, &value) in data.iter().enumerate() {
        let normalized = if range > 0.0 {
            ((value - min) / range).powf(1.0 / gamma)
        } else {
            0.0
        };
        grayscale[i] = (normalized * 255.0).round() as u8;
    }

    // Apply LUT if provided
    if let Some(lut) = lut {
        for pixel in grayscale.iter_mut() {
            *pixel = lut[*pixel as usize];
        }
    }

    // If dimensions are already within bounds, return as-is
    if width <= MAX_DIM && height <= MAX_DIM && 
       width >= MIN_DIM && height >= MIN_DIM {
        println!("Image dimensions within bounds: {}×{}", width, height);
        return (grayscale, width, height);
    }

    // 2) Calculate new dimensions that preserve aspect ratio
    let aspect_ratio = width as f64 / height as f64;
    
    let (new_width, new_height) = if aspect_ratio > 1.0 {
        // Wide image
        let new_width = MAX_DIM;
        let new_height = ((new_width as f64 / aspect_ratio).round() as usize).max(MIN_DIM);
        (new_width, new_height)
    } else {
        // Tall image
        let new_height = MAX_DIM;
        let new_width = ((new_height as f64 * aspect_ratio).round() as usize).max(MIN_DIM);
        (new_width, new_height)
    };

    println!(
        "⚠️ Downscaling image from {}×{} to {}×{} to fit SDL limits",
        width, height, new_width, new_height
    );

    // 3) Perform nearest-neighbor downsampling
    let mut downscaled = vec![0u8; new_width * new_height];
    let x_ratio = width as f64 / new_width as f64;
    let y_ratio = height as f64 / new_height as f64;

    for y in 0..new_height {
        let src_y = (y as f64 * y_ratio) as usize;
        for x in 0..new_width {
            let src_x = (x as f64 * x_ratio) as usize;
            let src_idx = src_y * width + src_x;  // Use width for source stride
            let dst_idx = y * new_width + x;           // Use new_width for destination stride
            
            if src_idx < grayscale.len() {
                downscaled[dst_idx] = grayscale[src_idx];
            }
        }
    }

    println!("Final downscaled dimensions: {}×{}", new_width, new_height);
    println!("Final buffer size: {}", downscaled.len());
    
    (downscaled, new_width, new_height)
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