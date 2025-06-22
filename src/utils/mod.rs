pub mod vector2;

use std::clone::Clone;
use std::default::Default;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub trait Vector<T>
where
    T: Clone,
{
    fn components(&self) -> Vec<&T>;

    fn dot(self, other: Self) -> T
    where
        T: Mul<T, Output = T> + Add<T, Output = T> + Default,
        Self: std::marker::Sized,
    {
        let self_components = self.components();
        let oth_components = other.components();

        self_components
            .iter()
            .zip(oth_components)
            .fold(T::default(), |acc, (&self_c, other_c)| {
                acc + self_c.clone() * other_c.clone()
            })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }
}

impl Vector<f64> for Vector3 {
    fn components(&self) -> Vec<&f64> {
        vec![&self.x, &self.y, &self.z]
    }
}

impl Mul for Vector3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::vector2::Vector2;

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

        assert!(Vector2::new(4., 4.).is_in_triangle(&a, &b, &c));
    }
}
