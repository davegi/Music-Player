use std::fs;
use std::io;

pub struct SimpleMenu;

impl SimpleMenu {
    pub fn new() -> Self {
        Self
    }

    fn list_files_in_directory(&self) -> Vec<String> {
        let mut file_names = Vec::new();

        if let Ok(entries) = fs::read_dir("music_library") {
            for entry in entries.flatten() {
                if let Some(file_stem) = entry.path().file_stem().and_then(|s| s.to_str()) {
                    file_names.push(file_stem.to_string());
                }
            }
        } else {
            eprintln!("Error reading directory");
        }

        file_names
    }

    pub fn pick_song(&self) -> String {
        println!("Available songs:");

        let files = self.list_files_in_directory();
        for file in files {
            println!(" - {}", file);
        }

        println!("Enter the name of the song you want to play:");

        let mut song_name = String::new();
        io::stdin().read_line(&mut song_name).unwrap();
        song_name
    }
}
