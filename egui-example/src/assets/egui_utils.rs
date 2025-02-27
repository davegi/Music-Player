use eframe::egui::{Context, Pos2, Ui};

/// Defines a trait for objects that can be drawn on the screen.
pub trait Drawable {
    fn draw(&mut self, ui: &mut Ui);
}

/// Defines a trait for objects that require updates each frame.
pub trait Updatable {
    fn update(&mut self, ctx: &Context);
}

/// Combines both drawable and updatable functionality into one trait.
trait DrawableAndUpdatable: Drawable + Updatable {}

/// Implements the combined trait for all types that satisfy both Drawable and Updatable.
impl<T: Drawable + Updatable> DrawableAndUpdatable for T {}

/// Represents a simple text label with a position and size.
pub struct TextLabel {
    text: String,
    position: Pos2,
    size: f32,
}

impl TextLabel {
    /// Creates a new text label with a specified size.
    pub fn new_with_size(text: String, position: Pos2, size: f32) -> Self {
        Self {
            text,
            position,
            size,
        }
    }

    /// Updates the position of the text label.
    pub fn set_position(&mut self, new_position: Pos2) {
        self.position = new_position;
    }

    /// Draws the text label on the screen.
    pub fn draw(&self, ui: &mut Ui) {
        ui.painter().text(
            self.position,
            eframe::egui::Align2::CENTER_CENTER,
            &self.text,
            eframe::egui::FontId::new(self.size, eframe::egui::FontFamily::Proportional),
            eframe::egui::Color32::WHITE,
        );
    }
}

/// Manages a collection of drawable and updatable objects.
///TODO: Split into two lists, one Drawable one Updatable
pub struct Scene {
    objects: Vec<Box<dyn DrawableAndUpdatable>>,
}

impl Scene {
    /// Creates an empty collection of objects.
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    /// Adds a new object to the collection.
    pub fn add(&mut self, obj: impl Drawable + Updatable + 'static) {
        self.objects.push(Box::new(obj));
    }
}

impl Drawable for Scene {
    /// Draws all objects within the collection.
    fn draw(&mut self, ui: &mut Ui) {
        for obj in &mut self.objects {
            obj.draw(ui);
        }
    }
}

impl Updatable for Scene {
    /// Updates all objects within the collection.
    fn update(&mut self, ctx: &Context) {
        for obj in &mut self.objects {
            obj.update(ctx);
        }
    }
}

// Example of an object with position and color attributes.
/*
pub struct Rectangle {
    top_left: Pos2,
    width: f32,
    height: f32,
    color: Color32,
}

impl Rectangle {
    /// Creates a new rectangle from two positions.
    pub fn new(top_left: Pos2, bottom_right: Pos2, color: Color32) -> Self {
        Self {
            top_left,
            width: bottom_right.x - top_left.x,
            height: bottom_right.y - top_left.y,
            color,
        }
    }

    /// Returns the width of the rectangle.
    pub fn get_width(&self) -> f32 {
        self.width
    }

    /// Returns the height of the rectangle.
    pub fn get_height(&self) -> f32 {
        self.height
    }

    /// Sets the position of the rectangle.
    pub fn set_position(&mut self, position: Pos2) {
        self.top_left = position;
    }
}

impl Drawable for Rectangle {
    /// Draws the rectangle on the screen.
    fn draw(&mut self, ui: &mut Ui) {
        let bottom_right = Pos2::new(self.top_left.x + self.width, self.top_left.y + self.height);
        let rect = Rect::from_two_pos(self.top_left, bottom_right);
        ui.painter().rect_filled(rect, 0.0, self.color);
    }
}
*/
