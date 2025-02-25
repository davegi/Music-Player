use crate::assets::constants::*;
use crate::assets::egui_utils::*;

use eframe::egui::{Color32, Context, Pos2, Ui};

pub struct Board {
    rect: Rectangle,
}

impl Board {
    pub fn new() -> Self {
        Self {
            rect: Rectangle::new(
                Pos2::new(MID_LN_POS1.x, MID_LN_POS1.y),
                Pos2::new(MID_LN_POS2.x, MID_LN_POS2.y),
                Color32::WHITE,
            ),
        }
    }
}

impl Drawable for Board {
    fn draw(&mut self, ui: &mut Ui) {
        self.rect.draw(ui);
    }
}

impl Updatable for Board {
    fn update(&mut self, _ctx: &Context) {}
}
