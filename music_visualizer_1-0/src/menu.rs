//! User interface menu module
//!
//! Handles the interactive control panel for the application, including:
//! - Play/pause button
//! - Menu layout and rendering
//! - Mouse interaction handling
//!
//! The menu provides visual feedback and translates user input into playback commands.

use crate::music_library::MusicLibrary;
use nannou::prelude::*;

/// Represents the interactive control menu
///
/// Manages:
/// - Playback state tracking
/// - Button layout and rendering
/// - Mouse interaction handling
/// - Visual feedback
pub struct Menu {
    /// Current playback state (true when audio is playing)
    is_playing: bool,
    /// Rectangle defining the play/pause button bounds
    play_button_rect: Rect,
    /// Rectangle defining the entire menu area
    menu_rect: Rect,
    /// Collection of all interactive buttons
    buttons: Vec<Rect>,
    /// Tracks mouse state from previous frame for click detection
    was_mouse_pressed: bool,
    pub music_library: MusicLibrary,
}

impl Menu {
    /// Creates a new Menu with default layout
    ///
    /// # Arguments
    /// * `menu_rect` - The bounding rectangle for the entire menu panel
    ///
    /// # Layout
    /// - Play/pause button is centered horizontally
    /// - Button takes up 80% of menu width
    /// - Positioned 30% down from top of menu
    pub fn new(menu_rect: Rect) -> Self {
        let button_height = 50.0;
        let button_width = menu_rect.w() * 0.8;

        let play_button = Rect::from_x_y_w_h(
            menu_rect.x(),
            menu_rect.y() + menu_rect.h() * 0.3,
            button_width,
            button_height,
        );

        Menu {
            is_playing: false,
            music_library: MusicLibrary::new(),
            play_button_rect: play_button,
            menu_rect,
            buttons: vec![play_button],
            was_mouse_pressed: false,
        }
    }

    /// Updates menu state based on user input
    ///
    /// Handles:
    /// - Mouse position tracking
    /// - Click detection (only triggers on new presses)
    /// - Button state toggling
    ///
    /// # Arguments
    /// * `app` - Reference to Nannou application for input access
    pub fn update(&mut self, app: &App) {
        let mouse = app.mouse.position();
        let is_mouse_pressed = app.mouse.buttons.pressed().next().is_some();

        // Only trigger on new presses, not while holding
        if is_mouse_pressed && !self.was_mouse_pressed {
            for (i, button) in self.buttons.iter().enumerate() {
                if button.contains(mouse) {
                    match i {
                        0 => self.is_playing = !self.is_playing,
                        _ => {}
                    }
                    break; // Only handle one button per click
                }
            }
        }

        self.was_mouse_pressed = is_mouse_pressed;
    }

    /// Renders the menu and all its components
    ///
    /// Draws:
    /// - Menu background panel
    /// - Play/pause button with state-appropriate color
    /// - Button text label
    /// - Menu title
    ///
    /// # Arguments
    /// * `draw` - Nannou Draw context for rendering
    pub fn draw(&self, draw: &Draw) {
        // Draw menu background
        draw.rect()
            .xy(self.menu_rect.xy())
            .wh(self.menu_rect.wh())
            .color(rgb(0.1, 0.1, 0.1));

        if self.music_library.has_selected_song() {
            self.draw_playback_controls(draw);
        } else {
            self.draw_song_selection_controls(draw);
        }
    }

    fn draw_playback_controls(&self, draw: &Draw) {
        // Draw buttons with state-dependent color
        let button_color = if self.is_playing { GREEN } else { RED };

        draw.rect()
            .xy(self.play_button_rect.xy())
            .wh(self.play_button_rect.wh())
            .color(button_color);

        // Draw button text
        draw.text(if self.is_playing { "PAUSE" } else { "PLAY" })
            .xy(self.play_button_rect.xy())
            .color(BLACK)
            .font_size(24);

        // Draw menu title
        draw.text("CONTROLS")
            .xy(pt2(self.menu_rect.x(), self.menu_rect.top() - 30.0))
            .color(WHITE)
            .font_size(30);
    }

    fn draw_song_selection_controls(&self, draw: &Draw) {
        for (index, name) in self.music_library.get_song_names().iter().enumerate() {
            draw.text(&name)
                .xy(pt2(
                    self.menu_rect.x(),
                    self.menu_rect.top() - (150.0 + (100.0 * index as f32)),
                ))
                .color(WHITE)
                .font_size(30);
        }
        // Draw menu title
        draw.text("SONG SELECTION")
            .xy(pt2(self.menu_rect.x(), self.menu_rect.top() - 30.0))
            .color(WHITE)
            .font_size(30);
    }

    /// Returns current playback state
    ///
    /// # Returns
    /// `true` if audio should be playing, `false` if paused
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}
