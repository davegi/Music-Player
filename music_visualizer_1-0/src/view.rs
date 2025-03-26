//! Visualization module
//!
//! Handles the main display area that shows playback status through:
//! - Color changes (green for playing, red for paused)
//! - Text status indicators
//! - Responsive layout based on assigned rectangle

use nannou::prelude::*;

/// Represents the main visualization view
///
/// Manages:
/// - Playback state visualization
/// - Display area dimensions
/// - Visual feedback rendering
pub struct View {
    /// Rectangle defining the view's bounds and position
    view_rect: Rect,
    /// Current playback state (true when audio is playing)
    is_playing: bool,
}

impl View {
    /// Creates a new View with specified dimensions
    ///
    /// # Arguments
    /// * `view_rect` - The bounding rectangle for the view area
    ///
    /// Initializes with paused (red) state by default
    pub fn new(view_rect: Rect) -> Self {
        View {
            view_rect,
            is_playing: false,
        }
    }

    /// Updates the playback state
    ///
    /// # Arguments
    /// * `is_playing` - New playback state (true for playing, false for paused)
    ///
    /// This affects both the background color and status text
    pub fn update(&mut self, is_playing: bool) {
        self.is_playing = is_playing;
    }

    /// Renders the visualization
    ///
    /// Draws:
    /// - Background rectangle with state-appropriate color
    ///   - Green with 80% opacity when playing
    ///   - Red with 80% opacity when paused
    /// - Centered status text
    ///   - "PLAYING" when active
    ///   - "PAUSED" when inactive
    ///
    /// # Arguments
    /// * `draw` - Nannou Draw context for rendering
    pub fn draw(&self, draw: &Draw) {
        // Set color based on playback state
        let bg_color = if self.is_playing {
            rgba(0.0, 0.5, 0.0, 0.8) // Green when playing
        } else {
            rgba(0.5, 0.0, 0.0, 0.8) // Red when paused
        };

        // Draw background
        draw.rect()
            .xy(self.view_rect.xy())
            .wh(self.view_rect.wh())
            .color(bg_color);

        // Add status text overlay
        let status_text = if self.is_playing { "PLAYING" } else { "PAUSED" };

        draw.text(status_text)
            .xy(self.view_rect.xy())
            .color(WHITE)
            .font_size(48);
    }
}
