use macroquad::prelude::*;

/// The `View` struct represents the visual state of the application.
/// It manages the background color and playback state.
pub struct View {
    /// The current background color of the view.
    pub color: Color,
    /// A flag indicating whether playback is paused.
    pub is_paused: bool,
}

impl View {
    /// Creates a new `View` instance with default values.
    ///
    /// # Returns
    ///
    /// A new `View` instance with a green background and playback unpaused.
    pub fn new() -> Self {
        Self {
            color: GREEN, // Default color when not paused
            is_paused: false,
        }
    }

    /// Updates the view state based on the playback status.
    /// If playback is paused, the background turns red; otherwise, it remains green.
    pub fn update(&mut self) {
        if self.is_paused {
            self.color = RED; // Change color to red when paused
        } else {
            self.color = GREEN; // Change color to green when playing
        }
    }

    /// Draws the view as a colored rectangle filling the screen.
    pub fn draw(&self) {
        const SCREEN_WIDTH: f32 = 600.0;
        const SCREEN_HEIGHT: f32 = 600.0;

        // Render a rectangle with the current background color
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, self.color);
    }
}
