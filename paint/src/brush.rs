use crate::screen::Screen;
use egui_macroquad::egui::Color32;
use macroquad::prelude::*;

/// Represents a brush tool for drawing on the canvas.
pub struct Brush {
    pub color: Color32,
    pub size: f32,
    pub pos: Vec2,
    pub drawing: bool,
}

impl Brush {
    /// Creates a new `Brush` with default values.
    pub fn new() -> Self {
        Self {
            color: Color32::BLACK,
            size: 10.0,
            pos: Vec2::ZERO,
            drawing: false,
        }
    }

    /// Updates the brush position and state based on user input.
    pub fn update(&mut self, screen: &mut Screen, mouse_pos: Vec2, color: Color32, size: f32) {
        self.pos = mouse_pos;
        self.color = color;
        self.size = size;

        // Start drawing on mouse press
        if is_mouse_button_pressed(MouseButton::Left) {
            self.drawing = true;
            screen.add_stroke(mouse_pos, self.size, self.color, false);
        }

        // Continue drawing while the left mouse button is held
        if self.drawing && is_mouse_button_down(MouseButton::Left) {
            screen.add_stroke(mouse_pos, self.size, self.color, true);
        }

        // Stop drawing when the mouse button is released
        if is_mouse_button_released(MouseButton::Left) {
            self.drawing = false;
        }
    }
}
