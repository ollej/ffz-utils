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

#[derive(PartialEq, Clone, Debug)]
pub enum StopDirection {
    X,
    Y,
    Both,
}

pub trait Movable {
    fn position(&self) -> Point;

    fn rect(&self) -> Rectangle;

    fn stop_movement(&mut self, stop_direction: StopDirection);

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

    fn overlaps(&self, other: Rectangle) -> bool {
        self.rect().overlaps(other)
    }

    fn is_position_free(&self, position: Point, tile_map: &TileMap) -> bool {
        !tile_map.is_blocked(position)
    }

    fn is_rect_free(&self, tile_map: &TileMap) -> bool {
        !tile_map.is_rect_blocked(&self.rect())
    }

    fn is_standing(&self, tile_map: &TileMap) -> bool {
        tile_map.is_blocked(self.rect().bottom_left().addy(1))
            || tile_map.is_blocked(self.rect().bottom_right().addy(1))
    }

    fn is_standing_on_rect(&self, rect: Rectangle) -> bool {
        rect.contains(self.rect().bottom_left().addy(1))
            || rect.contains(self.rect().bottom_right().addy(1))
    }
}
