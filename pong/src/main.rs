// Grouped assets module
mod ball;
mod board;
mod controller;
mod paddle; // Include the new controller module

mod assets {
    pub mod colors;
    pub mod constants;
    pub mod egui_utils;
    pub mod helper_functions;
    pub mod key_coords;
}

// Import necessary components from the assets module
use crate::assets::colors::theme;
use crate::assets::constants::*;
use crate::assets::egui_utils::{Drawable, Updatable};

use crate::controller::Controller;

// Import necessary components from eframe and egui for UI rendering
use eframe::egui::{self, CentralPanel, Context, Frame};
use std::time::{Duration, Instant};

/// `MyApp` serves as the main application struct.
/// It holds the `Controller`, which manages all game logic.
pub struct MyApp {
    controller: Controller, // Replaces Scene with Controller
    last_update: Instant,
}

impl MyApp {
    /// Creates a new instance of `MyApp`
    pub fn new() -> Self {
        Self {
            controller: Controller::new(), // Use Controller instead of Scene
            last_update: Instant::now(),
        }
    }
}

// Implement the eframe App trait for `MyApp`, defining the main update loop
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Force constant updates (60 FPS)
        ctx.request_repaint_after(Duration::from_secs_f64(1.0 / 60.0));

        // Update game logic
        self.controller.update(ctx);

        // Render the central panel where UI elements are drawn
        CentralPanel::default()
            .frame(Frame::NONE.fill(*theme::BACKGROUND)) // Apply background color
            .show(ctx, |ui| {
                self.controller.paddle1.draw(ui);
                self.controller.paddle2.draw(ui);
                self.controller.ball.draw(ui);
            });
    }
}

/// Entry point for the application.
/// Configures window settings and launches the eframe application.
fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]), // Set initial window size
        ..Default::default()
    };

    // Start the application using `eframe::run_native`
    eframe::run_native(
        "Pong",         // Window title
        native_options, // Window configuration
        Box::new(
            |_cc| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
                Ok(Box::new(MyApp::new()))
            },
        ), // Create an instance of `MyApp`
    )
    .expect("Failed to start application"); // Handle errors if the app fails to start
}
