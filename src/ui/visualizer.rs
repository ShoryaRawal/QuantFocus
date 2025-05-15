// src/ui/visualizer.rs
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

/// Displays an 8-bit grayscale buffer by converting it to RGB24 and streaming it.
pub fn display_image(
    title: &str,
    width: u32,
    height: u32,
    gray_buffer: &Vec<u8>,
) -> Result<(), String> {
    // Debug information
    println!("Input dimensions: {}×{}", width, height);
    if width == 0 || height == 0 {
        return Err("Invalid dimensions: width or height is zero".into());
    }
    if gray_buffer.len() != (width as usize * height as usize) {
        return Err(format!(
            "Buffer size mismatch: expected {}×{}={}, got {}",
            width, height, width as usize * height as usize, gray_buffer.len()
        ));
    }

    // Ensure minimum dimensions for display
    const MIN_DISPLAY_WIDTH: u32 = 320;  // Minimum width for reasonable display
    let display_width = width.max(MIN_DISPLAY_WIDTH);
    let display_height = if width < MIN_DISPLAY_WIDTH {
        // Scale height proportionally if we're increasing width
        (height as f64 * (display_width as f64 / width as f64)) as u32
    } else {
        height
    };

    println!("Display dimensions: {}×{}", display_width, display_height);

    // SDL2 init
    let sdl = sdl2::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;
    let window = video
        .window(title, display_width, display_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();

    // Create a texture in RGBA32 format (most widely supported)
    println!("Creating texture with dimensions: {}×{}", width, height);
    let texture_result = creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, width, height);

    let mut texture = match texture_result {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to create texture: {}", e);
            println!("Attempting with reduced dimensions...");
            // Try creating texture with halved dimensions
            let new_width = (width / 2).max(1);
            let new_height = (height / 2).max(1);
            println!("Retrying with dimensions: {}×{}", new_width, new_height);
            creator
                .create_texture_streaming(PixelFormatEnum::RGBA32, new_width, new_height)
                .map_err(|e| e.to_string())?
        }
    };

    // Convert grayscale to RGBA (replicating the value across R,G,B channels, alpha=255)
    let rgba_buffer: Vec<u8> = gray_buffer.iter()
        .flat_map(|&g| [g, g, g, 255])
        .collect();

    // Upload our RGBA buffer (pitch = width * 4 bytes)
    println!("Updating texture with buffer size: {}", rgba_buffer.len());
    texture
        .update(None, &rgba_buffer, width as usize * 4)
        .map_err(|e| e.to_string())?;

    // Calculate display rectangle to maintain aspect ratio
    let display_rect = {
        let src_aspect = width as f64 / height as f64;
        let dst_aspect = display_width as f64 / display_height as f64;
        
        if src_aspect > dst_aspect {
            // Fit to width
            let h = ((display_width as f64 / src_aspect) as i32).min(display_height as i32);
            let y = ((display_height as i32 - h) / 2).max(0);
            Rect::new(0, y, display_width, h as u32)
        } else {
            // Fit to height
            let w = ((display_height as f64 * src_aspect) as i32).min(display_width as i32);
            let x = ((display_width as i32 - w) / 2).max(0);
            Rect::new(x, 0, w as u32, display_height)
        }
    };

    // Event loop: render and wait for quit
    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    'running: loop {
        for ev in events.poll_iter() {
            match ev {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        canvas.clear();
        canvas.copy(&texture, None, Some(display_rect))?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
