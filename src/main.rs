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

use crate::vector::vector3::Vector3;

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
    let mut screen = Screen::new(512, 512);
    screen.scale(2);
    // let mut screen = Screen::new_fullscreen();

    // Load cube model
    let monkey_points = load_obj("models/monkey.obj").expect("Failed to read model data");
    let cube_points = load_obj("models/cube.obj").expect("Failed to read model data");

    // Assign a random color to each triangle
    let triangle_colors: Vec<Color> = (0..monkey_points.windows(3).count())
        .map(|_| Color::random())
        .collect();

    let monkey = Model::new(monkey_points, triangle_colors.clone());
    let cube = Model::new(cube_points, triangle_colors);

    let mut transform = Transform::default();
    transform.position = Vector3::new(0., 0., -2.5);
    // Init -----------------------------

    // Loop -----------------------------
    loop {
        handle_input(&mut stdin, &mut stdout, &mut screen, &mut transform);

        screen.render(&monkey, &transform);
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

            // Position
            Key::Char('w') => transform.position += Vector3::new(0., 0., 0.1),
            Key::Char('W') => transform.position += Vector3::new(0., 0., 0.5),
            Key::Char('s') => transform.position += Vector3::new(0., 0., -0.1),
            Key::Char('S') => transform.position += Vector3::new(0., 0., -0.5),
            Key::Char('a') => transform.position += Vector3::new(-0.1, 0., 0.),
            Key::Char('A') => transform.position += Vector3::new(-0.5, 0., 0.),
            Key::Char('d') => transform.position += Vector3::new(0.1, 0., 0.),
            Key::Char('D') => transform.position += Vector3::new(0.5, 0., 0.),

            // Rotation
            Key::Left => transform.yaw += 0.1,
            Key::Right => transform.yaw -= 0.1,
            Key::Up => transform.pitch += 0.1,
            Key::Down => transform.pitch -= 0.1,

            // Fov
            Key::Char('q') => screen.fov -= 0.01,
            Key::Char('e') => screen.fov += 0.01,
            Key::Char('Q') => screen.fov -= 0.05,
            Key::Char('E') => screen.fov += 0.05,

            // Reset transformation
            Key::Char('r') => {
                transform.yaw = 0.0;
                transform.pitch = 0.0;
                transform.position = Vector3::default();
            }
            _ => {}
        }
    }
}
