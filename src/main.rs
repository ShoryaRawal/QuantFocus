mod ui {
    pub mod app;
    pub mod visualizer;
}

use ui::app::run_simulation;
use ui::visualizer::display_image;

fn main() {
    let (width, height, buffer) = run_simulation();
    if let Err(e) = display_image("SEM Simulation Output", width, height, &buffer) {
        eprintln!("Image display failed: {:?}", e);
    }
}


// mod ui;

// use ui::app::SemSimulatorApp;
// use eframe::NativeOptions;

// fn main() -> Result<(), eframe::Error> {
//     // Set up logging (if needed in the future)
//     #[cfg(debug_assertions)]
//     env_logger::init();

//     // Configure native options
//     let native_options = NativeOptions {
//         initial_window_size: Some(egui::vec2(1024.0, 768.0)),
//         drag_and_drop_support: true,
//         icon_data: None, // Set a window icon here if desired
//         ..Default::default()
//     };

//     // Launch the GUI application
//     eframe::run_native(
//         "SEM Simulator",
//         native_options,
//         Box::new(|_cc| Box::new(SemSimulatorApp::default())),
//     )
// }
