use eframe::egui;

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
        egui::CentralPanel::default().show(ctx, |ui| {
            let screen_size = ctx.screen_rect().size();
            let middle = egui::vec2(screen_size.x / 2.0, screen_size.y / 2.0);

            let rect = egui::Rect::from_min_size(
                egui::pos2(middle.x - 100.0, middle.y - 250.0),
                egui::vec2(200.0, 40.0),
            );

            ui.put(
                rect,
                egui::Label::new(egui::RichText::new("TODO List:").heading()),
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
