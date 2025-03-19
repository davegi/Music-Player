use egui_macroquad::Egui;
use macroquad::prelude::*;

pub struct SimpleVisual;

impl SimpleVisual {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&mut self) {
        // Create an Egui instance
        let mut egui = Egui::new();

        loop {
            // Clear the screen with a black background
            clear_background(BLACK);

            // Begin the egui context
            egui.run(|ctx| {
                // Render the UI using egui
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Simple Visual");
                    if ui.button("Play").clicked() {
                        println!("Play button clicked!");
                    }
                    if ui.button("Pause").clicked() {
                        println!("Pause button clicked!");
                    }
                });
            });

            // Draw a simple visualizer (e.g., a red circle in the center)
            draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, RED);

            // Draw the egui UI
            egui.draw();

            // Wait for the next frame
            next_frame().await;
        }
    }
}
