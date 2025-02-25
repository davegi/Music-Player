use crate::assets::constants::*;
use crate::ball::Ball;
use crate::paddle::Paddle;

use eframe::egui::Context;

pub struct Controller {
    pub ball: Ball,
    pub paddle1: Paddle,
    pub paddle2: Paddle,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            paddle1: Paddle::new(1),
            paddle2: Paddle::new(2),
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        self.paddle1.update(ctx);
        self.paddle2.update(ctx);
        self.ball.update(ctx);

        // Ball collision with left paddle
        if self.ball.get_position().x - self.ball.get_radius()
            <= self.paddle1.get_position().x + PADDLE_WIDTH
            && self.ball.get_position().y >= self.paddle1.get_position().y
            && self.ball.get_position().y <= self.paddle1.get_position().y + PADDLE_HEIGHT
        {
            self.ball.set_velocity_x(self.ball.get_velocity().x.abs()); // Bounce right
        }

        // Ball collision with right paddle
        if self.ball.get_position().x + self.ball.get_radius() >= self.paddle2.get_position().x
            && self.ball.get_position().y >= self.paddle2.get_position().y
            && self.ball.get_position().y <= self.paddle2.get_position().y + PADDLE_HEIGHT
        {
            self.ball.set_velocity_x(-self.ball.get_velocity().x.abs()); // Bounce left
        }

        // Ball out of bounds (reset)
        if self.ball.get_position().x < 0.0 || self.ball.get_position().x > WINDOW_WIDTH {
            self.ball = Ball::new(); // Reset ball position and velocity
        }
    }
}
