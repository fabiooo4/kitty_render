use std::{borrow::Cow, io::Write, io::stdout};

use kitty_image::{Action, ActionTransmission, Command, Format, Medium, WrappedCommand};
use nix::{
    ioctl_read_bad,
    libc::{self, winsize},
};

pub struct Screen {
    pub width: usize,
    pub height: usize,
    action: Action,
}

impl Screen {
    /// Creates a new target with the given size
    pub fn new(width: usize, height: usize) -> Self {
        let action = Action::TransmitAndDisplay(
            ActionTransmission {
                format: Format::Rgba32,
                medium: Medium::Direct,
                width: width as u32,
                height: height as u32,
                ..Default::default()
            },
            kitty_image::ActionPut {
                ..Default::default()
            },
        );

        Screen {
            width,
            height,
            action,
        }
    }

    /// Creates a fullscreen target
    pub fn new_fullscreen() -> Self {
        let winsize = get_term_size();

        Screen::new(winsize.ws_xpixel as usize, winsize.ws_ypixel as usize)
    }

    /// Renders to a writer
    pub fn render_to<W>(&self, img_buf: &Vec<Vec<Color>>, writer: &mut W)
    where
        W: Write,
    {
        // Add the payload to the command
        let mut command = Command::new(self.action);
        command.payload = buf_to_payload(img_buf);

        // Wrap the command in escape codes
        let command = WrappedCommand::new(command);

        command.send_chunked(writer).unwrap();
    }

    /// Renders to stdout
    pub fn render(&self, img_buf: &Vec<Vec<Color>>) {
        self.render_to(img_buf, &mut stdout());
    }
}

/// Converts a color matrix to a byte array and returns an owned payload
fn buf_to_payload(img_buf: &Vec<Vec<Color>>) -> Cow<[u8]> {
    let mut payload: Vec<u8> = Vec::with_capacity(4 * img_buf.len());

    for row in img_buf {
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
