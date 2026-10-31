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
    spritesheet_position: Point,
}

impl Tile {
    /// Create a new tile
    ///
    /// - `rect` is the size and position of the tile in the map.
    /// - `spritesheet_size` is the size of the spritesheet in tiles.
    /// - `sprite_index` is the index of the sprite in the spritesheet.
    /// - `solid` is whether the tile is blocking.
    pub fn new(rect: Rectangle, spritesheet_size: Size, sprite_index: i32, solid: bool) -> Self {
        let spritesheet_position = Point {
            x: ((sprite_index % spritesheet_size.width) * rect.width()),
            y: ((sprite_index / spritesheet_size.height) * rect.height()),
        };
        Self {
            rect,
            sprite_index,
            solid,
            spritesheet_position,
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
        let tile_sprite = spritesheet.sub(self.spritesheet_position, self.rect.size);
        draw_sub_image(&tile_sprite, position);
    }
}
