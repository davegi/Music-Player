use crate::brush::Brush;
use crate::screen::Screen;
use crate::settings::Settings;
use macroquad::prelude::*;

/// The main controller that manages the brush, screen, and settings.
pub struct Controller {
    pub brush: Brush,
    pub screen: Screen,
    pub settings: Settings,
}

impl Controller {
    /// Creates a new instance of `Controller` with default values.
    pub fn new() -> Self {
        Self {
            brush: Brush::new(),
            screen: Screen::new(),
            settings: Settings::new(),
        }
    }

    /// Updates the application state, handling input and settings changes.
    pub fn update(&mut self) {
        // Update UI settings (Egui window)
        self.settings.update();

        // Handle drawing when the left mouse button is pressed
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position(); // Get current mouse position
            self.brush.update(
                &mut self.screen,
                Vec2::new(x, y), // Convert tuple to `Vec2`
                self.settings.brush_color,
                self.settings.brush_size,
            );
        }

        // Clear the screen if the clear button was pressed
        if self.settings.is_clear {
            self.screen.clear();
        }
    }

    /// Draws the current state of the screen.
    pub fn draw(&self) {
        self.screen.draw();
    }
}
