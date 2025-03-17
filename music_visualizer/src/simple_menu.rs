use eframe::egui;
use std::fs;

/// A simple menu for selecting a song from a list of songs in a directory.
pub struct SimpleMenu {
    /// A list of song names (as strings) retrieved from the music library directory.
    song_list: Vec<String>,

    /// The song currently selected from the menu, if any.
    selected_song: Option<String>,
}

impl SimpleMenu {
    /// Creates a new `SimpleMenu` by reading song files from the "music_library" directory.
    ///
    /// This method attempts to read all files in the "music_library" directory,
    /// extracting the file names (without extensions) as the list of available songs.
    /// If the directory cannot be read or there are any errors, the song list will default
    /// to an empty vector.
    ///
    /// # Returns
    /// A new `SimpleMenu` instance with the list of songs and no song selected initially.
    pub fn new() -> Self {
        let song_list = fs::read_dir("music_library")
            .and_then(|entries| {
                entries
                    .map(|entry| {
                        entry
                            .ok()
                            .and_then(|e| e.path().file_stem()?.to_str().map(|s| s.to_string()))
                    })
                    .collect::<Option<Vec<String>>>()
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::Other, "Failed to read directory")
                    })
            })
            .unwrap_or_else(|_| vec![]); // Default to empty vec if directory read fails

        Self {
            song_list,
            selected_song: None,
        }
    }

    /// Displays the song selection menu and returns the selected song.
    ///
    /// This method iterates over the list of available songs and creates a button for each.
    /// When a button is clicked, it sets the `selected_song` to the corresponding song name.
    /// The selected song is returned as an `Option<String>`.
    ///
    /// # Arguments
    /// * `ui` - A mutable reference to the `egui::Ui` used for rendering the interface.
    ///
    /// # Returns
    /// The selected song as `Option<String>`, or `None` if no song was selected.
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        for song in &self.song_list {
            if ui.button(song).clicked() {
                self.selected_song = Some(song.clone());
            }
        }
        self.selected_song.clone()
    }
}
