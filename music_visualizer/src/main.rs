mod app;
mod view;

use app::{App, View};
use view::macroquad_egui_view::MacroquadEguiView;
use view::pure_egui_view::PureEguiView;

use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use std::env;

fn window_conf() -> Conf {
    Conf {
        window_title: "".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Read command line arguments safely
    let args: Vec<String> = env::args().collect();
    let view_type = args.get(1).map(|s| s.as_str()).unwrap_or("macroquad-egui");

    let mut app = match view_type {
        "macroquad-egui" => App::new(Box::new(MacroquadEguiView::new())),
        "pure-egui" => App::new(Box::new(PureEguiView::new())),
        _ => panic!("Invalid command line argument"),
    };

    app.setup();

    println!("Starting Rust Music Player...");

    // Proper async loop for Macroquad
    loop {
        app.run(); // Now correctly calling instance method
        next_frame().await; // Yield back to Macroquad's async event loop
    }
}
