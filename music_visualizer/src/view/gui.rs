use crate::controller::audio::MusicController;
use eframe::{App, NativeOptions, egui};

pub fn run_gui(music_controller: &mut MusicController) {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Rust Music Player",
        native_options,
        Box::new(|_cc| Ok(Box::new(Gui { music_controller }))),
    )
    .expect("Failed to start GUI");
}

struct Gui<'a> {
    music_controller: &'a mut MusicController,
}

impl<'a> App for Gui<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Music Player");
            if ui.button("Play Song").clicked() {
                self.music_controller
                    .load_and_play("music_library/scarlet_begonias.mp3");
            }
        });
    }
}
