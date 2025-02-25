mod colors;
mod controller;
mod egui_utils;
mod list_item;
mod todo_list;

use crate::colors::theme;
use crate::egui_utils::{Drawable, Scene, Updatable};
use crate::todo_list::TodoList;
use controller::Controller;

use eframe::egui::{self, CentralPanel, Context, Frame};

pub struct MyApp {
    scene: Scene,
}

impl MyApp {
    pub fn new() -> Self {
        let mut scene = Scene::new();

        let mut controller = Controller::new(TodoList::new("Todo List:".to_string()));

        controller.add("Buy Milk".to_string());
        controller.add("Buy eggs".to_string());

        scene.add(controller);

        Self { scene }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.scene.update(ctx);
        CentralPanel::default()
            .frame(Frame::none().fill(*theme::BACKGROUND)) // Use background color
            .show(ctx, |ui| {
                self.scene.draw(ui); // Ensure draw is correctly called here
            });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Todo List App", // Window title
        native_options,
        Box::new(|_cc| Box::new(MyApp::new())), // Proper closure format
    )
    .expect("Failed to start application");
}
