use eframe::egui;
use eframe::{App, NativeOptions};
use macroquad::miniquad::window::set_window_size;

pub struct PureEguiView;

impl PureEguiView {
    pub fn new() -> Self {
        Self
    }
}

impl App for PureEguiView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Music Player");
            if ui.button("Play").clicked() {
                println!("Play button clicked");
            }
        });
    }
}

impl super::super::app::View for PureEguiView {
    fn run(&mut self) {
        let options = NativeOptions::default();
        eframe::run_native(
            "Rust Music Player",
            options,
            Box::new(|_cc| Ok(Box::new(PureEguiView))),
        )
        .expect("Failed to start GUI");
    }

    fn setup(&mut self) {
        set_window_size(1, 1);
    }
}
