use eframe::egui::Vec2;

/// **Window dimensions**
pub const WINDOW_WIDTH: f32 = 600.0; // Width of the application window
pub const WINDOW_HEIGHT: f32 = 600.0; // Height of the application window

pub const MID_LN_POS1: Vec2 = Vec2::new((WINDOW_WIDTH / 2.0) - 5.0, 0.0);
pub const MID_LN_POS2: Vec2 = Vec2::new((WINDOW_WIDTH / 2.0) + 5.0, WINDOW_HEIGHT);

pub const PADDLE_POS1: Vec2 = Vec2::new(20.0, (WINDOW_HEIGHT / 2.0) - 50.0);
pub const PADDLE_POS2: Vec2 = Vec2::new(WINDOW_WIDTH - 20.0, (WINDOW_HEIGHT / 2.0) - 50.0);

pub const PADDLE_WIDTH: f32 = 10.0; // Width of the paddle
pub const PADDLE_HEIGHT: f32 = 100.0; // Height of the paddle

pub const BALL_RADIUS: f32 = 10.0; // Radius of the ball
pub const BALL_SPEED: f32 = 3.0; // Speed of the ball
pub const BALL_START_POS: Vec2 = Vec2::new((WINDOW_WIDTH / 2.0) - 5.0, (WINDOW_HEIGHT / 2.0) - 5.0);
