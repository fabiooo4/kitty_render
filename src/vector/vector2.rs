use crate::vector::Vector;

use std::clone::Clone;
use std::default::Default;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use super::vector3::Vector3;

#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn is_in_triangle(
        &self,
        a: &Vector2<T>,
        b: &Vector2<T>,
        c: &Vector2<T>,
        weights: &mut Vector3<f64>,
    ) -> bool
    where
        T: Div<f64, Output = T> + PartialOrd<f64> + Into<f64>,
    {
        let area_abp: f64 = self.signed_triangle_area(a, b).into();
        let area_bcp: f64 = self.signed_triangle_area(b, c).into();
        let area_cap: f64 = self.signed_triangle_area(c, a).into();

        let is_in_triangle = area_abp >= 0. && area_bcp >= 0. && area_cap >= 0.;

        let normalization = (area_abp / area_abp) /* 1 */ / (area_abp + area_bcp + area_cap);
        *weights = Vector3::new(
            area_abp * normalization,
            area_bcp * normalization,
            area_cap * normalization,
        );

        is_in_triangle
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
