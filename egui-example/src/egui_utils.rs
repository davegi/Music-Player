use eframe::egui;
use eframe::egui::{Color32, Painter, Pos2, Rect, Ui, Vec2};

/// A collection of helper functions for drawing in `egui`.
pub struct EguiUtils;

impl EguiUtils {
    /// Draws text at a specific (x, y) coordinate.
    pub fn draw_text(ui: &Ui, painter: &Painter, text: &str, pos: Pos2, color: Color32) {
        let font_id = egui::TextStyle::Heading.resolve(ui.style());
        painter.text(pos, egui::Align2::LEFT_TOP, text, font_id, color);
    }

    /// Draws a rectangle at a specific position with given width, height, and color.
    pub fn draw_rect(painter: &Painter, pos: Pos2, size: Vec2, color: Color32) {
        let rect = Rect::from_min_size(pos, size);
        painter.rect_filled(rect, 5.0, color);
    }

    /// Draws a circle at a given position with a specified radius and color.
    pub fn draw_circle(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
        painter.circle_filled(center, radius, color);
    }

    /// Draws a line from `start` to `end` with a specified thickness and color.
    pub fn draw_line(painter: &Painter, start: Pos2, end: Pos2, thickness: f32, color: Color32) {
        painter.line_segment([start, end], egui::Stroke::new(thickness, color));
    }
}
