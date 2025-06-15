mod screen;

use screen::{Color, Screen};
use std::{
    ops::{Mul, Sub},
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
        let mut screen = Screen::new(64, 64);

        let mut img_buf: Vec<Vec<Color>> =
            vec![vec![Color::try_from("#000000").unwrap(); screen.width]; screen.height];

        let a = Point::new(0.2 * screen.width as f64, 0.2 * screen.height as f64);
        let b = Point::new(0.7 * screen.width as f64, 0.4 * screen.height as f64);
        let c = Point::new(0.4 * screen.width as f64, 0.8 * screen.height as f64);

        (0..screen.height).for_each(|y| {
            (0..screen.width).for_each(|x| {
                if Point::new(x as f64, y as f64).is_in_triangle(a, b, c) {
                    img_buf[y][x] = Color::new(0xFF, 0xFF, 0xFF, 0xFF);
                }
            });
        });

        screen.render_scaled(&img_buf, 10);
        // screen.render(&mut img_buf);

        sleep(Duration::from_millis(100));
    }
    clearscreen::clear().unwrap();
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn point_on_right_side_of_line(self, a: Point, b: Point) -> bool {
        let ap = self - a;
        let ab_perp = (b - a).perpendicular();

        ap * ab_perp >= 0.
    }

    pub fn is_in_triangle(&self, a: Point, b: Point, c: Point) -> bool {
        let right_of_ab = self.point_on_right_side_of_line(a, b);
        let right_of_bc = self.point_on_right_side_of_line(b, c);
        let right_of_ca = self.point_on_right_side_of_line(c, a);

        right_of_ab == right_of_bc && right_of_bc == right_of_ca
    }

    fn perpendicular(&self) -> Point {
        Point::new(self.y, -self.x)
    }
}

impl Vector for Point {
    fn components(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}

pub trait Vector {
    fn components(&self) -> Vec<f64>;

    fn dot(self, other: Self) -> f64
    where
        Self: std::marker::Sized,
    {
        let self_components = self.components();
        let oth_components = other.components();

        self_components
            .iter()
            .zip(oth_components)
            .fold(0., |acc, (&self_c, other_c)| acc + self_c * other_c)
    }
}

impl Mul for Point {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod test {
    use crate::{Point, Vector};

    #[test]
    fn test_dot_product() {
        let a = Point::new(1., 2.);
        let b = Point::new(2., 2.);

        assert_eq!(a.dot(b), 6.);
        assert_eq!(a * b, 6.);
    }

    #[test]
    fn test_point_in_triangle() {
        let a = Point::new(2., 2.);
        let b = Point::new(7., 4.);
        let c = Point::new(4., 8.);

        assert!(Point::new(4., 4.).is_in_triangle(a, b, c));
    }
}
