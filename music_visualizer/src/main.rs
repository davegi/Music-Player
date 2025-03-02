mod audio;
mod simple_menu;

use audio::Audio;
use simple_menu::SimpleMenu;

use eframe::NativeOptions;
use eframe::egui;
use egui::ViewportBuilder;

struct MyApp {
    audio: Audio,
}

impl MyApp {
    fn new(song_name: &str) -> Self {
        Self {
            audio: Audio::new(&format!("music_library/{}.mp3", song_name)),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if self.audio.status().is_paused {
                    if ui.button("Play").clicked() {
                        self.audio.play();
                    }
                } else {
                    if ui.button("Pause").clicked() {
                        self.audio.pause();
                    }
                }
            });
        });
    }
}

fn main() {
    //TODO: try and find a way to add the menu to the app
    let menu = SimpleMenu::new();

    let song_name = menu.pick_song().trim().to_lowercase();

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([150.0, 100.0]), // Correct way to set window size
        ..Default::default()
    };

    eframe::run_native(
        "Audio Player",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(&song_name)))),
    )
    .unwrap();
}
