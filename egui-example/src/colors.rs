use eframe::egui::Color32;
use once_cell::sync::Lazy;

// Define global colors as pub static
pub static BLACK: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(30, 30, 30));
pub static WHITE: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(255, 255, 255));
pub static RED: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(255, 0, 0));
pub static GREEN: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0, 255, 0));
pub static BLUE: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0, 0, 255));
pub static GRAY: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(128, 128, 128));

// Theme Colors
pub mod theme {
    use super::*;

    pub static BACKGROUND: Lazy<Color32> = Lazy::new(|| BLACK.clone());
    pub static PRIMARY: Lazy<Color32> = Lazy::new(|| WHITE.clone());
}
