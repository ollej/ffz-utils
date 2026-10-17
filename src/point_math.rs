use firefly_rust::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, utilities::random_range};

pub trait PointMath {
    fn angle_to(&self, other: &Point) -> Angle;
    fn distance(&self, other: &Point) -> f32;
    fn point_from_distance_and_angle(&self, distance: f32, angle: Angle) -> (Point, f32);
    fn scatter(&self, modifier: u32) -> Point;
    fn addx(&self, x: i32) -> Point;
    fn addy(&self, y: i32) -> Point;
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
            (distance - self.distance(&new_point)).max(0.0)
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
        self.x >= 0 && self.x <= SCREEN_WIDTH && self.y >= 0 && self.y <= SCREEN_HEIGHT
    }
}

#[cfg(test)]
mod tests {
    use crate::point_math::*;
    use firefly_rust::Point;

    #[test]
    fn angle_to() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 20, y: 20 };
        let angle = p1.angle_to(&p2);
        assert_eq!(angle.to_degrees(), 45.0);
    }
}
