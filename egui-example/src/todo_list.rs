// Import necessary modules and utilities
use crate::assets::constants::*;
use crate::assets::egui_utils::{Drawable, TextLabel, Updatable};
use crate::assets::helper_functions::HelperFunctions;
use crate::assets::key_coords::KeyCoords;

use crate::list_item::ListItem;
use eframe::egui::{Context, Pos2, Ui};

/// The `TodoList` struct represents a simple to-do list.
/// It contains a list of `ListItem`s and a title label.
pub struct TodoList {
    todos: Vec<ListItem>, // List of to-do items
    name: TextLabel,      // Title of the to-do list
}

impl TodoList {
    /// Creates a new `TodoList` with a given name.
    pub fn new(name: String) -> Self {
        Self {
            todos: vec![], // Initialize with an empty list
            name: TextLabel::new_with_size(name, Pos2::new(0.0, 0.0), 24.0), // Set font size to 24 for better visibility
        }
    }

    /// Adds a new item to the to-do list.
    /// Ensures the list does not exceed the allowed item count based on screen height.
    pub fn add(&mut self, text: String) {
        if self.todos.len()
            < ((WINDOW_HEIGHT / (BUTTON_SIZE.y * 1.65)) as i32)
                .try_into()
                .unwrap()
        {
            let id = self.todos.len() as i32; // Assign an ID based on current list length
            self.todos.push(ListItem::new(text, id)); // Add new item to the list
        }
    }
}

// Implement the `Drawable` trait to handle rendering the to-do list
impl Drawable for TodoList {
    fn draw(&mut self, ui: &mut Ui) {
        self.name.draw(ui); // Draw the title
        for item in self.todos.iter_mut() {
            item.draw(ui); // Draw each list item
        }
    }
}

// Implement the `Updatable` trait to handle updates and interactions
impl Updatable for TodoList {
    fn update(&mut self, ctx: &Context) {
        // Remove items marked for deletion
        self.todos.retain(|item| !item.to_remove);

        // Renumber the IDs to keep them sequential after removing items
        for (new_id, item) in self.todos.iter_mut().enumerate() {
            item.set_id(new_id as i32);
        }

        // Position the list title at the top center of the screen
        let mut pos = KeyCoords::top_center_pos(ctx);
        pos.y += HelperFunctions::get_percentage_of_screen_height(ctx, 0.05);
        self.name.set_position(pos);

        // Update each list item
        for item in self.todos.iter_mut() {
            item.update(ctx);
        }
    }
}
