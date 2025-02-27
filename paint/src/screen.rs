use egui_macroquad::egui::Color32;
use macroquad::prelude::*;

/// Converts an `egui::Color32` to a `macroquad::Color`.
pub fn color32_to_color(c: Color32) -> Color {
    Color::new(
        c.r() as f32 / 255.0,
        c.g() as f32 / 255.0,
        c.b() as f32 / 255.0,
        c.a() as f32 / 255.0,
    )
}

/// Represents a single brush stroke on the screen.
#[derive(Clone, Copy)]
pub struct Stroke {
    pub pos: Vec2,
    pub size: f32,
    pub color: Color32,
    pub connected: bool, // If true, connects to the previous stroke
}

/// Handles the drawing history and rendering of strokes.
pub struct Screen {
    pub history: Vec<Stroke>,
    pub last_was_connected: bool,
}

impl Screen {
    /// Creates a new `Screen` instance.
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            last_was_connected: false,
        }
    }

    /// Adds a new stroke to the history.
    pub fn add_stroke(&mut self, pos: Vec2, size: f32, color: Color32, is_connected: bool) {
        self.history.push(Stroke {
            pos,
            size,
            color,
            connected: is_connected,
        });

        self.last_was_connected = is_connected;
    }

    /// Clears all strokes from the screen.
    pub fn clear(&mut self) {
        self.history.clear();
        self.last_was_connected = false;
    }

    /// Draws all recorded strokes on the screen.
    pub fn draw(&self) {
        for (i, stroke) in self.history.iter().enumerate() {
            // Draw a line connecting to the previous stroke if applicable
            if stroke.connected && i > 0 {
                let prev = self.history[i - 1];

                let distance = stroke.pos.distance(prev.pos);
                let steps = (distance / (stroke.size * 0.3)).ceil() as i32;

                for j in 0..=steps {
                    let t = j as f32 / steps as f32;
                    let interpolated_pos = prev.pos.lerp(stroke.pos, t);

                    draw_circle(
                        interpolated_pos.x,
                        interpolated_pos.y,
                        stroke.size * 0.5, // Adjust to ensure full coverage
                        color32_to_color(stroke.color),
                    );
                }
            }

            // Ensure no gaps by drawing a circle at the stroke position
            draw_circle(
                stroke.pos.x,
                stroke.pos.y,
                stroke.size * 0.5,
                color32_to_color(stroke.color),
            );
        }
    }
}
