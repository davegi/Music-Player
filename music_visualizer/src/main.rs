mod app;
mod controller;
mod model;
mod view;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
