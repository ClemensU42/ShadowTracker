use fltk::{window::Window, app::App, prelude::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 100, 100, "ShadowTracker");
    wind.end();
    wind.show();
    app.run().unwrap(); 
}
