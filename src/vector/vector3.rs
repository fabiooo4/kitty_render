use crate::vector::Vector;
use crate::vector::vector2::Vector2;

use std::clone::Clone;
use std::default::Default;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vector3<T>
where
    T: Clone,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Neg<Output = T>
        + Clone
        + Sub<T, Output = T>
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + Default
        + PartialOrd,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }
}

impl<T> Vector<T> for Vector3<T>
where
    T: Clone,
{
    fn components(&self) -> Vec<&T> {
        vec![&self.x, &self.y, &self.z]
    }
}

impl<T> Mul for Vector3<T>
where
    T: Clone + Mul<T, Output = T> + Add<T, Output = T> + Default,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Clone
        + Mul<T, Output = T>
        + Neg<Output = T>
        + Add<T, Output = T>
        + Default
        + PartialOrd
        + Sub<T, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::new(
            self.x * rhs.clone(),
            self.y * rhs.clone(),
            self.z * rhs.clone(),
        )
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Clone
        + Div<T, Output = T>
        + Neg<Output = T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector3::new(
            self.x / rhs.clone(),
            self.y / rhs.clone(),
            self.z / rhs.clone(),
        )
    }
}

impl<T> Sub for Vector3<T>
where
    T: Clone
        + Sub<T, Output = T>
        + Neg<Output = T>
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Add for Vector3<T>
where
    T: Clone
        + Add<T, Output = T>
        + Neg<Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> AddAssign for Vector3<T>
where
    T: Clone
        + Add<T, Output = T>
        + Neg<Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl<T> Into<Vector2<T>> for Vector3<T>
where
    T: Clone
        + Neg<Output = T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    fn into(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }
}

impl Div<Vector3<f64>> for f64 {
    type Output = Vector3<f64>;

    fn div(self, rhs: Vector3<f64>) -> Self::Output {
        Vector3::new(
            self / rhs.x.clone(),
            self / rhs.y.clone(),
            self / rhs.z.clone(),
        )
    }
}
