// Import module definitions
mod controller;
mod list_item;
mod todo_list;

// Grouped assets module
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
use crate::assets::egui_utils::{Drawable, Scene, Updatable};

// Import the Controller, which manages the todo list logic
use controller::Controller;

// Import necessary components from eframe and egui for UI rendering
use eframe::egui::{self, CentralPanel, Context, Frame};

/// `MyApp` serves as the main application struct.
/// It holds the `Scene`, which contains all drawable and updatable elements.
pub struct MyApp {
    scene: Scene, // Manages all interactive UI components
}

impl MyApp {
    /// Creates a new instance of `MyApp`
    pub fn new() -> Self {
        let mut scene = Scene::new(); // Initialize an empty scene

        let controller = Controller::new(); // Create a new Controller instance

        scene.add(controller); // Add the controller to the scene

        Self { scene }
    }
}

// Implement the eframe App trait for `MyApp`, defining the main update loop
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Update all objects in the scene
        self.scene.update(ctx);

        // Render the central panel where UI elements are drawn
        CentralPanel::default()
            .frame(Frame::none().fill(*theme::BACKGROUND)) // Apply background color
            .show(ctx, |ui| {
                self.scene.draw(ui); // Draw all UI elements in the scene
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
        "Todo List App",                        // Window title
        native_options,                         // Window configuration
        Box::new(|_cc| Box::new(MyApp::new())), // Create an instance of `MyApp`
    )
    .expect("Failed to start application"); // Handle errors if the app fails to start
}
