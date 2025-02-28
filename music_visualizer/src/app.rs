pub trait View {
    fn setup(&mut self);

    fn run(&mut self);
}

pub struct App {
    pub view: Box<dyn View>, // Change from generic V to Box<dyn View>
}

impl App {
    pub fn new(view: Box<dyn View>) -> Self {
        Self { view }
    }

    pub fn run(&mut self) {
        self.view.run();
    }

    pub fn setup(&mut self) {
        self.view.setup();
    }
}
