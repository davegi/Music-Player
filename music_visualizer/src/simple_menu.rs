use crate::song::Song;
use eframe::egui;
use std::fs;
use std::io;

/// A simple menu for selecting and controlling song playback.
pub struct SimpleMenu {
    /// List of song names in the music library.
    song_list: Vec<String>,
    /// The currently selected song, if any.
    selected_song: Option<String>,
}

impl SimpleMenu {
    /// Creates a new `SimpleMenu` by reading song files from the "music_library" directory.
    ///
    /// # Returns
    /// A new `SimpleMenu` instance with the song list loaded, or an empty list if the
    /// songs cannot be loaded.
    pub fn new() -> Self {
        // Try loading the song list from the "music_library" directory.
        let song_list = Self::load_song_list().unwrap_or_else(|_| vec![]);

        Self {
            song_list,
            selected_song: None,
        }
    }

    /// Loads the list of songs from the "music_library" directory.
    ///
    /// # Returns
    /// A result containing a vector of song names as strings or an error if the directory
    /// cannot be read.
    fn load_song_list() -> io::Result<Vec<String>> {
        // Read the contents of the "music_library" directory, filter valid entries, and collect song names.
        fs::read_dir("music_library")?
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.path().file_stem()?.to_str().map(|s| s.to_string()))
            })
            .collect::<Vec<String>>()
            .into_iter()
            .map(Ok)
            .collect()
    }

    /// Displays the play/pause screen and returns whether to go back to the song selection screen.
    ///
    /// # Arguments
    /// * `ctx` - The `egui` context to render the UI.
    /// * `song` - The current song that is being played or paused.
    ///
    /// # Returns
    /// `true` if the user wants to go back to the song selection screen, otherwise `false`.
    pub fn show_play_pause_screen(&mut self, ctx: &egui::Context, song: &mut Song) -> bool {
        let mut go_back = false;

        // Show the central panel with song info, play/pause button, and a back button.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Render the song information (name and separator).
                Self::render_song_info(ui, song);
                // Render the play/pause button.
                Self::render_play_pause_button(ui, song);
                // Render the back button and set the `go_back` flag if clicked.
                go_back = Self::render_back_button(ui);
            });
        });

        go_back
    }

    /// Renders the song information (name and separator).
    ///
    /// # Arguments
    /// * `ui` - The `egui::Ui` to render the song info in.
    /// * `song` - The current song to display its name.
    fn render_song_info(ui: &mut egui::Ui, song: &Song) {
        // Display the song name in the header.
        ui.heading(format!("Now Playing: {}", song.name));
        // Render a separator after the song name.
        ui.separator();
    }

    /// Renders the play/pause button and handles its click event.
    ///
    /// # Arguments
    /// * `ui` - The `egui::Ui` to render the button in.
    /// * `song` - The current song to control its playback status.
    fn render_play_pause_button(ui: &mut egui::Ui, song: &mut Song) {
        // Determine the text for the button based on the song's current status.
        let button_text = if song.status().is_paused {
            "▶ Play"
        } else {
            "⏸ Pause"
        };

        // Create a play/pause button with custom size.
        let play_pause_button = ui.add_sized([120.0, 50.0], egui::Button::new(button_text));

        // If the button is clicked, toggle the song's playback state.
        if play_pause_button.clicked() {
            if song.status().is_paused {
                song.play();
            } else {
                song.pause();
            }
        }

        // Add space between buttons to prevent overlapping.
        ui.add_space(225.0);
    }

    /// Renders a back button and returns whether it was clicked.
    ///
    /// # Arguments
    /// * `ui` - The `egui::Ui` to render the back button in.
    ///
    /// # Returns
    /// `true` if the back button was clicked, otherwise `false`.
    fn render_back_button(ui: &mut egui::Ui) -> bool {
        // Use RichText to customize the button's text with a specific font size.
        let back_button = ui.add_sized(
            [50.0, 50.0],
            egui::Button::new(egui::RichText::new("🔙").size(30.0)),
        );

        // Return whether the back button was clicked.
        back_button.clicked()
    }

    /// Displays the song selection screen and returns the selected song.
    ///
    /// # Arguments
    /// * `ctx` - The `egui::Context` to render the UI.
    /// * `song` - The currently selected song, or `None` if no song is selected.
    ///
    /// # Returns
    /// The selected `Song` object, or `None` if no song was selected.
    pub fn show_song_selection_screen(
        &mut self,
        ctx: &egui::Context,
        mut song: Option<Song>,
    ) -> Option<Song> {
        // Show the central panel with song selection options.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Render the header for the song selection screen.
                Self::render_song_selection_header(ui);
                // Render the list of songs and handle song selection.
                Self::render_song_list(ui, &self.song_list, &mut self.selected_song, &mut song);
            });
        });

        song
    }

    /// Renders the header for the song selection screen.
    ///
    /// # Arguments
    /// * `ui` - The `egui::Ui` to render the header in.
    fn render_song_selection_header(ui: &mut egui::Ui) {
        // Display the header for song selection.
        ui.heading("Select a Song");
        // Render a separator after the header.
        ui.separator();
    }

    /// Renders the list of songs and handles song selection.
    ///
    /// # Arguments
    /// * `ui` - The `egui::Ui` to render the list of songs.
    /// * `song_list` - The list of song names to display.
    /// * `selected_song` - The currently selected song, if any.
    /// * `current_song` - The current `Song` object to be updated based on selection.
    fn render_song_list(
        ui: &mut egui::Ui,
        song_list: &[String],
        selected_song: &mut Option<String>,
        current_song: &mut Option<Song>,
    ) {
        // If there are no songs, display a message.
        if song_list.is_empty() {
            ui.label("No songs found in the library.");
        } else {
            // Adjust the button padding for better UI spacing.
            ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);

            // Render a button for each song in the list.
            for song_name in song_list {
                let button = ui.add_sized(
                    [200.0, 40.0],
                    egui::Button::new(format!("🎵 {}", song_name)),
                );

                // If the button is clicked, set the selected song and create a new `Song` object.
                if button.clicked() {
                    *selected_song = Some(song_name.clone());
                    *current_song = Some(Song::new(song_name));
                }
            }
        }
    }
}
