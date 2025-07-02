mod model;
mod screen;
mod vector;

use kitty_image::{
    Action, ActionAnimationFrameControl, ActionAnimationFrameLoading, ActionPut,
    ActionTransmission, Command, Format, Frame, LoopMode, Medium, WrappedCommand,
};
use model::{Model, load_obj};
use screen::{Color, Screen};
use std::{
    borrow::Cow,
    io::{stdin, stdout},
    num::NonZero,
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

    let mut screen = Screen::new(128, 128);
    // screen.scale(8);

    // Load cube model
    let model_points = load_obj("models/cube.obj").expect("Failed to read model data");

    // Assign a random color to each triangle
    let triangle_colors: Vec<Color> = (0..model_points.windows(3).count())
        .map(|_| Color::random())
        .collect();

    let cube = Model::new(model_points, triangle_colors);

    let mut transform = Transform::default();

    // Init -----------------------------

    // Loop -----------------------------
    /* clearscreen::clear().unwrap();
    while running.load(Ordering::SeqCst) {
        screen.render(&cube, &transform);

        screen.draw();

        transform.yaw -= 0.05;
        transform.pitch -= 0.03;
    }
    sleep(Duration::from_millis(50));
    screen.clear();
    clearscreen::clear().unwrap(); */
    // Loop -----------------------------

    clearscreen::clear().unwrap();

    // --- First frame ---
    screen.render(&cube, &transform);
    screen.draw(true);

    // Display frame 1 and wait for more frames
    let action = Action::AnimationFrameControl(ActionAnimationFrameControl {
        mode: kitty_image::AnimationMode::RunWithNewFrames,
        frame_number: Some(Frame(NonZero::new(1).unwrap())),
        gap: 1,
        ..Default::default()
    });
    screen.action = action;
    screen.draw(false); // no payload

    // --- Subsequent frames ---
    let mut line = String::new();
    println!("Press Enter to render the next frame...");
    while stdin().read_line(&mut line).unwrap() > 0 {
        if line.trim().is_empty() {
            transform.yaw -= 0.15;
            transform.pitch -= 0.13;
            screen.render(&cube, &transform);

            // Transmit frame N
            // TODO: Add width and height to AnimationFrameLoading and fix Overwrite
            let action = Action::AnimationFrameLoading(ActionAnimationFrameLoading {
                composition_mode: kitty_image::CompositionMode::Overwrite,
                frame_number: Some(Frame(NonZero::new(1).unwrap())),
                gap: 1,
                ..Default::default()
            });
            screen.action = action;
            screen.draw(true); // send payload

            line.clear();
            println!("Press Enter to render the next frame...");
        } else {
            break;
        }
    }
}
