use crate::model::{playlist::Playlist, song::Song};

pub struct PlaylistController {
    playlist: Playlist,
}

impl PlaylistController {
    pub fn new(name: &str) -> Self {
        Self {
            playlist: Playlist::new(name),
        }
    }

    pub fn add_song(&mut self, song: Song) {
        self.playlist.add_song(song);
    }
}
