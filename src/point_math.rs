//! A trait to add calculation methods to `Point`.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::point_math::*;
//! let p1 = Point { x: 10, y: 10 };
//! let p2 = Point { x: 20, y: 20 };
//! let angle = p1.angle_to(&p2);
//! assert_eq!(angle.to_degrees(), 45.0);
//! assert_eq!(p1.distance(&p2), 14.25);
//! assert_eq!(p1.addx(5), Point::new(15, 10));
//! assert_eq!(p1.addy(5), Point::new(10, 15));
//! let (new_point, remainder) = p1.point_from_distance_and_angle(4.5, Angle::HALF_CIRCLE);
//! assert_eq!(new_point, Point::new(5, 10));
//! assert_eq!(remainder, 0.625);
//! assert!(Point::new(10, 10).is_in_screen());
//! ```

use firefly_rust::*;

use crate::utilities::random_range;

pub trait PointMath {
    /// Calculate the distance between two points.
    fn distance(&self, other: &Point) -> f32;

    /// The angle between two points.
    fn angle_to(&self, other: &Point) -> Angle;

    /// Calculate new point based on a `distance` and an `angle`.
    ///
    /// Returns a tuple with the `new_point` and a `remainder` of the distance.
    fn point_from_distance_and_angle(&self, distance: f32, angle: Angle) -> (Point, f32);

    /// Return a new point that's been randomly offset by up to
    /// +/- half `modifier` in both `x` and `y`.
    fn scatter(&self, modifier: u32) -> Point;

    /// Return a new Point with `x` added.
    fn addx(&self, x: i32) -> Point;

    /// Return a new Point with `y` added.
    fn addy(&self, y: i32) -> Point;

    /// Is the point within the screen?
    fn is_in_screen(&self) -> bool;
}

impl PointMath for Point {
    fn distance(&self, other: &Point) -> f32 {
        math::sqrt(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32)
    }

    fn angle_to(&self, other: &Point) -> Angle {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let theta = math::atan2(dy as f32, dx as f32);
        Angle::from_radians(theta)
    }

    fn point_from_distance_and_angle(&self, distance: f32, angle: Angle) -> (Point, f32) {
        let xx = self.x as f32 + (distance * math::cos(angle.to_radians()));
        let yy = self.y as f32 + (distance * math::sin(angle.to_radians()));

        let new_point = Point {
            x: math::floor(xx) as i32,
            y: math::floor(yy) as i32,
        };

        let remainder = if self == &new_point {
            distance
        } else {
            (distance - self.distance(&new_point)).abs()
        };

        (new_point, remainder)
    }

    fn scatter(&self, modifier: u32) -> Point {
        Point {
            x: self.x + (random_range(0, modifier) - modifier / 2) as i32,
            y: self.y + (random_range(0, modifier) - modifier / 2) as i32,
        }
    }

    fn addx(&self, x: i32) -> Point {
        Point {
            x: self.x + x,
            y: self.y,
        }
    }

    fn addy(&self, y: i32) -> Point {
        Point {
            x: self.x,
            y: self.y + y,
        }
    }

    fn is_in_screen(&self) -> bool {
        self.x >= 0 && self.x <= WIDTH && self.y >= 0 && self.y <= HEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use firefly_rust::Point;

    #[test]
    fn angle_to() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 20, y: 20 };
        let angle = p1.angle_to(&p2);
        assert_eq!(angle.to_degrees(), 45.0);
    }

    #[test]
    fn distance() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 20, y: 20 };
        assert_eq!(p1.distance(&p2), 14.25);
    }

    #[test]
    fn point_from_distance_and_angle() {
        let p1 = Point { x: 10, y: 10 };
        let (new_point, remainder) = p1.point_from_distance_and_angle(4.5, Angle::HALF_CIRCLE);
        assert_eq!(new_point, Point::new(5, 10));
        assert_eq!(remainder, 0.625);
    }

    #[test]
    fn addx() {
        let p1 = Point { x: 10, y: 10 };
        assert_eq!(p1.addx(5), Point::new(15, 10));
    }

    #[test]
    fn addy() {
        let p1 = Point { x: 10, y: 10 };
        assert_eq!(p1.addy(5), Point::new(10, 15));
    }

    #[test]
    fn is_in_screen() {
        assert!(Point::new(10, 10).is_in_screen());
        assert!(!Point::new(241, 10).is_in_screen());
        assert!(!Point::new(10, 161).is_in_screen());
    }
}
