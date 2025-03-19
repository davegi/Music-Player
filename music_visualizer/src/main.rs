use egui_macroquad::Egui;
use macroquad::prelude::*;

#[macroquad::main("Test App")]
async fn main() {
    let mut egui = Egui::new();

    loop {
        clear_background(BLACK);

        egui.run(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Test App");
            });
        });

        egui.draw();
        next_frame().await;
    }
}
