mod controller;
mod simple_menu;
mod song;

use controller::Controller;

use eframe::NativeOptions;
use eframe::egui;
use egui::ViewportBuilder;

/// Main application structure that manages audio playback.
struct MyApp {
    controller: Controller,
}

impl MyApp {
    /// Creates a new `MyApp` instance with the selected song.
    ///
    /// # Arguments
    /// * `song_name` - The name of the song to load and play.
    fn new() -> Self {
        Self {
            controller: Controller::new(),
        }
    }
}

/// Implements the GUI application using `eframe`.
impl eframe::App for MyApp {
    /// Updates the UI and handles button interactions for playback control.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.controller.update(ctx);
    }
}

/// Entry point of the application.
fn main() {
    // Define the window options for the application.
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([150.0, 100.0]),
        ..Default::default()
    };

    // Run the `eframe` application with the selected song.
    eframe::run_native(
        "Audio Player",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
    .unwrap();
}
