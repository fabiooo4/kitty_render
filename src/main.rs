mod model;
mod screen;
mod vector;

use model::{Model, load_obj};
use screen::{Color, Screen};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};
use vector::transform::Transform;

fn main() {
    // Setup CTRL + C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Init -----------------------------

    let mut screen = Screen::new(2 * 64, 2 * 64);
    screen.scale(10);

    // Load cube model
    let model_points = load_obj("models/cube.obj").expect("Failed to read model data");

    // Assign a random color to each triangle
    let triangle_colors: Vec<Color> = (0..model_points.windows(3).count())
        .map(|_| Color::random())
        .collect();

    let cube = Model::new(model_points, triangle_colors);

    let mut transform = Transform::new(0.);

    // Init -----------------------------

    // Loop -----------------------------
    clearscreen::clear().unwrap();
    while running.load(Ordering::SeqCst) {
        screen.render(&cube, &transform);

        screen.draw();

        transform.yaw -= 0.2;
    }
    clearscreen::clear().unwrap();
    // Loop -----------------------------
}
