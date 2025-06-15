use std::ops::Mul;
use std::ops::Sub;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    fn point_on_right_side_of_line(self, a: Vector2, b: Vector2) -> bool {
        let ap = self - a;
        let ab_perp = (b - a).perpendicular();

        ap * ab_perp >= 0.
    }

    pub fn is_in_triangle(&self, a: Vector2, b: Vector2, c: Vector2) -> bool {
        let right_of_ab = self.point_on_right_side_of_line(a, b);
        let right_of_bc = self.point_on_right_side_of_line(b, c);
        let right_of_ca = self.point_on_right_side_of_line(c, a);

        right_of_ab == right_of_bc && right_of_bc == right_of_ca
    }

    fn perpendicular(&self) -> Vector2 {
        Vector2::new(self.y, -self.x)
    }
}

impl Vector for Vector2 {
    fn components(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}

impl Mul for Vector2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }
}

impl Vector for Vector3 {
    fn components(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }
}

impl Mul for Vector3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = Vector2::new(1., 2.);
        let b = Vector2::new(2., 2.);

        assert_eq!(a.dot(b), 6.);
        assert_eq!(a * b, 6.);
    }

    #[test]
    fn test_point_in_triangle() {
        let a = Vector2::new(2., 2.);
        let b = Vector2::new(7., 4.);
        let c = Vector2::new(4., 8.);

        assert!(Vector2::new(4., 4.).is_in_triangle(a, b, c));
    }
}
