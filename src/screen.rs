use std::{
    borrow::Cow,
    io::{Write, stdout},
    num::NonZero,
};

use kitty_image::{
    Action, ActionDelete, ActionTransmission, Command, DeleteTarget, Format, ID, Medium, Quietness,
    WrappedCommand,
};
use nix::{
    ioctl_read_bad,
    libc::{self, winsize},
};
use rand::Rng;

use crate::{
    model::Model,
    vector::{transform::Transform, vector2::Vector2, vector3::Vector3},
};

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub size: Vector2<f64>,
    scale: usize,
    frame_buf: Vec<Vec<Color>>,
    frame: u32,

    action: Action,
    action_transmission: ActionTransmission,
}

impl Screen {
    /// Creates a new target with the given size
    pub fn new(width: usize, height: usize) -> Self {
        let action_transmission = ActionTransmission {
            format: Format::Rgba32,
            medium: Medium::Direct,
            width: width as u32,
            height: height as u32,
            compression: false,
            ..Default::default()
        };
        let action = Action::TransmitAndDisplay(
            action_transmission,
            kitty_image::ActionPut {
                ..Default::default()
            },
        );

        Screen {
            width,
            height,
            size: Vector2::new(width as f64, height as f64),
            scale: 1,
            frame_buf: vec![vec![Color::default(); width]; height],
            frame: 1,
            action,
            action_transmission,
        }
    }

    /// Creates a fullscreen target
    pub fn new_fullscreen() -> Self {
        let winsize = get_term_size();

        Screen::new(winsize.ws_xpixel as usize, winsize.ws_ypixel as usize)
    }

    /// Renders to a writer
    pub fn draw_to<W>(&mut self, writer: &mut W)
    where
        W: Write,
    {
        let overflow = self.frame.checked_add(1);
        match overflow {
            Some(new) => self.frame = new,
            None => self.frame = 2,
        }

        // Add the payload to the command
        let mut command = Command::new(self.action);
        command.id = Some(ID(NonZero::new(self.frame as u32).unwrap()));
        command.quietness = Quietness::SuppressAll;
        command.payload = buf_to_payload(&self.frame_buf);

        // Wrap the command in escape codes
        let command = WrappedCommand::new(command);

        command.send_chunked(writer).unwrap();

        self.clear_frame_buf();
        self.clear();
    }

    /// Renders to stdout
    pub fn draw(&mut self) {
        self.draw_to(&mut stdout());
    }

    /// Scales 1 pixel to be `scale` times larger
    pub fn scale(&mut self, scale: usize) {
        self.scale = scale;

        let scaled_width = self.width * scale;
        let scaled_height = self.height * scale;

        // Scale kitty protocol action
        let mut action_transmission = self.action_transmission;
        action_transmission.width = scaled_width as u32;
        action_transmission.height = scaled_height as u32;

        self.action = Action::TransmitAndDisplay(
            action_transmission,
            kitty_image::ActionPut {
                ..Default::default()
            },
        );

        // Scale image buffer
        self.frame_buf = vec![vec![Color::default(); scaled_width]; scaled_height];
    }

    pub fn render(&mut self, model: &Model, transform: &Transform) {
        for (color_idx, triangle) in model.points.windows(3).step_by(3).enumerate() {
            let triangle = (
                self.vertex_to_screen(triangle[0], transform),
                self.vertex_to_screen(triangle[1], transform),
                self.vertex_to_screen(triangle[2], transform),
            );

            // Min and max bounds for a triangle
            let min_bounds = Vector2::new(
                triangle.0.x.min(triangle.1.x).min(triangle.2.x),
                triangle.0.y.min(triangle.1.y).min(triangle.2.y),
            );
            let max_bounds = Vector2::new(
                triangle.0.x.max(triangle.1.x).max(triangle.2.x),
                triangle.0.y.max(triangle.1.y).max(triangle.2.y),
            );

            // Get the bounding box of the triangle
            let block_start = (
                (min_bounds.x as usize).clamp(0, self.width - 1),
                (min_bounds.y as usize).clamp(0, self.height - 1),
            );
            let block_end = (
                (max_bounds.x.ceil() as usize).clamp(0, self.width - 1),
                (max_bounds.y.ceil() as usize).clamp(0, self.height - 1),
            );

            // Precalculate the steps to rasterize the triangle (Optimization)
            let top_left_point = Vector2::new(block_start.0 as f64, block_start.1 as f64);
            let delta_weights_row = Vector3::new(
                triangle.1.x - triangle.0.x,
                triangle.2.x - triangle.1.x,
                triangle.0.x - triangle.2.x,
            );

            let delta_weights_col = Vector3::new(
                triangle.0.y - triangle.1.y,
                triangle.1.y - triangle.2.y,
                triangle.2.y - triangle.0.y,
            );

            // Baricentric coordinates
            let mut weights =
                top_left_point.get_baricentric_coordinates(&triangle.0, &triangle.1, &triangle.2);

            // Render only the pixels contained in the triangle
            for y in block_start.1..block_end.1 {
                let mut step = weights;
                for x in block_start.0..block_end.0 {
                    if !Vector2::is_in_triangle(&step) {
                        step += delta_weights_col;
                        continue;
                    }

                    render_scaled((x, y), self.scale, |scaled_x, scaled_y| {
                        self.frame_buf[scaled_y][scaled_x] = model.face_colors[color_idx];
                    });

                    step += delta_weights_col;
                }
                weights += delta_weights_row;
            }
        }
    }

    /// Transform vertex position to screen space (pixel coordinates)
    fn vertex_to_screen(&self, vertex: Vector3<f64>, transform: &Transform) -> Vector2<f64> {
        let vertex = transform.vertex_to_world(vertex);

        let screen_height_world = 5.;
        let pixels_per_world_unit = self.size.y / screen_height_world;

        let pixel_offset = Vector2::new(vertex.x, vertex.y) * pixels_per_world_unit;
        self.size / 2. + pixel_offset
    }

    fn clear_frame_buf(&mut self) {
        for y in self.frame_buf.iter_mut() {
            for x in y {
                *x = Color::default()
            }
        }
    }

    pub fn clear(&mut self) {
        let action = Action::Delete(ActionDelete {
            hard: true,
            target: DeleteTarget::IDLessEqual {
                id: ID(NonZero::new((self.frame as u32).saturating_sub(1)).unwrap()),
            },
        });

        let mut command = Command::new(action);
        command.quietness = Quietness::SuppressAll;

        let command = WrappedCommand::new(command);

        print!("{command}");
    }
}

/// Replicates a pixel for `scale` amount of times
fn render_scaled<T: FnMut(usize, usize)>(point: (usize, usize), scale: usize, mut callback: T) {
    (0..scale).for_each(|y_offset| {
        (0..scale).for_each(|x_offset| {
            callback(
                (point.0 * scale).saturating_add(x_offset),
                (point.1 * scale).saturating_add(y_offset),
            )
        });
    });
}

/// Converts a color matrix to a byte array and returns an owned payload
fn buf_to_payload(frame_buf: &[Vec<Color>]) -> Cow<[u8]> {
    let mut payload: Vec<u8> = Vec::with_capacity(4 * frame_buf.len());

    for row in frame_buf.iter().rev() {
        for color in row {
            payload.push(color.red);
            payload.push(color.green);
            payload.push(color.blue);
            payload.push(color.alpha);
        }
    }

    Cow::Owned(payload)
}

/// Returns the terminal size
fn get_term_size() -> winsize {
    ioctl_read_bad!(tiocgwinsz, libc::TIOCGWINSZ, winsize);

    unsafe {
        let sz: *mut winsize = &mut winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        let _ = tiocgwinsz(0, sz);
        *sz
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        Color::new(rng.random(), rng.random(), rng.random(), 0xff)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: Default::default(),
            green: Default::default(),
            blue: Default::default(),
            alpha: Default::default(),
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if (value.len() != 9 || value.len() != 7) && value.chars().nth(0) != Some('#') {
            return Err(String::from("not a hex string"));
        }

        let bytes = match u32::from_str_radix(&value[1..], 16) {
            Ok(bytes) => {
                if value.len() == 7 {
                    bytes << 8
                } else {
                    bytes
                }
            }

            Err(e) => return Err(e.to_string()),
        };

        let red = ((bytes & 0xFF000000) >> (8 * 3)) as u8;
        let green = ((bytes & 0x00FF0000) >> (8 * 2)) as u8;
        let blue = ((bytes & 0x0000FF00) >> 8) as u8;
        let mut alpha = 0xFF;

        if value.len() == 9 {
            alpha = (bytes & 0x000000FF) as u8;
        }

        Ok(Color::new(red, green, blue, alpha))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_from_str_noalpha() {
        assert_eq!(
            Color::try_from("#fce822"),
            Ok(Color::new(0xfc, 0xe8, 0x22, 0xff))
        )
    }

    #[test]
    fn test_color_from_str_alpha() {
        assert_eq!(
            Color::try_from("#ab23ffa0"),
            Ok(Color::new(0xab, 0x23, 0xff, 0xa0))
        )
    }

    #[test]
    fn test_color_from_str_parse_error() {
        assert_eq!(
            Color::try_from("#ab23ffag"),
            Err(String::from("invalid digit found in string"))
        )
    }

    #[test]
    fn test_color_from_str_error() {
        assert_eq!(
            Color::try_from("ab23ffag"),
            Err(String::from("not a hex string"))
        )
    }
}
