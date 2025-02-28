mod app;
mod view;

use app::{App, View};
use view::macroquad_egui_view::MacroquadEguiView;
use view::pure_egui_view::PureEguiView;

use macroquad::prelude::*;
use once_cell::sync::Lazy;
use std::env;

// Default values
const DEFAULT_WIDTH: i32 = 1280;
const DEFAULT_HEIGHT: i32 = 720;
const DEFAULT_VIEW: &str = "macroquad-egui";

// Parse command-line arguments safely with defaults
fn parse_args() -> (i32, i32, String) {
    let args: Vec<String> = env::args().collect();

    let width = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_WIDTH);
    let height = args
        .get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_HEIGHT);
    let view_type = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_VIEW)
        .to_string();

    (width, height, view_type)
}

// Global window configuration
static WINDOW_CONF: Lazy<Conf> = Lazy::new(|| {
    let (width, height, _) = parse_args();
    Conf {
        window_title: "Rust Music Player".to_owned(),
        window_width: width,
        window_height: height,
        ..Default::default()
    }
});

// Function for macroquad to get the window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_CONF.window_title.clone(),
        window_width: WINDOW_CONF.window_width,
        window_height: WINDOW_CONF.window_height,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let (_, _, view_type) = parse_args();

    let mut app = match view_type.as_str() {
        "macroquad-egui" => App::new(Box::new(MacroquadEguiView::new())),
        "pure-egui" => App::new(Box::new(PureEguiView::new())),
        _ => panic!("Invalid command line argument"),
    };

    app.setup();

    println!(
        "Starting Rust Music Player with resolution: {}x{} (View: {})",
        WINDOW_CONF.window_width, WINDOW_CONF.window_height, view_type
    );

    // Proper async loop for Macroquad
    loop {
        app.run();
        next_frame().await;
    }
}
