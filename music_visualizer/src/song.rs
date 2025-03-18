use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

/// Represents the status of a song, indicating whether it is paused.
pub struct SongStatus {
    pub is_paused: bool,
}

/// Manages audio playback using the `rodio` library.
pub struct Song {
    pub name: String,
    _stream: OutputStream, // Keeps the audio stream alive
    pub sink: Sink,        // Controls playback operations
}

impl Song {
    /// Creates a new `Song` instance and loads an audio file.
    ///
    /// # Arguments
    /// * `song_file_path` - Path to the audio file to be played.
    ///
    /// # Panics
    /// * If the song file cannot be opened or decoded.
    pub fn new(song_file_path: &str) -> Self {
        // Create an output audio stream and its associated handle
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        // Create a sink (audio controller) linked to the stream handle
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Try opening the specified audio file and provide more context on failure
        let file =
            File::open(format!("music_library/{}.mp3", song_file_path)).unwrap_or_else(|_| {
                panic!(
                    "Failed to open file at path: music_library/{}.mp3",
                    song_file_path
                );
            });

        // Decode the audio file into a source format suitable for playback
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

        // Append the decoded audio source to the sink for playback
        sink.append(source);

        Self {
            name: song_file_path.to_string(),
            _stream: stream, // Keep the stream alive to prevent audio cutoff
            sink,
        }
    }

    /// Starts or resumes audio playback.
    pub fn play(&mut self) {
        self.sink.play();
    }

    /// Pauses audio playback.
    pub fn pause(&mut self) {
        self.sink.pause();
    }

    /// Retrieves the current playback status.
    ///
    /// # Returns
    /// A `SongStatus` struct indicating whether the song is paused.
    pub fn status(&self) -> SongStatus {
        SongStatus {
            is_paused: self.sink.is_paused(),
        }
    }
}
