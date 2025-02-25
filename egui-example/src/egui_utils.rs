use eframe::egui::{Align2, Color32, Context, FontId, Pos2, Rect, Ui};

/// Trait for drawable objects in egui.
pub trait Drawable {
    fn draw(&self, ui: &mut Ui);
}

/// A text label with a position.
pub struct TextLabel {
    text: String,
    position: Pos2,
}

impl TextLabel {
    pub fn new(text: impl Into<String>, position: Pos2) -> Self {
        Self {
            text: text.into(),
            position,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_position(&mut self, position: Pos2) {
        self.position = position;
    }
}

impl Drawable for TextLabel {
    fn draw(&self, ui: &mut Ui) {
        ui.painter().text(
            self.position,
            Align2::CENTER_CENTER,
            &self.text,
            FontId::default(),
            Color32::WHITE,
        );
    }
}

/// A rectangle with a position and color.
pub struct Rectangle {
    top_left: Pos2,
    width: f32,
    height: f32,
    color: Color32,
}

impl Rectangle {
    pub fn new(top_left: Pos2, bottom_right: Pos2, color: Color32) -> Self {
        Self {
            top_left,
            width: bottom_right.x - top_left.x,
            height: bottom_right.y - top_left.y,
            color,
        }
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn set_position(&mut self, position: Pos2) {
        self.top_left = position;
    }
}

impl Drawable for Rectangle {
    fn draw(&self, ui: &mut Ui) {
        let bottom_right = Pos2::new(self.top_left.x + self.width, self.top_left.y + self.height);
        let rect = Rect::from_two_pos(self.top_left, bottom_right);
        ui.painter().rect_filled(rect, 0.0, self.color);
    }
}

pub trait Updatable {
    fn update(&mut self, ctx: &Context);
}

/// A collection of drawable objects.

trait DrawableAndUpdatable: Drawable + Updatable {}
impl<T: Drawable + Updatable> DrawableAndUpdatable for T {}

pub struct Scene {
    objects: Vec<Box<dyn DrawableAndUpdatable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, obj: impl Drawable + Updatable + 'static) {
        self.objects.push(Box::new(obj));
    }

    pub fn remove(&mut self, index: usize) {
        self.objects.remove(index);
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Drawable for Scene {
    fn draw(&self, ui: &mut Ui) {
        for obj in &self.objects {
            obj.draw(ui);
        }
    }
}

impl Updatable for Scene {
    fn update(&mut self, ctx: &Context) {
        for obj in &mut self.objects {
            obj.update(ctx);
        }
    }
}

pub struct KeyCoords;

impl KeyCoords {
    pub fn top_left_pos(ctx: &Context) -> Pos2 {
        Pos2::new(0.0, 0.0)
    }
    pub fn top_right_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(screen_size.x, 0.0)
    }

    pub fn bottom_left_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(0.0, screen_size.y)
    }

    pub fn bottom_right_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(screen_size.x, screen_size.y)
    }

    pub fn middle_left_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(0.0, screen_size.y / 2.0)
    }

    pub fn middle_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(screen_size.x / 2.0, screen_size.y / 2.0)
    }

    pub fn middle_right_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(screen_size.x, screen_size.y / 2.0)
    }

    pub fn top_center_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(screen_size.x / 2.0, 0.0)
    }

    pub fn bottom_center_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(screen_size.x / 2.0, screen_size.y)
    }
}

pub struct HelperFunctions;

impl HelperFunctions {
    pub fn get_percentage_of_screen_width(ctx: &Context, percentage: f32) -> f32 {
        let screen_size = ctx.screen_rect().size();
        screen_size.x * percentage
    }

    pub fn get_percentage_of_screen_height(ctx: &Context, percentage: f32) -> f32 {
        let screen_size = ctx.screen_rect().size();
        screen_size.y * percentage
    }
}
