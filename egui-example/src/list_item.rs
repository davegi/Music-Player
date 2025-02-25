// Import necessary modules and utilities
use crate::assets::constants::*;
use crate::assets::egui_utils::Drawable;
use crate::assets::helper_functions::HelperFunctions;
use crate::assets::key_coords::KeyCoords;

use eframe::egui::{Button, Color32, Context, Pos2, Rect, Ui, Vec2};

/// Represents an individual to-do list item with a checkbox and delete button.
pub struct ListItem {
    id: i32,             // Unique identifier for ordering the item in the list
    pos: Pos2,           // Determines the position of the item in the UI
    text: String,        // The text content of the to-do item
    done: bool,          // Whether the item is marked as completed
    pub to_remove: bool, // Flag indicating whether the item should be deleted
}

impl ListItem {
    /// Creates a new `ListItem` with a given text and ID.
    pub fn new(text: String, id: i32) -> Self {
        Self {
            id,
            pos: Pos2::new(0.0, 0.0), // Default position; updated later in `update()`
            text,
            done: false,      // Starts as not completed
            to_remove: false, // Default to not marked for removal
        }
    }

    /// Updates the position of the list item based on its index.
    pub fn update(&mut self, ctx: &Context) {
        let x = KeyCoords::top_center_pos(ctx).x; // Get center X position
        let y = KeyCoords::top_center_pos(ctx).y
            + (HelperFunctions::get_percentage_of_screen_height(ctx, 0.07)
                * (self.id as f32 + 1.5)); // Position each item below the previous one

        self.pos = Pos2::new(x, y); // Apply the calculated position
    }

    /// Sets a new ID for the item (used when items are reordered).
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }
}

// Implements the `Drawable` trait to render the list item in the UI.
impl Drawable for ListItem {
    fn draw(&mut self, ui: &mut Ui) {
        // Display checkmark if done, otherwise an empty box
        let button_text = if self.done {
            format!("✅ |  {}", self.text)
        } else {
            format!("⬜ |  {}", self.text)
        };

        // Create the main button that represents the to-do item
        let button = Button::new(button_text).fill(if self.done {
            Color32::DARK_GREEN // Green when marked as done
        } else {
            Color32::DARK_GRAY // Default gray color
        });

        // Define the position and size of the main todo button
        let button_rect = Rect::from_min_size(
            Pos2::new(
                self.pos.x - BUTTON_SIZE.x / 2.0, // Center horizontally
                self.pos.y - BUTTON_SIZE.y / 2.0, // Center vertically
            ),
            BUTTON_SIZE, // Standard button size
        );

        // Define the position and size of the delete button (❌) next to the main button
        let delete_rect = Rect::from_min_size(
            Pos2::new(
                button_rect.right() + 10.0,       // Offset slightly to the right
                self.pos.y - BUTTON_SIZE.y / 2.0, // Align with the main button
            ),
            Vec2::new(BUTTON_SIZE.y, BUTTON_SIZE.y), // Square delete button
        );

        // Draw the main todo button and toggle `done` state when clicked
        if ui.put(button_rect, button).clicked() {
            self.done = !self.done; // Toggle completion status
            ui.ctx().request_repaint(); // Request a UI refresh
        }

        // Draw the delete button and mark the item for removal when clicked
        if ui
            .put(delete_rect, Button::new("❌").fill(Color32::RED))
            .clicked()
        {
            self.to_remove = true; // Mark for deletion
            ui.ctx().request_repaint(); // Request a UI refresh
        }
    }
}
