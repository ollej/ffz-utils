use core::ops::Sub;

use firefly_rust::{Angle, math};

#[allow(unused)]
use micromath::F32Ext;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    const AXIS_ANGLE: Vector = Vector::new(1.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn xu(&self) -> i32 {
        math::floor(self.x) as i32
    }

    pub fn yu(&self) -> i32 {
        math::floor(self.y) as i32
    }

    pub fn rotate(&self, rotation: f32) -> Vector {
        let old_dir_x = self.x;
        let dir_x = old_dir_x * math::cos(rotation) - self.y * math::sin(rotation);
        let dir_y = old_dir_x * math::sin(rotation) + self.y * math::cos(rotation);
        Vector::new(dir_x, dir_y)
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn angle_to(&self, other: &Vector) -> Angle {
        let angle = other.y.atan2(other.x) - self.y.atan2(self.x);
        Angle::from_radians(angle).normalize()
    }

    pub fn angle_from_axis(&self) -> Angle {
        Vector::AXIS_ANGLE.angle_to(self)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests_vector {
    use super::*;

    fn vectors() -> [Vector; 8] {
        [
            Vector::new(1.0, 0.0),
            Vector::new(-1.0, 0.0),
            Vector::new(0.0, 1.0),
            Vector::new(0.0, -1.0),
            Vector::new(1.0, 1.0),
            Vector::new(-1.0, 1.0),
            Vector::new(1.0, -1.0),
            Vector::new(-1.0, -1.0),
        ]
    }

    #[test]
    fn test_dot() {
        let vecs = vectors();
        let orig_vec = Vector::new(0.5, 0.5);
        let actual = [0.5, -0.5, 0.5, -0.5, 1.0, 0.0, 0.0, -1.0];
        for (vec, dot_product) in vecs.iter().zip(actual.iter()) {
            assert_eq!(orig_vec.dot(vec), *dot_product);
        }
    }

    #[test]
    fn test_magnitude() {
        let vecs = vectors();
        let magnitudes = [
            1.0, 1.0, 1.0, 1.0, 1.4142135, 1.4142135, 1.4142135, 1.4142135,
        ];
        for (vec, magnitude) in vecs.iter().zip(magnitudes.iter()) {
            assert_eq!(vec.magnitude(), *magnitude);
        }
    }

    #[test]
    fn test_angle_to() {
        let vecs = vectors();
        let angles = [
            Angle::from_degrees(0.0),
            Angle::from_degrees(180.0),
            Angle::from_degrees(90.0),
            Angle::from_degrees(270.0),
            Angle::from_degrees(45.0),
            Angle::from_degrees(135.0),
            Angle::from_degrees(315.0),
            Angle::from_degrees(225.0),
        ];
        for (vec, angle) in vecs.iter().zip(angles.iter()) {
            assert_eq!(
                Vector::AXIS_ANGLE.angle_to(vec).to_degrees().round(),
                angle.to_degrees().round()
            );
        }
    }

    #[test]
    fn test_angle_from_axis() {
        let vecs = vectors();
        let angles = [
            Angle::from_degrees(0.0),
            Angle::from_degrees(180.0),
            Angle::from_degrees(90.0),
            Angle::from_degrees(270.0),
            Angle::from_degrees(45.0),
            Angle::from_degrees(135.0),
            Angle::from_degrees(315.0),
            Angle::from_degrees(225.0),
        ];
        for (vec, angle) in vecs.iter().zip(angles.iter()) {
            assert_eq!(
                vec.angle_from_axis().to_degrees().round(),
                angle.to_degrees().round()
            );
        }
    }
}
