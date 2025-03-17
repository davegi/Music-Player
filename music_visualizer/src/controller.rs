use crate::simple_menu::SimpleMenu;
use crate::song::Song;

use eframe::egui;

//TODO: Move most egui menu type code into simple menu (Make update much smaller just a overall controller)

/// A controller for managing a song's playback and menu interactions.
///
/// The `Controller` struct manages the state of a song, its playback status,
/// and the selection of songs from the menu.
pub struct Controller {
    /// The currently selected song. This is an `Option<Song>`, so it can be `None`
    /// if no song is selected, or `Some(Song)` when a song is active.
    pub song: Option<Song>,

    /// The menu used to select a song.
    pub menu: SimpleMenu,
}

impl Controller {
    /// Creates a new `Controller` instance with no song selected.
    ///
    /// Initializes the `Controller` with the song set to `None` and a new `SimpleMenu`.
    ///
    /// # Returns
    /// A new `Controller` instance.
    pub fn new() -> Self {
        Self {
            song: None,
            menu: SimpleMenu::new(),
        }
    }

    /// Updates the controller's state and renders the UI.
    ///
    /// This method checks if a song has been selected. If no song is selected, it
    /// shows a song selection menu. If a song is selected, it will show either a
    /// "Play" or "Pause" button depending on the song's playback status.
    ///
    /// # Arguments
    /// * `ctx` - The context for rendering the UI, provided by `eframe::egui`.
    pub fn update(&mut self, ctx: &egui::Context) {
        if self.song.is_none() {
            // No song is selected, show the song selection menu
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Select a song");
                if let Some(selected_song) = self.menu.show(ui) {
                    self.song = Some(Song::new(&selected_song));
                }
            });
        } else {
            // A song is selected, show the Play/Pause button
            egui::CentralPanel::default().show(ctx, |ui| {
                if let Some(song) = &self.song {
                    if song.status().is_paused {
                        // If the song is paused, show the Play button
                        if ui.button("Play").clicked() {
                            if let Some(song) = &mut self.song {
                                song.play();
                            }
                        }
                    } else {
                        // If the song is playing, show the Pause button
                        if ui.button("Pause").clicked() {
                            if let Some(song) = &mut self.song {
                                song.pause();
                            }
                        }
                    }
                }
            });
        }
    }
}
