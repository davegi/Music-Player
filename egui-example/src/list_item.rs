use crate::egui_utils::{Drawable, HelperFunctions, KeyCoords, Rectangle, TextLabel};
use eframe::egui::{Color32, Context, Pos2, Ui};

pub struct ListItem {
    id: i32,
    pos: Pos2,
    text: TextLabel,
    rect: Rectangle,
    done: bool,
}

impl ListItem {
    pub fn new(text: String, id: i32) -> Self {
        Self {
            id: id,
            pos: Pos2::new(0.0, 0.0),
            text: TextLabel::new(text, Pos2::new(0.0, 0.0)),
            rect: Rectangle::new(Pos2::new(0.0, 0.0), Pos2::new(100.0, 20.0), Color32::GRAY),
            done: false,
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        let x = KeyCoords::top_center_pos(ctx).x;
        let y = KeyCoords::top_center_pos(ctx).y
            + (HelperFunctions::get_percentage_of_screen_height(ctx, 0.1) * (self.id as f32 + 2.0));

        self.rect.set_position(Pos2::new(
            x - self.rect.get_width() / 2.0,
            y - self.rect.get_height() / 2.0,
        ));

        self.text.set_position(Pos2::new(x, y));
    }
}

impl Drawable for ListItem {
    fn draw(&self, ui: &mut Ui) {
        self.rect.draw(ui);
        self.text.draw(ui);
    }
}
