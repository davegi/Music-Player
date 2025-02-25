use crate::egui_utils::{Drawable, HelperFunctions, KeyCoords, TextLabel, Updatable};
use crate::list_item::ListItem;
use eframe::egui::{Context, Pos2, Ui};

pub struct TodoList {
    todos: Vec<ListItem>,
    name: TextLabel,
}

impl TodoList {
    pub fn new(name: String) -> Self {
        Self {
            todos: vec![],
            name: TextLabel::new(name, Pos2::new(0.0, 0.0)),
        }
    }

    pub fn add(&mut self, text: String) {
        let id = self.todos.len() as i32;
        self.todos.push(ListItem::new(text, id));
    }

    pub fn remove(&mut self, index: usize) {
        self.todos.remove(index);
    }
}

impl Drawable for TodoList {
    fn draw(&self, ui: &mut Ui) {
        self.name.draw(ui);
        for item in self.todos.iter() {
            item.draw(ui);
        }
    }
}

impl Updatable for TodoList {
    fn update(&mut self, ctx: &Context) {
        //List Name
        let mut pos = KeyCoords::top_center_pos(ctx);
        pos.y += HelperFunctions::get_percentage_of_screen_height(ctx, 0.10);
        self.name.set_position(pos);

        for item in self.todos.iter_mut() {
            item.update(ctx);
        }
    }
}
