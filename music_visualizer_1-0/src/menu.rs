use egui_macroquad::egui::{Button, Window};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::view::View;

/// The `Menu` struct manages the UI overlay using `egui_macroquad`.
/// It allows user interaction with the playback controls.
pub struct Menu {
    /// Shared reference to the `View`, allowing UI elements to modify it safely.
    view: Arc<Mutex<View>>,
}

impl Menu {
    /// Creates a new `Menu` instance, linking it to the shared `View` state.
    ///
    /// # Arguments
    ///
    /// * `view` - A thread-safe, shared reference to the `View` instance.
    pub fn new(view: Arc<Mutex<View>>) -> Self {
        Self { view }
    }

    /// Updates the UI by rendering the control panel using `egui`.
    /// This includes buttons to toggle playback state.
    pub fn update(&mut self) {
        // Start the Egui frame
        egui_macroquad::ui(|egui_ctx| {
            // Create a control panel window
            Window::new("Controls")
                .fixed_pos([10.0, 10.0]) // Position the window in the top-left corner
                .resizable(false) // Make the window non-resizable
                .title_bar(true) // Show the title bar
                .show(egui_ctx, |ui| {
                    // Lock the shared `View` for safe access
                    let mut view = self.view.lock().unwrap();

                    // Display a label for playback controls
                    ui.label("Playback Controls:");

                    // Add a button to toggle play/pause
                    if view.is_paused {
                        if ui.add(Button::new("▶ Play")).clicked() {
                            view.is_paused = false; // Resume playback
                        }
                    } else {
                        if ui.add(Button::new("⏸ Pause")).clicked() {
                            view.is_paused = true; // Pause playback
                        }
                    }
                });
        });
    }
}
