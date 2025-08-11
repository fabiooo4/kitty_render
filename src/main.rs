mod model;
mod screen;
mod vector;

use model::{Model, load_obj};
use nix::libc::EXIT_SUCCESS;
use screen::{Color, Screen};
use std::io;
use std::io::Write;
use std::process::exit;
use termion::event::Key;
use termion::raw::RawTerminal;
use termion::{input::TermRead, raw::IntoRawMode};
use vector::transform::Transform;

fn main() {
    // Terminal setup -------------------
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    stdout.activate_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    write!(
        stdout,
        "{}{}{}",
        // Clear the screen.
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
    // Terminal setup -------------------

    // Init -----------------------------
    let mut screen = Screen::new(256, 256);
    screen.scale(4);

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
    loop {
        handle_input(&mut stdin, &mut stdout, &mut screen, &mut transform);

        screen.render(&cube, &transform);

        screen.draw();
    }
    // Loop -----------------------------
}

fn handle_input(
    stdin: &mut termion::input::Keys<termion::AsyncReader>,
    stdout: &mut RawTerminal<io::Stdout>,
    screen: &mut Screen,
    transform: &mut Transform,
) {
    // Read input (if any)
    let input = stdin.next();

    // If a key was pressed
    if let Some(Ok(key)) = input {
        match key {
            // Exit if 'CTRL+C' is pressed
            Key::Ctrl('c') | Key::Esc => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Show,
                    termion::clear::AfterCursor
                )
                .unwrap();

                stdout.suspend_raw_mode().unwrap();
                stdout.flush().unwrap();

                screen.delete_all_images();
                exit(EXIT_SUCCESS);
            }

            Key::Char('w') => transform.pitch += 0.05,
            Key::Char('s') => transform.pitch -= 0.05,
            Key::Char('a') => transform.yaw += 0.05,
            Key::Char('d') => transform.yaw -= 0.05,
            _ => {}
        }
    }
}
