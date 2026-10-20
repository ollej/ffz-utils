//! A tile from a spritesheet with position and size
//!
//! Can be declared to be `solid` which means it can block
//! movement.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::text_box::*;
//! ```

use firefly_rust::*;

use crate::rectangle::Rectangle;

pub type Sprite = i32;

/// Represents a tile from a spritesheet positioned in a map.
#[derive(Clone, Copy)]
pub struct Tile {
    pub rect: Rectangle,
    pub sprite_index: Sprite,
    solid: bool,
}

impl Tile {
    /// Create a new tile
    pub fn new(rect: Rectangle, sprite_index: i32, is_solid: bool) -> Self {
        Self {
            rect,
            sprite_index,
            solid: is_solid,
        }
    }

    /// Position of the top left corner of the tile.
    pub fn position(&self) -> Point {
        self.rect.position
    }

    /// A solid tile cannot be moved into.
    pub fn is_solid(&self) -> bool {
        self.solid
    }

    /// Draw tile as a sub_image from `spritesheet` at `position`.
    pub fn draw(&self, position: Point, spritesheet: &ImageBuf) {
        let tile_sprite = spritesheet.sub(
            Point {
                x: ((self.sprite_index % self.rect.width()) * self.rect.width()),
                y: ((self.sprite_index / self.rect.height()) * self.rect.height()),
            },
            self.rect.size,
        );
        draw_sub_image(&tile_sprite, position);
    }
}
