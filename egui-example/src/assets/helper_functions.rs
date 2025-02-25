use eframe::egui::Context;

/// Provides utility functions for screen-related calculations.
pub struct HelperFunctions;

impl HelperFunctions {
    /// Calculates a percentage of the screen height.
    pub fn get_percentage_of_screen_height(ctx: &Context, percentage: f32) -> f32 {
        let screen_size = ctx.screen_rect().size();
        screen_size.y * percentage
    }

    // Calculates a percentage of the screen width. (commented out to avoid warnings)
    // pub fn get_percentage_of_screen_width(ctx: &Context, percentage: f32) -> f32 {
    //     let screen_size = ctx.screen_rect().size();
    //     screen_size.x * percentage
    // }
}
