use eframe::egui::Color32;
use once_cell::sync::Lazy;

/// Defines commonly used colors in the application.
pub static BLACK: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(30, 30, 30));

/// Theme-related color definitions.
pub mod theme {
    use super::*;

    /// Background color used throughout the UI.
    pub static BACKGROUND: Lazy<Color32> = Lazy::new(|| BLACK.clone());
}
