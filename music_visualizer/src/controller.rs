use crate::simple_menu::SimpleMenu;
use crate::song::Song;
use eframe::egui;

/// A controller for managing song playback and menu interactions.
///
/// The `Controller` is responsible for handling the UI state between
/// the song selection menu and the playback screen. It interacts with
/// [`SimpleMenu`] to allow users to select a song and control playback.
pub struct Controller {
    /// The currently selected song.
    ///
    /// This will be `None` when no song is selected. Once a song is chosen
    /// from the selection screen, it is stored as `Some(Song)`. If the user
    /// returns to the selection screen, it is set back to `None`.
    pub song: Option<Song>,

    /// The menu for selecting a song and managing playback.
    ///
    /// This instance of [`SimpleMenu`] handles all user interactions related to
    /// song selection and playback controls.
    pub menu: SimpleMenu,
}

impl Controller {
    /// Creates a new `Controller` instance with no song selected.
    ///
    /// Initializes the `SimpleMenu` for song selection and playback control.
    ///
    /// # Returns
    ///
    /// A new instance of `Controller` with an empty song selection.
    pub fn new() -> Self {
        Self {
            song: None, // No song selected at startup
            menu: SimpleMenu::new(),
        }
    }

    /// Updates the UI state and handles user interactions.
    ///
    /// This function determines whether the song selection screen or the playback
    /// screen should be displayed. If no song is selected, it presents the song
    /// selection menu. If a song is selected, it switches to the playback screen.
    ///
    /// If the user presses the "Back to Song Selection" button while in the
    /// playback screen, the selected song is cleared, and the UI returns to
    /// the song selection menu.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The [`egui::Context`] that provides the UI state for rendering.
    pub fn update(&mut self, ctx: &egui::Context) {
        if self.song.is_none() {
            // Display song selection screen and update `song` if one is selected
            self.song = self.menu.show_song_selection_screen(ctx, self.song.take());
        } else {
            // Display playback screen; if "Back" is pressed, return to selection
            if self
                .menu
                .show_play_pause_screen(ctx, self.song.as_mut().unwrap())
            {
                self.song = None; // Reset to selection screen
            }
        }
    }
}
