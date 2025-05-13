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
    // SDL2 init
    let sdl = sdl2::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;
    let window = video
        .window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();

    // Build an RGB24 buffer (3 bytes per pixel)
    let mut rgb_buffer = Vec::with_capacity((width * height * 3) as usize);
    for &g in gray_buffer.iter() {
        rgb_buffer.push(g);  // R
        rgb_buffer.push(g);  // G
        rgb_buffer.push(g);  // B
    }

    // Create a streaming texture in RGB24 format
    let mut texture = creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
        .map_err(|e| e.to_string())?;

    // Upload our 3-byte buffer (pitch = width * 3)
    texture
        .update(None, &rgb_buffer, (width * 3) as usize)
        .map_err(|e| e.to_string())?;

    // Event loop: render and wait for quit
    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
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
        canvas.copy(&texture, None, Some(Rect::new(0, 0, width, height)))?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
