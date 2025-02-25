use eframe::egui::{Context, Pos2};

/// Provides predefined screen positions for UI elements.
pub struct KeyCoords;

impl KeyCoords {
    /// Returns the position at the top center of the screen.
    pub fn top_center_pos(ctx: &Context) -> Pos2 {
        let screen_size = ctx.screen_rect().size();
        Pos2::new(screen_size.x / 2.0, 0.0)
    }

    // Additional possible predefined positions (commented out for future use)
    // pub fn top_left_pos(_ctx: &Context) -> Pos2 { Pos2::new(0.0, 0.0) }
    // pub fn top_right_pos(ctx: &Context) -> Pos2 { ... }
    // pub fn bottom_left_pos(ctx: &Context) -> Pos2 { ... }
    // pub fn bottom_right_pos(ctx: &Context) -> Pos2 { ... }
    // pub fn middle_left_pos(ctx: &Context) -> Pos2 { ... }
    // pub fn middle_pos(ctx: &Context) -> Pos2 { ... }
    // pub fn middle_right_pos(ctx: &Context) -> Pos2 { ... }
    // pub fn bottom_center_pos(ctx: &Context) -> Pos2 { ... }
}
