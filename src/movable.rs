//! A trait that for movable objects.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::{movable::*, rectangle::*, tile::*, tile_map::*, vec2::*};
//!
//! // A `TileMap` is needed to check where object can move.
//! let spritesheet = firefly_rust::load_file_buf("spritesheet").unwrap().into_image();
//! let tile_size = Size::new(16, 16);
//! let tiles = vec![
//!   Tile::new(Rectangle::new(Point::new(0, 0), tile_size), 1, false),
//!   Tile::new(Rectangle::new(Point::new(16, 0), tile_size), 1, false),
//!   Tile::new(Rectangle::new(Point::new(0, 16), tile_size), 1, false),
//!   Tile::new(Rectangle::new(Point::new(16, 16), tile_size), 1, false),
//! ];
//! let tile_map = TileMap::new(tiles, Size::new(32, 32), tile_size, spritesheet);
//!
//! struct Player {
//!   rect: Rectangle,
//!   movement: Vec2,
//! }
//!
//! impl Movable for Player {
//!   fn position(&self) -> Point {
//!     self.rect.position
//!   }
//!   
//!   fn rect(&self) -> Rectangle {
//!     self.rect
//!   }
//!   
//!   fn stop_movement(&mut self, stop_direction: StopDirection) {
//!     match stop_direction {
//!       StopDirection::X => self.movement.x = 0.0,
//!       StopDirection::Y => self.movement.y = 0.0,
//!       StopDirection::Both => self.movement = Vec2::zero(),
//!     }
//!   }
//! }
//!
//! let mut player = Player {
//!   rect: Rectangle::new(Point::new(10, 10), Size::new(16, 16)),
//!   movement: Vec2::new(2.0, 2.0),
//! };
//! player.move_horizontally(player.position(), player.movement, Vec2::zero(), &tile_map);
//! player.move_vertically(player.position(), player.movement, Vec2::zero(), &tile_map);
//! assert_eq!(player.position(), Point::new(12, 12));
//! ```

use firefly_rust::{Point, math};

use crate::{point_math::PointMath, rectangle::Rectangle, tile_map::TileMap, vec2::Vec2};

#[inline]
pub fn movement_to_step(amount: f32) -> i32 {
    if amount > 0. {
        1
    } else if amount < 0. {
        -1
    } else {
        0
    }
}

/// Which direction blocks the movement.
#[derive(PartialEq, Clone, Debug)]
pub enum StopDirection {
    X,
    Y,
    Both,
}

pub trait Movable {
    /// Return the position of the object
    fn position(&self) -> Point;

    /// Return `Rectangle` covering the object.
    fn rect(&self) -> Rectangle;

    /// Stop the current movement in a specific direction.
    fn stop_movement(&mut self, stop_direction: StopDirection);

    /// Move horizontally from position at a velocity.
    ///
    /// Supports nudging upwards 1 pixel if it is blocked.
    /// Calls `stop_movement` with `StopDirection::X` if movement is blocked.
    fn move_horizontally(
        &mut self,
        mut position: Point,
        velocity: Vec2,
        mut remainder: Vec2,
        tile_map: &TileMap,
    ) -> (Point, Vec2) {
        remainder.x += velocity.x;
        let amount = math::floor(remainder.x + 0.5);
        remainder.x -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = position.addx(step);
            let nudge_pos = test_pos.addy(-1);
            if test_pos.is_in_screen() && !tile_map.is_blocked(test_pos) {
                position = test_pos
            } else if nudge_pos.is_in_screen() && !tile_map.is_blocked(nudge_pos) {
                // There was a collision, let's nudge up
                position = nudge_pos
            } else {
                self.stop_movement(StopDirection::X);
                break;
            }
        }

        (position, remainder)
    }

    /// Move vertically from position at a velocity.
    ///
    /// Calls `stop_movement` with `StopDirection::Y` if movement is blocked.
    fn move_vertically(
        &mut self,
        mut position: Point,
        velocity: Vec2,
        mut remainder: Vec2,
        tile_map: &TileMap,
    ) -> (Point, Vec2) {
        remainder.y += velocity.y;
        let amount = math::floor(remainder.y + 0.5);
        remainder.y -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = position.addy(step);
            if test_pos.is_in_screen() && !tile_map.is_blocked(test_pos) {
                position = test_pos;
            } else {
                self.stop_movement(StopDirection::Y);
                break;
            }
        }

        (position, remainder)
    }

    /// If the object overlaps another `Rectangle`.
    fn overlaps(&self, other: Rectangle) -> bool {
        self.rect().overlaps(other)
    }

    /// Are all corners of the object free in the `TileMap`.
    fn is_rect_free(&self, tile_map: &TileMap) -> bool {
        !tile_map.is_rect_blocked(&self.rect())
    }

    /// Whether either bottom corners are on top of a solid tile.
    fn is_standing(&self, tile_map: &TileMap) -> bool {
        tile_map.is_blocked(self.rect().bottom_left().addy(1))
            || tile_map.is_blocked(self.rect().bottom_right().addy(1))
    }

    /// Whether either bottom corners are on top of a `Rectangle`.
    fn is_standing_on_rect(&self, rect: Rectangle) -> bool {
        rect.contains(self.rect().bottom_left().addy(1))
            || rect.contains(self.rect().bottom_right().addy(1))
    }
}
