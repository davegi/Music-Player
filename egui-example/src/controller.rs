use eframe::egui::{Context, Ui};

use crate::egui_utils::{Drawable, Updatable};
use crate::todo_list::TodoList;

pub struct Controller {
    todo_list: TodoList,
}

impl Controller {
    pub fn new(todo_list: TodoList) -> Self {
        let mut todo_list = todo_list;

        Self { todo_list }
    }

    pub fn add(&mut self, text: String) {
        self.todo_list.add(text);
    }
}

impl Updatable for Controller {
    fn update(&mut self, ctx: &Context) {
        self.todo_list.update(ctx);
    }
}

impl Drawable for Controller {
    fn draw(&self, ui: &mut Ui) {
        self.todo_list.draw(ui);
    }
}
