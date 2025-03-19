use macroquad::prelude::*;
use egui_macroquad::egui::{self, Button, Slider, Window};
use std::sync::{Arc, Mutex};

// Constants for screen size
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 600.0;

// A general trait for objects that can be updated each frame
trait Updatable {
    fn update(&mut self);
}

// A general trait for objects that can be drawn
trait Drawable {
    fn draw(&self);
}

// Structure for a moving square
struct Square {
    position: Vec2,
    size: f32,
    color: Color,
    speed: f32,
}

impl Square {
    fn new() -> Self {
        Self {
            position: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            size: 50.0,
            color: RED,
            speed: 5.0,
        }
    }
}

// Implement the Updatable trait for Square
impl Updatable for Square {
    fn update(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.position.x -= self.speed;
        }
        if is_key_down(KeyCode::Right) {
            self.position.x += self.speed;
        }
        if is_key_down(KeyCode::Up) {
            self.position.y -= self.speed;
        }
        if is_key_down(KeyCode::Down) {
            self.position.y += self.speed;
        }

        // Keep the square inside screen bounds
        let max_x = SCREEN_WIDTH - self.size;
        let max_y = SCREEN_HEIGHT - self.size;
        self.position.x = self.position.x.clamp(0.0, max_x);
        self.position.y = self.position.y.clamp(0.0, max_y);
    }
}

// Implement the Drawable trait for Square
impl Drawable for Square {
    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size, self.size, self.color);
    }
}

// UI Manager for handling Egui interactions
struct UIManager {
    square: Arc<Mutex<Square>>, // Shared mutable access to Square
}

impl UIManager {
    fn new(square: Arc<Mutex<Square>>) -> Self {
        Self { square }
    }
}

// Implement Updatable for UIManager
impl Updatable for UIManager {
    fn update(&mut self) {
        egui_macroquad::ui(|ctx| {
            Window::new("Control Panel")
                .fixed_pos([10.0, 10.0]) // Locked in top-left corner
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Move the square using arrow keys.");

                    // Lock and mutate the square safely
                    let mut square = self.square.lock().unwrap();

                    ui.add(Slider::new(&mut square.size, 10.0..=100.0).text("Size"));
                    ui.add(Slider::new(&mut square.speed, 1.0..=20.0).text("Speed"));

                    ui.label("Change Color:");
                    ui.horizontal(|ui| {
                        if ui.add(Button::new("Red")).clicked() {
                            square.color = RED;
                        }
                        if ui.add(Button::new("Green")).clicked() {
                            square.color = GREEN;
                        }
                        if ui.add(Button::new("Blue")).clicked() {
                            square.color = BLUE;
                        }
                    });
                });
        });
    }
}

// Main game struct to manage everything
struct Game {
    square: Arc<Mutex<Square>>,
    ui_manager: UIManager,
}

impl Game {
    fn new() -> Self {
        let square = Arc::new(Mutex::new(Square::new()));
        let ui_manager = UIManager::new(square.clone()); // Share square with UI

        Self { square, ui_manager }
    }

    async fn run(&mut self) {
        loop {
            clear_background(BLACK);

            // Update and draw square safely
            {
                let mut square = self.square.lock().unwrap();
                square.update();
                square.draw();
            }

            self.ui_manager.update();

            egui_macroquad::draw();
            next_frame().await;
        }
    }
}

#[macroquad::main("Egui & Macroquad App")]
async fn main() {
    // Ensure correct screen size
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut game = Game::new();
    game.run().await;
}