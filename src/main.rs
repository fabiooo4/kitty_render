mod screen;

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

    clearscreen::clear().unwrap();
    while running.load(Ordering::SeqCst) {
        let screen = Screen::new_fullscreen();

        let mut img_buf: Vec<Vec<Color>> =
            vec![vec![Color::try_from("#000000").unwrap(); screen.width]; screen.height];

        (0..screen.height).for_each(|y| {
            (0..screen.width).for_each(|x| {
                let red: f32 = x as f32 / (screen.width as f32 - 1.);
                let green: f32 = y as f32 / (screen.height as f32 - 1.);
                img_buf[y][x] = Color::new((red * 255.) as u8, (green * 255.) as u8, 0, 0xFF);
            });
        });

        screen.render(&img_buf);

        sleep(Duration::from_millis(100));
    }
    clearscreen::clear().unwrap();
}
