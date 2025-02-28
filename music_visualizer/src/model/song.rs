pub struct Song {
    pub title: String,
    pub artist: String,
    pub path: String,
    pub duration: u32, // in seconds
}

impl Song {
    pub fn new(title: &str, artist: &str, path: &str, duration: u32) -> Self {
        Self {
            title: title.to_string(),
            artist: artist.to_string(),
            path: path.to_string(),
            duration,
        }
    }
}
