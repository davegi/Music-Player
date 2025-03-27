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
///

pub struct MenuButton {
    title: String,
    tag: String,
    rect: Rect,
}

pub struct Menu {
    /// Current playback state (true when audio is playing)
    is_playing: bool,
    /// Rectangle defining the entire menu area
    menu_rect: Rect,
    /// Collection of all interactive buttons
    buttons: Vec<MenuButton>,
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
        Menu {
            is_playing: false,
            music_library: MusicLibrary::new(),
            menu_rect,
            buttons: vec![MenuButton {
                title: "PLAY".to_string(),
                tag: "play_button".to_string(),
                rect: Rect::from_x_y_w_h(
                    menu_rect.x(),
                    menu_rect.y() + menu_rect.h() * 0.3,
                    menu_rect.w() * 0.8,
                    50.0,
                ),
            }],
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
                if button.rect.contains(mouse) {
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

        let play_button = self.get_button("play_button");

        draw.rect()
            .xy(play_button.unwrap().rect.xy())
            .wh(play_button.unwrap().rect.wh())
            .color(button_color);

        // Draw button text
        draw.text(if self.is_playing { "PAUSE" } else { "PLAY" })
            .xy(play_button.unwrap().rect.xy())
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

    fn get_button(&self, tag: &str) -> Option<&MenuButton> {
        self.buttons.iter().find(|b| b.tag == tag)
    }

    // fn create_buttons(&mut self, menu_rect: Rect) -> Vec<MenuButton> {
    //     let mut buttons = Vec::new();

    //     let button_height = 50.0;
    //     let button_width = menu_rect.w() * 0.8;

    //     let play_button = Rect::from_x_y_w_h(
    //         menu_rect.x(),
    //         menu_rect.y() + menu_rect.h() * 0.3,
    //         button_width,
    //         button_height,
    //     );

    //     buttons.push(MenuButton {
    //         title: "PLAY".to_string(),
    //         tag: "play_button".to_string(),
    //         rect: play_button,
    //     });

    //     for button in Self::create_song_buttons(&self) {
    //         buttons.push(button);
    //     }

    //     buttons
    // }

    // fn create_song_buttons(&self) {
    //     let mut buttons = Vec::new();
    //     for (index, name) in self.music_library.get_song_names().iter().enumerate() {
    //         let button_rect = Rect::from_x_y_w_h(
    //             self.menu_rect.x(),
    //             self.menu_rect.top() - (100.0 + (100.0 * index as f32)),
    //             self.menu_rect.w(),
    //             50.0,
    //         );
    //         buttons.push(MenuButton {
    //             title: name.clone(),
    //             tag: format!("song_{}", index),
    //             rect: button_rect,
    //         });
    //     }

    //     self.buttons = buttons;
    // }

    /// Returns current playback state
    ///
    /// # Returns
    /// `true` if audio should be playing, `false` if paused
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}
