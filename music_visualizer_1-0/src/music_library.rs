use crate::song::Song;
use std::fs;

fn load_library() -> Vec<Song> {
    let mut songs = Vec::new();
    let wav_files = MusicLibrary::get_file_names("music_library");

    for file_name in wav_files {
        let song = Song::from_file(&format!("{}", file_name));
        songs.push(song);
    }

    songs
}

pub struct MusicLibrary {
    pub songs: Vec<Song>,
    pub selected_song: Song,
}

impl MusicLibrary {
    pub fn new() -> Self {
        MusicLibrary {
            songs: load_library(),
            selected_song: Song::from_file("charleston-girl-live.wav"), //Song::empty()
        }
    }

    fn get_file_names(dir_path: &str) -> Vec<String> {
        let mut file_names = Vec::new();

        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        file_names.push(file_name.to_string_lossy().into_owned());
                    }
                }
            }
        }

        file_names
    }

    pub fn select_song(&mut self, title: &str) {
        for song in &self.songs {
            if song.title == title {
                self.selected_song = Song::from_file(&song.filename);
                break;
            }
        }
    }

    pub fn get_song_names(&self) -> Vec<String> {
        self.songs.iter().map(|song| song.title.clone()).collect()
    }

    pub fn has_selected_song(&self) -> bool {
        !self.selected_song.title.is_empty()
    }
}
