use crate::menu::Menu;
use crate::view::View;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

/// The `Controller` struct manages the main game loop and interactions between
/// the UI (`Menu`) and the game state (`View`).
pub struct Controller {
    /// Shared reference to the `View`, allowing UI elements to modify it safely.
    view: Arc<Mutex<View>>,
    /// The `Menu` responsible for UI interactions.
    menu: Menu,
}

impl Controller {
    /// Creates a new `Controller` instance, initializing the game state (`View`)
    /// and linking it to the `Menu`.
    pub fn new() -> Self {
        let view = Arc::new(Mutex::new(View::new())); // Create shared game state
        let menu = Menu::new(view.clone()); // Pass shared state to the UI

        Self { view, menu }
    }

    /// Runs the main game loop, handling updates, rendering, and UI interactions.
    pub async fn run(&mut self) {
        loop {
            // Clear the screen before rendering the next frame
            clear_background(BLACK);

            // Lock the `View` state to safely update and draw the game elements
            {
                let mut view = self.view.lock().unwrap();
                view.update(); // Update game logic
                view.draw(); // Render game elements
            }

            // Update the UI menu
            self.menu.update();

            // Render the Egui UI components
            egui_macroquad::draw();

            // Proceed to the next frame
            next_frame().await;
        }
    }
}
