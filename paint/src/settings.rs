use egui_macroquad::egui::{
    color_picker::color_edit_button_srgba, color_picker::Alpha, Color32, Slider, Window,
};
use macroquad::prelude::*;

/// Represents the settings for the painting application, including brush size, color, and clear state.
pub struct Settings {
    pub brush_size: f32,
    pub brush_color: Color32,
    pub is_clear: bool,
}

impl Settings {
    /// Creates a new `Settings` instance with default values.
    pub fn new() -> Self {
        Self {
            brush_size: 10.0,
            brush_color: Color32::BLACK,
            is_clear: false,
        }
    }

    /// Updates the settings UI and handles user interactions.
    pub fn update(&mut self) {
        egui_macroquad::ui(|ctx| {
            Window::new("Settings")
                .fixed_pos([10.0, 10.0]) // Position the settings window at (10,10)
                .show(ctx, |ui| {
                    ui.label("Brush Settings");

                    // Brush size slider
                    ui.horizontal(|ui| {
                        ui.label("Size:");
                        ui.add(Slider::new(&mut self.brush_size, 1.0..=50.0).text("px"));
                    });

                    // Color picker with alpha blending support
                    if color_edit_button_srgba(ui, &mut self.brush_color, Alpha::OnlyBlend)
                        .changed()
                    {
                        // Color updates automatically when changed
                    }

                    // Additional controls
                    ui.horizontal(|ui| {
                        // Clear button
                        if ui.button("Clear").clicked() {
                            self.is_clear = true;
                        } else {
                            self.is_clear = false;
                        }

                        // Eraser mode: sets brush to white and increases size
                        if ui.button("Eraser").clicked() {
                            self.brush_color = Color32::WHITE;
                            self.brush_size = 50.0;
                        }
                    });
                });
        });
    }
}
