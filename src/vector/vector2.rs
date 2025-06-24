use crate::vector::Vector;

use std::clone::Clone;
use std::default::Default;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use super::vector3::Vector3;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vector2<T>
where
    T: Clone,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Neg<Output = T>
        + Clone
        + Sub<T, Output = T>
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + Default
        + PartialOrd,
{
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    fn signed_triangle_area(&self, a: &Vector2<T>, b: &Vector2<T>) -> T {
        let ap = self.clone() - a.clone();
        let ab_perp = (b.clone() - a.clone()).perpendicular();

        ap * ab_perp
    }

    pub fn get_baricentric_coordinates(
        &self,
        a: &Vector2<T>,
        b: &Vector2<T>,
        c: &Vector2<T>,
    ) -> Vector3<T> {
        let area_abp = self.signed_triangle_area(a, b);
        let area_bcp = self.signed_triangle_area(b, c);
        let area_cap = self.signed_triangle_area(c, a);

        Vector3::new(area_abp, area_bcp, area_cap)
    }

    pub fn is_in_triangle(baricentric: &Vector3<T>) -> bool
    where
        T: From<f64>,
    {
        (baricentric.x >= T::from(0.))
            && (baricentric.y >= T::from(0.))
            && (baricentric.z >= T::from(0.))
    }

    fn perpendicular(&self) -> Vector2<T>
    where
        T: Neg<Output = T> + Clone,
    {
        Vector2::new(-self.y.clone(), self.x.clone())
    }
}

impl<T> Vector<T> for Vector2<T>
where
    T: Clone,
{
    fn components(&self) -> Vec<&T> {
        vec![&self.x, &self.y]
    }
}

impl<T> Mul for Vector2<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Default + Clone,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul<T, Output = T>
        + Neg<Output = T>
        + Clone
        + Add<T, Output = T>
        + Default
        + PartialOrd
        + Sub<T, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::new(self.x * rhs.clone(), self.y * rhs)
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Neg<Output = T>
        + Div<T, Output = T>
        + Clone
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector2::new(self.x / rhs.clone(), self.y / rhs)
    }
}

impl<T> Sub for Vector2<T>
where
    T: Sub<T, Output = T>
        + Neg<Output = T>
        + Clone
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Add for Vector2<T>
where
    T: Sub<T, Output = T>
        + Neg<Output = T>
        + Clone
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + PartialOrd
        + Default,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
