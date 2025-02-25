use crate::assets::constants::*;
use crate::assets::egui_utils::*;
use eframe::egui::{Color32, Context, Pos2, Ui, Vec2};

pub struct Ball {
    position: Pos2,
    velocity: Vec2,
    radius: f32,
    color: Color32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: Pos2::new(BALL_START_POS.x, BALL_START_POS.y),
            velocity: Vec2::new(BALL_SPEED, BALL_SPEED),
            radius: BALL_RADIUS,
            color: Color32::WHITE,
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        self.position += self.velocity;

        if self.position.y - self.radius <= 0.0 || self.position.y + self.radius >= WINDOW_HEIGHT {
            self.velocity.y = -self.velocity.y;
        }
    }

    pub fn get_position(&self) -> Pos2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn set_velocity_x(&mut self, x: f32) {
        self.velocity.x = x;
    }
}

impl Drawable for Ball {
    fn draw(&mut self, ui: &mut Ui) {
        let mut circle = Circle::new(self.position, self.radius, self.color);
        circle.draw(ui);
    }
}
