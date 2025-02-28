use super::song::Song;

pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Song>,
}

impl Playlist {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tracks: Vec::new(),
        }
    }

    pub fn add_song(&mut self, song: Song) {
        self.tracks.push(song);
    }
}
