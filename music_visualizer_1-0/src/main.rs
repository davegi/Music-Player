mod controller;
mod menu;
mod view;

use macroquad::prelude::*;

use controller::Controller;

#[macroquad::main("Music Visualizer - 1.0")]
async fn main() {
    //TODO: Make screen size command line argument
    // Ensure correct screen size
    const SCREEN_WIDTH: f32 = 600.0;
    const SCREEN_HEIGHT: f32 = 600.0;
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut controller = Controller::new();
    controller.run().await;
}
