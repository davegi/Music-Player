# 🎵 Music Visualizer 1.0 - Pure Rust

This project is a feature-rich **Rust Music Visualizer** with a strict **Model-View-Controller (MVC)** architecture. It consists of:

- 🎶 **Music Controller** – Handles audio playback, playlists, and metadata.
- 🎨 **Visual Component** – Generates psychedelic visuals that react to the music.

Built with **egui**, **macroquad**, and **egui-macroquad** for the GUI.

---

## 🚀 Features
✅ **Play/Pause/Resume Songs**  
✅ **Playlist Management (JSON or SQLite)**  
✅ **Audio Metadata Extraction**  
✅ **Visual Effects Reacting to Music**  
✅ **Fast & Responsive UI with Egui**  

---

## 📦 Dependencies

| Purpose               | Library        |
|----------------------|--------------|
| **Audio Playback**   | `rodio` |
| **Audio Decoding**   | `symphonia` |
| **Metadata Parsing** | `lofty` |
| **File Management**  | `std::fs`, `walkdir` |
| **GUI**             | `egui`, `eframe`, `egui-macroquad` |
| **Visual Effects**  | `macroquad`, `rustfft`, `dasp` |
| **Storage**         | `serde + json/ron` or `rusqlite` |

---
