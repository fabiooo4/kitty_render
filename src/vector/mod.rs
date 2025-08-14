pub mod transform;
pub mod vector2;
pub mod vector3;

use std::ops::Add;
use std::ops::Mul;

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

#[cfg(test)]
mod test {
    use crate::vector::vector2::Vector2;

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

        let baricentric = Vector2::new(4., 4.).get_barycentric_weights(&a, &b, &c);
        assert!(Vector2::is_in_triangle(&baricentric));
    }
}
