use eframe::egui::Vec2;

/// **Window dimensions**
pub const WINDOW_WIDTH: f32 = 600.0; // Width of the application window
pub const WINDOW_HEIGHT: f32 = 800.0; // Height of the application window

/// **General button sizes**
pub const BUTTON_SIZE: Vec2 = Vec2::new(200.0, 40.0); // Standard button size used for list items

/// **Input and Add Button dimensions**
pub const INPUT_SIZE: Vec2 = Vec2::new(200.0, 30.0); // Text input field size
pub const ADD_BUTTON_SIZE: Vec2 = Vec2::new(80.0, 30.0); // "Add Item" and "Done" button size

/// **Combined size for the input field and add button**
pub const TOTAL_ADD_SIZE: f32 = INPUT_SIZE.x + ADD_BUTTON_SIZE.x + 10.0; // Includes a 10px gap

/// **Total input area dimensions**
pub const ADD_SIZE: Vec2 = Vec2::new(TOTAL_ADD_SIZE, 30.0); // Ensures spacing consistency
