// src/ui/app.rs

// Pull in the library crate (your core simulator modules)
extern crate QuantFocus;

use QuantFocus::simulation::SimulationManager;
use QuantFocus::simulation::parameters::SimulationParameters;

/// Runs the SEM simulation with fixed parameters and returns
/// (width, height, grayscale buffer) ready for display.
pub fn run_simulation() -> (u32, u32, Vec<u8>) {
    // 1) Create and configure the simulation manager
    let mut sim = SimulationManager::new();
    sim.clear();

    // 2) Hard-coded simulation parameters:
    //    energy_kev = 15.0 keV
    //    thickness_nm = 100.0 nm
    //    angle_stddev_rad = 0.5 rad
    //    num_electrons = 50_000
    let params = SimulationParameters::new(
        15.0,    // beam energy (keV)
        100.0,   // sample thickness (nm)
        0.5,     // angular spread (rad)
        50_000,  // electrons per pixel
    )
    .expect("Valid default parameters");

    sim.enqueue(params);

    // 3) Run the simulation (blocking). Returns Vec<SimulationResult>.
    let mut results = sim.run_all();
    let result = results
        .pop()
        .expect("SimulationManager produced no results");

    // 4) Extract image data
    //    We assume `result.image_buffer` is a Vec<u8> of length width*height,
    //    already normalized to 0–255 grayscale.
    let width = result.width as u32;
    let height = result.height as u32;
    let buffer = result.image_buffer;

    (width, height, buffer)
}



// use eframe::{egui, epi};
// use egui::ColorImage;
// use crate::simulation::{SimulationManager, SimulationParameters};
// use crate::ui::visualizer::Visualizer;

// pub struct App {
//     texture: egui::TextureHandle,
// }

// impl App {
//     /// Create the UI app, run one hard‐coded simulation, upload image.
//     pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         // 1) Run simulation with fixed params
//         let mut sim = SimulationManager::new();
//         sim.clear();
//         sim.enqueue(
//             SimulationParameters::new(
//                 15.0,    // beam energy keV
//                 100.0,   // thickness nm
//                 0.5,     // angle stddev rad
//                 50_000,  // number of electrons
//             ).unwrap(),
//         );
//         let result = sim.run_all()
//             .into_iter()
//             .next()
//             .expect("Simulation produced no results");

//         // 2) Convert to egui::ColorImage
//         let size = [result.width as usize, result.height as usize];
//         let pixels: Vec<u8> = result.image_buffer
//             .iter()
//             .flat_map(|&b| [b, b, b, 255])
//             .collect();
//         let image = ColorImage::from_rgba_unmultiplied(size, &pixels);

//         // 3) Upload texture
//         let texture = cc.egui_ctx.load_texture(
//             "sem_result",
//             image,
//             egui::TextureOptions::default(),
//         );

//         Self { texture }
//     }
// }

// impl epi::App for App {
//     fn name(&self) -> &str {
//         "SEM Simulator"
//     }
//     fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Hard‐coded SEM Simulation Result");
//             ui.separator();
//             // Display the image
//             Visualizer::show(ui, &self.texture);
//         });
//         ctx.request_repaint();
//     }
// }
