use crate::assets::constants::*;
use crate::assets::egui_utils::*;
use eframe::egui::{Color32, Context, Key, Pos2, Ui};

const PADDLE_SPEED: f32 = 5.0; // Speed of paddle movement

pub struct Paddle {
    pub position: Pos2,
    id: i32,
}

impl Paddle {
    pub fn new(id: i32) -> Self {
        Self {
            position: match id {
                1 => Pos2::new(PADDLE_POS1.x, PADDLE_POS1.y),
                2 => Pos2::new(PADDLE_POS2.x, PADDLE_POS2.y),
                _ => panic!("Invalid paddle ID"),
            },
            id,
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        ctx.input(|i| {
            match self.id {
                1 => {
                    if i.key_down(Key::W) {
                        self.move_paddle(-PADDLE_SPEED); // Move up
                    }
                    if i.key_down(Key::S) {
                        self.move_paddle(PADDLE_SPEED); // Move down
                    }
                }
                2 => {
                    if i.key_down(Key::ArrowUp) {
                        self.move_paddle(-PADDLE_SPEED); // Move up
                    }
                    if i.key_down(Key::ArrowDown) {
                        self.move_paddle(PADDLE_SPEED); // Move down
                    }
                }
                _ => {}
            }
        });
    }

    /// Moves the paddle up or down within the screen bounds.
    fn move_paddle(&mut self, delta_y: f32) {
        self.position.y = (self.position.y + delta_y).clamp(0.0, WINDOW_HEIGHT - PADDLE_HEIGHT);
        // Keep within bounds
    }

    pub fn get_position(&self) -> Pos2 {
        self.position
    }
}

impl Drawable for Paddle {
    fn draw(&mut self, ui: &mut Ui) {
        let mut rect = Rectangle::new(
            self.position,
            Pos2::new(
                self.position.x + PADDLE_WIDTH,
                self.position.y + PADDLE_HEIGHT,
            ),
            Color32::WHITE,
        );
        rect.draw(ui);
    }
}
