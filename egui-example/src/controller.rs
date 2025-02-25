// Import necessary modules from `egui`
use eframe::egui::{Context, Ui};

// Import constants and utility traits
use crate::assets::constants::*;
use crate::assets::egui_utils::{Drawable, Updatable};

// Import the `TodoList` struct
use crate::todo_list::TodoList;
use eframe::egui;

/// The `Controller` struct manages user interaction with the `TodoList`.
/// It handles adding items, managing input, and rendering UI elements.
pub struct Controller {
    todo_list: TodoList,   // The main to-do list
    adding_item: bool,     // Tracks if the user is currently adding an item
    new_item_text: String, // Stores text input for the new item
}

impl Controller {
    /// Creates a new `Controller` instance with an empty `TodoList`.
    pub fn new() -> Self {
        let todo_list = TodoList::new("Todo List:".to_string());

        Self {
            todo_list,
            adding_item: false,
            new_item_text: String::new(),
        }
    }

    /// Adds a new item to the `TodoList`.
    pub fn add(&mut self, text: String) {
        self.todo_list.add(text);
    }

    /// Displays the text input field and "Done" button when adding a new item.
    fn display_add_option(&mut self, ui: &mut Ui) {
        // Calculate centered position for the input field
        let input_x = (ui.available_width() - TOTAL_ADD_SIZE) / 2.0;
        let input_y = ui.max_rect().bottom() - 40.0; // Position near the bottom

        // Define the rectangle for the input field and button
        let rect = egui::Rect::from_min_size(
            egui::pos2(input_x, input_y),
            egui::vec2(ADD_SIZE.x, ADD_SIZE.y),
        );

        // Render the input field and "Done" button inside the allocated rectangle
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.horizontal(|ui| {
                // Text input field
                ui.add_sized(
                    INPUT_SIZE,
                    egui::TextEdit::singleline(&mut self.new_item_text),
                );

                // "Done" button: adds the item when clicked
                if ui
                    .add_sized(ADD_BUTTON_SIZE, egui::Button::new("Done"))
                    .clicked()
                {
                    if !self.new_item_text.trim().is_empty() {
                        self.add(self.new_item_text.clone()); // Add new item
                    }
                    self.new_item_text.clear(); // Clear input field
                    self.adding_item = false; // Exit adding mode
                }
            });
        });
    }

    /// Displays the "Add Item" button when not in adding mode.
    fn display_add_button(&mut self, ui: &mut Ui) {
        // Calculate centered position for the button
        let button_x = (ui.available_width() - ADD_BUTTON_SIZE.x) / 2.0;
        let button_y = ui.max_rect().bottom() - 40.0; // Position near the bottom

        // Define the rectangle for the button
        let button_rect = egui::Rect::from_min_size(
            egui::pos2(button_x, button_y),
            egui::vec2(ADD_BUTTON_SIZE.x, ADD_BUTTON_SIZE.y),
        );

        // Render the "Add Item" button inside the allocated rectangle
        ui.allocate_ui_at_rect(button_rect, |ui| {
            if ui
                .add_sized(ADD_BUTTON_SIZE, egui::Button::new("Add Item"))
                .clicked()
            {
                self.adding_item = true; // Switch to adding mode
            }
        });
    }
}

// Implement `Updatable` trait for `Controller` to handle updates
impl Updatable for Controller {
    fn update(&mut self, ctx: &Context) {
        self.todo_list.update(ctx); // Update the to-do list
    }
}

// Implement `Drawable` trait for `Controller` to render UI components
impl Drawable for Controller {
    fn draw(&mut self, ui: &mut Ui) {
        self.todo_list.draw(ui); // Draw the to-do list

        // Display either the input field or the "Add Item" button
        if self.adding_item {
            self.display_add_option(ui);
        } else {
            self.display_add_button(ui);
        }
    }
}
