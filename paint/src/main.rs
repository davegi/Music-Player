mod settings;
mod brush;
mod screen;
mod controller;

use macroquad::prelude::*;
use controller::Controller;

/// Configures the window size before the application starts.
fn window_conf() -> Conf {
    Conf {
        window_title: "Paint".to_owned(),
        window_width: 1280, // Increased width
        window_height: 720, // Increased height
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut controller = Controller::new();
    
    loop {
        // Clear screen with white background
        clear_background(WHITE);

        // Update UI
        controller.update();

        // Draw canvas
        controller.draw();
        egui_macroquad::draw();

        // Wait for the next frame
        next_frame().await;
    }
}
