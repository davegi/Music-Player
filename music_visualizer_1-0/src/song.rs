//! Audio playback and song management module
//!
//! Handles loading and playing WAV audio files using CPAL for audio output.
//! Manages playback state and audio stream lifecycle.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Represents an audio song with playback capabilities
///
/// Manages the audio playback state, current position in the song,
/// and the underlying CPAL audio stream. Provides simple play/pause controls.
///

pub struct Song {
    /// Current playback state (true if playing)
    is_playing: bool,
    /// Active audio stream (None when paused/stopped)
    audio_stream: Option<cpal::Stream>,
    /// Shared audio sample data (32-bit float samples between -1.0 and 1.0)
    audio_data: Arc<Mutex<Vec<f32>>>,
    /// Current playback position in samples
    current_frame: usize,
    pub title: String,
    pub filename: String,
}

impl Song {
    /// Creates a new Song instance from file
    pub fn from_file(song_file_name: &str) -> Self {
        let song_path = format!("music_library/{}", song_file_name);
        let audio_data = match Self::load_wav(&song_path) {
            Ok(data) => Arc::new(Mutex::new(data)),
            Err(e) => {
                eprintln!("Failed to load audio file: {}", e);
                Arc::new(Mutex::new(Vec::new()))
            }
        };

        Song {
            is_playing: false,
            audio_stream: None,
            audio_data,
            current_frame: 0,
            title: Self::parse_title(song_file_name),
            filename: song_file_name.to_string(),
        }
    }

    /// Creates an empty Song instance
    pub fn empty() -> Self {
        Song {
            is_playing: false,
            audio_stream: None,
            audio_data: Arc::new(Mutex::new(Vec::new())),
            current_frame: 0,
            title: "".to_string(),
            filename: "".to_string(),
        }
    }

    /// Updates playback state based on external command
    ///
    /// # Arguments
    /// * `should_play` - True if audio should be playing, false if paused
    ///
    /// This will automatically start or stop playback as needed.
    pub fn update(&mut self, should_play: bool) {
        if should_play && !self.is_playing {
            self.play();
        } else if !should_play && self.is_playing {
            self.pause();
        }
        self.is_playing = should_play;
    }

    /// Returns current playback state
    ///
    /// # Returns
    /// True if audio is currently playing, false if paused/stopped
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Starts audio playback
    ///
    /// Initializes audio stream if not already playing.
    /// Uses the default audio output device.
    ///
    /// # Panics
    /// - If no audio output device is available
    /// - If audio device configuration is unsupported
    fn play(&mut self) {
        if self.audio_stream.is_some() {
            return;
        }

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let config = device.default_output_config().unwrap();

        let audio_data = self.audio_data.clone();
        let mut current_frame = self.current_frame;

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let audio_data = audio_data.lock().unwrap();
                    for sample in data.iter_mut() {
                        *sample = if current_frame < audio_data.len() {
                            audio_data[current_frame]
                        } else {
                            0.0
                        };
                        current_frame += 1;
                    }
                },
                move |err| eprintln!("an error occurred on stream: {}", err),
                None,
            ),
            _ => panic!("unsupported sample format"),
        };

        match stream {
            Ok(stream) => {
                stream.play().unwrap();
                self.audio_stream = Some(stream);
            }
            Err(e) => eprintln!("Error creating audio stream: {}", e),
        }
    }

    /// Pauses audio playback
    ///
    /// Stops the audio stream and maintains current playback position.
    fn pause(&mut self) {
        if let Some(stream) = self.audio_stream.take() {
            drop(stream); // This will stop the stream
        }
    }

    pub fn parse_title(song_file_name: &str) -> String {
        // Remove the file extension if present
        let mut title = song_file_name.to_string();
        if let Some(index) = title.rfind('.') {
            title.truncate(index);
        }

        // Replace hyphens with spaces and capitalize each word
        title
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Loads audio samples from WAV file
    ///
    /// # Arguments
    /// * `path` - Path to WAV file (16-bit PCM format)
    ///
    /// # Returns
    /// Result containing vector of normalized f32 samples (-1.0 to 1.0) or error
    ///
    /// # Errors
    /// Returns hound::Error if file cannot be read or is in invalid format
    fn load_wav(path: &str) -> Result<Vec<f32>, hound::Error> {
        let reader = hound::WavReader::open(Path::new(path))?;
        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.unwrap_or(0) as f32 / i16::MAX as f32)
            .collect();
        Ok(samples)
    }
}
