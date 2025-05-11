//! Visualization component for SEM simulation results.
//! Provides interactive display (zoom, pan, histogram) of the result image.

use eframe::egui::{self, ColorImage, TextureHandle, Ui, Vec2};
use egui::paint::callbacks::{PaintCallback, PaintCallbackFn};

/// State for interactive image visualization.
pub struct Visualizer {
    /// Current zoom factor (1.0 = 100%).
    pub zoom: f32,
    /// Current panning offset in pixels.
    pub offset: Vec2,
    /// Optional histogram data (0..255 counts).
    pub histogram: Option<[usize; 256]>,
}

impl Default for Visualizer {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: Vec2::ZERO,
            histogram: None,
        }
    }
}

impl Visualizer {
    /// Display the image texture with interactive controls.
    pub fn show(&mut self, ui: &mut Ui, texture: &TextureHandle) {
        // Controls: zoom slider and reset button
        ui.horizontal(|ui| {
            ui.label("Zoom:");
            ui.add(
                egui::DragValue::new(&mut self.zoom)
                    .clamp_range(0.1..=10.0)
                    .speed(0.1),
            );
            if ui.button("Reset").clicked() {
                self.zoom = 1.0;
                self.offset = Vec2::ZERO;
            }
        });

        // Interactive image area
        let available = ui.available_size();
        let image_size = texture.size_vec2() * self.zoom;
        let (response, painter) = ui.allocate_painter(image_size, egui::Sense::drag());

        // Handle panning
        if response.dragged() {
            self.offset += response.drag_delta();
        }

        // Calculate top-left position with offset
        let pos = response.rect.min + self.offset;
        painter.add(egui::PaintCallback {
            callback: std::sync::Arc::new(egui::PaintCallbackFn::new(move |_, painter| {
                painter.image(
                    texture.id(),
                    egui::Rect::from_min_size(pos, image_size),
                    egui::Rect::from_min_size(egui::Pos2::ZERO, texture.size_vec2()),
                    egui::Color32::WHITE,
                );
            })),
            rect: response.rect,
        });

        ui.separator();
        ui.collapsing("Histogram", |ui| {
            if self.histogram.is_none() {
                // Compute histogram lazily
                if let Some(data) = texture // cannot access raw; skip for now
                {
                    // Placeholder: histogram computation should be done pre-texture
                }
            }
            ui.label("Histogram view TBD");
        });
    }
}
