//! A 2D vector struct with f32 values.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::vec2::*;
//! let v1 = Vec2::new(1.0, 2.0);
//! let v2 = v1.add(Vec2::new(3.0, 4.0));
//! assert_eq!(v2, Vec2::new(4.0, 6.0));
//! let v3 = Vec2::zero();
//! assert!(v3.is_zero());
//! assert_eq!(v3, Vec2::new(0.0, 0.0));
//! ```

use firefly_rust::Point;

/// A 2D vector with f32 values.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// A Vec2 with zero values.
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Are both values zero?
    pub fn is_zero(&self) -> bool {
        self.x.abs() == 0.0 && self.y.abs() == 0.0
    }

    /// Add another `Vec2`.
    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<Point> for Vec2 {
    fn from(value: Point) -> Self {
        Vec2::new(value.x as f32, value.y as f32)
    }
}
