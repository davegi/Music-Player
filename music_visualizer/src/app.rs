use crate::controller::audio::MusicController;
use crate::view::gui::run_gui;

pub struct App {
    pub music_controller: MusicController,
}

impl App {
    pub fn new() -> Self {
        Self {
            music_controller: MusicController::new(),
        }
    }

    pub fn run(&mut self) {
        run_gui(&mut self.music_controller);
    }
}
