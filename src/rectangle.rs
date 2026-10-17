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
    pub fn new(position: Point, size: Size) -> Self {
        Self { position, size }
    }

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

    pub fn x(&self) -> i32 {
        self.position.x
    }

    pub fn y(&self) -> i32 {
        self.position.y
    }

    pub fn width(&self) -> i32 {
        self.size.width
    }

    pub fn height(&self) -> i32 {
        self.size.height
    }

    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.position.x + self.size.width - 1,
            y: self.position.y + self.size.height - 1,
        }
    }

    pub(crate) fn top_left(&self) -> Point {
        self.position
    }

    pub(crate) fn top_right(&self) -> Point {
        Point {
            x: self.position.x + self.size.width - 1,
            y: self.position.y,
        }
    }

    pub(crate) fn bottom_left(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y + self.size.height - 1,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.position.x
            && point.x <= self.bottom_right().x
            && point.y >= self.position.y
            && point.y <= self.bottom_right().y
    }

    pub fn overlaps(&self, other: Rectangle) -> bool {
        self.contains(other.position) || self.contains(other.bottom_right())
    }

    pub fn is_rect_free(&self, tile_map: &TileMap) -> bool {
        !tile_map.is_rect_blocked(self)
    }
}
