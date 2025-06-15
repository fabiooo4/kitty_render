mod model;
mod screen;
mod utils;

use model::{load_obj, Model};
use screen::{Color, Screen};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};

fn main() {
    // Setup CTRL + C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut screen = Screen::new(64, 64);

    clearscreen::clear().unwrap();
    while running.load(Ordering::SeqCst) {
        // Load cube model
        let cube_points = load_obj("models/cube.obj");

        // Assign a random color to each triangle
        let triangle_colors: Vec<Color> = vec![Color::random(); cube_points.len() / 3];

        let cube = Model::new(cube_points, triangle_colors);

        screen.render();

        sleep(Duration::from_millis(100));
    }
    clearscreen::clear().unwrap();
}
