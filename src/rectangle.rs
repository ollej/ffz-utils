//! A rectangle is a `position` and a `size`.
//!
//! The position is the top left corner of the rectangle.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::rectangle::*;
//! let rectangle = Rectangle::new(Point::MIN, Size::new(10, 10));
//! assert!(rectangle.contains(Point::new(5, 5)));
//! let overlapping_rectangle = Rectangle::new(Point::new(5, 5), Size::new(10, 10));
//! assert!(rectangle.overlaps(overlapping_rectangle));
//! ```

use firefly_rust::{Point, Size};

use crate::tile_map::TileMap;

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub position: Point,
    pub size: Size,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            position: Point::MIN,
            size: Size::MAX,
        }
    }
}

impl Rectangle {
    /// Create a new `Rectangle` from a `position` and `size`.
    pub fn new(position: Point, size: Size) -> Self {
        Self { position, size }
    }

    /// Create a `Rectangle` from `x`, `y`, `width`, and `height`.
    pub fn from<I: Into<i32>>(x: I, y: I, width: I, height: I) -> Self {
        Self {
            position: Point {
                x: x.into(),
                y: y.into(),
            },
            size: Size {
                width: width.into(),
                height: height.into(),
            },
        }
    }

    /// X position of top left of rectangle.
    pub fn x(&self) -> i32 {
        self.position.x
    }

    /// Y position of top left of rectangle.
    pub fn y(&self) -> i32 {
        self.position.y
    }

    /// Width of rectangle
    pub fn width(&self) -> i32 {
        self.size.width
    }

    /// Height of the rectangle
    pub fn height(&self) -> i32 {
        self.size.height
    }

    /// The position of the bottom right of the rectangle.
    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.position.x + self.size.width - 1,
            y: self.position.y + self.size.height - 1,
        }
    }

    /// The position of the top left of the rectangle.
    pub fn top_left(&self) -> Point {
        self.position
    }

    /// The position of the top right of the rectangle.
    pub fn top_right(&self) -> Point {
        Point {
            x: self.position.x + self.size.width - 1,
            y: self.position.y,
        }
    }

    /// The position of the bottom left of the rectangle.
    pub fn bottom_left(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y + self.size.height - 1,
        }
    }

    /// Whether the rectangle contains the point.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.position.x
            && point.x <= self.bottom_right().x
            && point.y >= self.position.y
            && point.y <= self.bottom_right().y
    }

    /// Whether the rectangle overlaps another rectangle.
    pub fn overlaps(&self, other: Rectangle) -> bool {
        self.contains(other.position) || self.contains(other.bottom_right())
    }

    /// Checks whether any corner of the rectangle is in a solid tile in `TileMap`.
    pub fn is_rect_free(&self, tile_map: &TileMap) -> bool {
        !tile_map.is_rect_blocked(self)
    }
}
