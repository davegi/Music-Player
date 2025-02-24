use eframe::egui;
use eframe::egui::{Color32, Pos2, Vec2};
mod colors;
mod egui_utils; // Import colors module

use colors::theme::BACKGROUND;

struct MyApp {
    clicked: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { clicked: false }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(*BACKGROUND))
            .show(ctx, |ui| {
                let painter = ui.painter();

                // Use EguiUtils to draw elements
                egui_utils::EguiUtils::draw_text(
                    ui,
                    painter,
                    "Hello, egui!",
                    Pos2::new(100.0, 50.0),
                    Color32::WHITE,
                );
                egui_utils::EguiUtils::draw_rect(
                    painter,
                    Pos2::new(150.0, 100.0),
                    Vec2::new(200.0, 100.0),
                    Color32::RED,
                );
                egui_utils::EguiUtils::draw_circle(
                    painter,
                    Pos2::new(300.0, 300.0),
                    50.0,
                    Color32::BLUE,
                );
                egui_utils::EguiUtils::draw_line(
                    painter,
                    Pos2::new(100.0, 400.0),
                    Pos2::new(400.0, 400.0),
                    2.0,
                    Color32::GREEN,
                );

                if ui.button("Click Me!").clicked() {
                    self.clicked = !self.clicked;
                }

                if self.clicked {
                    println!("Button Clicked!");
                    self.clicked = false;
                }
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
