
## **Rust Music Visualizer - First Pass Plan**

This project will be structured with a strict **Model-View-Controller (MVC)** pattern to ensure separation of concerns. Below is a breakdown of how to tackle this project, including the **backend architecture, file storage**, **recommended libraries**, and **file structure**.


### **Step 1: Backend (Model & Controller) - First Pass**

The **backend** is responsible for managing **audio playback, file handling, playlists, metadata, and data storage**.

#### **1.1 Libraries to Use**

-   **Audio Handling**
    -   `rodio` – High-level audio playback library for Rust.
    -   `symphonia` – For decoding and metadata extraction (MP3, FLAC, WAV, etc.).
-   **File Management**
    -   `std::fs` & `walkdir` – To manage file I/O and directory scanning.
-   **Metadata Management**
    -   `lofty` – Read and write metadata for audio files (ID3 tags, album art, etc.).
-   **Data Storage (Playlists, Settings, Library)**
    -   **Option 1:** `serde + ron/toml/json` – For simple, human-readable storage.
    -   **Option 2:** `sqlite + rusqlite` – If you need a more scalable database.

----------

#### **1.2 File Storage Structure**

-   **Audio Files**: Users can load music from their local storage.
-   **Playlists & Settings**: Stored in `.json` or `.ron` files or an SQLite database.

**Recommended file structure:**
```rb
MusicPlayer/
│── src/
│   │── main.rs                   # Application entry point
│   │── controller/
│   │   ├── mod.rs                # Controller module
│   │   ├── audio.rs               # Handles playback logic
│   │   ├── playlist.rs            # Manages playlists
│   │── model/
│   │   ├── mod.rs                # Model module
│   │   ├── song.rs                # Defines song structure
│   │   ├── metadata.rs            # Handles song metadata
│   │   ├── storage.rs             # Handles file storage & DB
│   │── view/
│   │   ├── mod.rs                # View module
│   │   ├── gui.rs                 # Egui-based UI
│   │   ├── visualizer.rs          # Macroquad psychedelic visuals
│   │── app.rs                     # Core application logic
│   ├── Cargo.toml                 # Dependencies
│── assets/                        # UI and theme assets
│── music_library/                  # User music files
│── playlists/                      # Playlist storage
│── config/                         # Configuration files
```

**Example JSON Playlist:**
```json
{
  "name": "Favorites",
  "tracks": [
    {
      "title": "Song A",
      "artist": "Artist A",
      "path": "music_library/song_a.mp3",
      "duration": 245
    }
  ]
}
```

----------

### **Step 2: Visual Component (View)**

The **visualizer** will generate psychedelic visuals that react to audio input.

#### **2.1 Libraries to Use**

-   **Audio Analysis**
    -   `hound` – Read raw audio samples.
    -   `symphonia` – Decode and analyze audio in real-time.
-   **Graphics**
    -   `macroquad` – For real-time visuals and animations.
    -   `egui-macroquad` – Overlay `egui` UI elements.
-   **DSP & Signal Processing**
    -   `rustfft` – Fast Fourier Transform (FFT) for spectrum analysis.
    -   `dasp` – Digital signal processing.

----------


### **Step 4: Building the Core Functionality**

#### **4.1 Implementing the Model**

#### `Song` struct:

```rust
`pub struct Song {
    pub title: String,
    pub artist: String,
    pub path: String,
    pub duration: u32, // seconds
}` 
```

#### `Playlist` struct:

```rust
`pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Song>,
}` 
```

#### `Storage` system (JSON example):
```rust
`fn save_playlist(playlist: &Playlist, path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(playlist)?;
    fs::write(path, json)
}` 
```
----------

#### **4.2 Implementing the Controller**

```rust
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct MusicController {
    sink: Sink,
}

impl MusicController {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self { sink }
    }

    pub fn play(&self, file_path: &str) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }
}` 
```
----------

#### **4.3 Implementing the View**

**Egui GUI example:**
```rust
use egui::{CentralPanel, Context};

pub fn ui(ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Rust Music Player");
        if ui.button("Play").clicked() {
            // Call controller to play
        }
    });
}` 
```
**Macroquad visualizer (basic start):**
```rust
use macroquad::prelude::*;

#[macroquad::main("Visualizer")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, RED);
        next_frame().await;
    }
}` 
```
----------

### **Summary**

-   ✅ Set up MVC file structure
-   ✅ Implement `rodio` for playback
-   ✅ Use `symphonia` for metadata extraction
-   ✅ Design GUI with `egui`
-   ✅ Add real-time visualizer with `macroquad`

# Summary of Useful Libraries and Frameworks for Audio-Visual Creative Coding in Rust

## Key Libraries and Frameworks

### 1. Audio Libraries
- **`cpal`**: Low-level audio input/output handling. Provides flexible access to audio devices, useful for capturing or playing back audio in real-time.
- **`rodio`**: Higher-level audio playback library, good for audio file playback and simple audio handling.
- **`rustfft`**: Provides Fast Fourier Transform (FFT) for signal processing, useful for visualizing audio spectrum (e.g., frequency analysis for equalizer effects).
- **`dasp`**: A digital signal processing library, useful for manipulating and processing audio signals in real-time.
- **`symphonia`**: A library for decoding audio files (MP3, FLAC, WAV) and extracting metadata. It’s more comprehensive and useful for handling diverse audio formats.
- **`hound`**: A simple library for reading and writing WAV files.

### 2. Graphics and Visualization Libraries
- **`macroquad`**: A 2D graphics library with support for real-time visualizations. It is suitable for generative art, simple animations, and audio-reactive visuals.
- **`miniquad`**: A lightweight graphics framework, ideal for GPU-accelerated, performance-oriented rendering (using WebGPU or OpenGL).
- **`wgpu`**: A modern GPU abstraction for Rust, focusing on high-performance graphics rendering with WebGPU. Suitable for complex, real-time, GPU-driven visualizations.
- **`glium`**: A low-level OpenGL wrapper in Rust, perfect for writing custom shaders and building complex, interactive visualizations.
- **`eframe` and `egui`**: Used for building UI components in your creative applications. `egui` is an immediate mode GUI, and `eframe` provides an application framework around `egui` for desktop apps.

### 3. Creative Coding Libraries
- **`shred`**: A framework focused on parallel processing for high-performance audio-visual applications, great for real-time, intensive projects like creative coding or generative art.
- **`bevy`**: A game engine that can also be used for creative coding, offering a flexible architecture with ECS (Entity-Component-System) for building interactive art or generative projects.

## General Architecture and Setup

### Audio Visualization
- **Use `rustfft`** for FFT analysis to extract frequency data from audio, then use this data to drive visual elements like equalizers or waveforms.
- **`cpal`** or **`rodio`** can be used for audio playback and streaming.
- Real-time visualization could be done with libraries like **`macroquad`** (2D), **`miniquad`** (performance), or **`wgpu`** (GPU-based).

### Graphics
- For 2D animations or interactive art based on audio input, **`macroquad`** provides a simple API.
- If you require more advanced rendering or real-time 3D visuals, **`wgpu`** or **`glium`** can provide the flexibility needed.

### UI and Integration
- Use **`eframe`/`egui`** for creating UI components (like buttons or control panels) that interact with your audio-visual visualizations.

## Summary
To create creative coding audio-visual projects in Rust:
- **`cpal`**, **`rodio`**, or **`symphonia`** handle the audio input/output and file decoding.
- **`rustfft`** or **`dasp`** will help with real-time signal analysis for visualizations.
- **`macroquad`**, **`miniquad`**, or **`wgpu`** enable efficient rendering of visual effects.
- **`eframe`/`egui`** can be used for UI components.

These libraries provide flexibility to build interactive, audio-reactive visualizations or installations in 2D or 3D, with a focus on performance, real-time processing, and GPU-acceleration where necessary.
