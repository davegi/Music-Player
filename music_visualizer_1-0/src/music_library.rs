// Import required modules and types
use crate::song::Song; // Song struct from local song module
use std::fs; // Standard filesystem operations

/// Loads all WAV files from the music library directory into Song objects
///
/// # Returns
/// A vector containing Song objects for all WAV files found
fn load_library() -> Vec<Song> {
    let mut songs = Vec::new(); // Create empty vector to store songs

    // Get list of all WAV files in music library directory
    let wav_files = MusicLibrary::get_file_names("music_library");

    // Convert each filename to a Song object and add to vector
    for file_name in wav_files {
        // Create song from file path (format adds directory prefix)
        let song = Song::from_file(&format!("{}", file_name));
        songs.push(song);
    }

    songs // Return populated vector
}

/// Represents a collection of songs with selection capabilities
pub struct MusicLibrary {
    pub songs: Vec<Song>,    // All songs in the library
    pub selected_song: Song, // Currently selected song for playback
}

impl MusicLibrary {
    /// Creates a new MusicLibrary instance
    ///
    /// # Returns
    /// Initialized MusicLibrary with all songs loaded and default selection
    pub fn new() -> Self {
        MusicLibrary {
            songs: load_library(), // Load all songs from directory
            // Set default selected song (using a popular track as example)
            selected_song: Song::empty(),
            // Song::from_file("charleston-girl-live.wav")
        }
    }

    /// Gets all filenames from a directory
    ///
    /// # Arguments
    /// * `dir_path` - Path to directory to scan
    ///
    /// # Returns
    /// Vector of filenames as Strings
    fn get_file_names(dir_path: &str) -> Vec<String> {
        let mut file_names = Vec::new(); // Create empty vector for results

        // Attempt to read directory entries
        if let Ok(entries) = fs::read_dir(dir_path) {
            // Process each directory entry that was successfully read
            for entry in entries.flatten() {
                let path = entry.path(); // Get full path of entry

                // Only process files (skip directories)
                if path.is_file() {
                    // Convert filename to String if possible
                    if let Some(file_name) = path.file_name() {
                        file_names.push(file_name.to_string_lossy().into_owned());
                    }
                }
            }
        }

        file_names // Return collected filenames
    }

    /// Selects a song from the library by title
    ///
    /// # Arguments
    /// * `title` - Title of song to select
    pub fn select_song(&mut self, title: &str) {
        // Search through all songs for matching title
        for song in &self.songs {
            if song.title == title {
                // Create new Song instance from filename when found
                self.selected_song = Song::from_file(&song.filename);
                break; // Exit loop after first match
            }
        }
    }

    /// Gets all song titles in the library
    ///
    /// # Returns
    /// Vector of song title strings
    pub fn get_song_names(&self) -> Vec<String> {
        // Map each song to its title and collect into vector
        self.songs.iter().map(|song| song.title.clone()).collect()
    }

    /// Checks if a song is currently selected
    ///
    /// # Returns
    /// true if a valid song is selected, false otherwise
    pub fn has_selected_song(&self) -> bool {
        // Simple check for non-empty title
        !self.selected_song.title.is_empty()
    }
}
