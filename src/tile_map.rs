//! A map of `Tile` objects
//!
//! Draw tiles from a `spritesheet` `ImageBuf` to draw a world map.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::{camera::*, rectangle::*, tile::*, tile_map::*};
//! let spritesheet = firefly_rust::load_file_buf("spritesheet").unwrap().into_image();
//! let tile_size = Size::new(16, 16);
//! let tiles = vec![
//!   Tile::new(Rectangle::new(Point::new(0, 0), tile_size), 1, false),
//!   Tile::new(Rectangle::new(Point::new(16, 0), tile_size), 1, false),
//!   Tile::new(Rectangle::new(Point::new(0, 16), tile_size), 1, false),
//!   Tile::new(Rectangle::new(Point::new(16, 16), tile_size), 1, false),
//! ];
//! let map = TileMap::new(tiles, Size::new(32, 32), tile_size, spritesheet);
//! let camera = Camera::new(Size::new(500, 500));
//! map.draw(&camera);
//! ```

use alloc::vec::Vec;

use firefly_rust::*;

use crate::{camera::Camera, rectangle::Rectangle, tile::Tile, utilities::random_range};

/// A world map based on tiles that can be drawn on the screen.
pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub map_size: Size,
    pub tile_size: Size,
    pub spritesheet: ImageBuf,
}

impl TileMap {
    /// Create a new `TileMap`
    pub fn new(tiles: Vec<Tile>, map_size: Size, tile_size: Size, spritesheet: ImageBuf) -> Self {
        Self {
            tiles,
            map_size,
            tile_size,
            spritesheet,
        }
    }

    /// A `Rectangle` of the map's position and size.
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            position: Point::MIN,
            size: self.map_size,
        }
    }

    /// Create a map from a 2D array and a spritesheet.
    ///
    /// - `data` contains a number for each tile in the map.
    ///   The number is the `sprite_index` of the tile in the spritesheet.
    /// - `tile_size` is the size of each tile in the spritesheet.
    /// - `is_solid` is called with the `sprite_index`, the tile will be solid if it returns true.
    /// - `spritesheet` is an image where tiles will be drawn from.
    pub fn new_from_2d_array(
        data: &[&[i32]],
        tile_size: Size,
        is_solid: impl Fn(i32) -> bool,
        spritesheet: ImageBuf,
    ) -> Self {
        let height = data.len() as i32;
        let width = data.first().map_or(0, |row| row.len() as i32);

        let mut tiles = Vec::with_capacity((width * height) as usize);

        for (y, row) in data.iter().enumerate() {
            for (x, &sprite_index) in row.iter().enumerate() {
                let solid = is_solid(sprite_index);
                let rect = Rectangle::new(Point::new(x as i32, y as i32), tile_size);
                tiles.push(Tile::new(rect, sprite_index, solid));
            }
        }

        Self {
            tiles,
            map_size: Size { width, height },
            tile_size,
            spritesheet,
        }
    }

    /// Draw the tiles to the screen based on the `camera` position.
    ///
    /// The tile map can be bigger than the screen, the camera is used
    /// to translate the position in the map to the screen.
    pub fn draw(&self, camera: &Camera) {
        let screen_start = camera.screen_to_world(Point { x: 0, y: 0 });
        let screen_end = camera.screen_to_world(Point {
            x: WIDTH,
            y: HEIGHT,
        });

        let start_x = (screen_start.x / self.tile_size.width).max(0);
        let start_y = (screen_start.y / self.tile_size.height).max(0);
        let end_x = ((screen_end.x / self.tile_size.width) + 1).min(self.map_size.width);
        let end_y = ((screen_end.y / self.tile_size.height) + 1).min(self.map_size.height);

        // Culling out-of-bounds tiles
        for y in start_y..end_y {
            for x in start_x..end_x {
                if let Some(tile) = self.get_tile(x, y)
                    && tile.sprite_index != 0
                {
                    let screen_pos = camera.world_to_screen(tile.position());
                    tile.draw(screen_pos, &self.spritesheet);
                }
            }
        }
    }

    /// Draw the map without using a camera for translation.
    ///
    /// Mostly useful for debugging.
    pub fn draw_all_without_camera(&self) {
        // For testing / debug
        for tile in self.tiles.iter() {
            tile.draw(tile.position(), &self.spritesheet);
        }
    }

    /// Returns a random point in the map that isn't occupied by a solid tile.
    ///
    /// Can be used to generate random positions of monsters.
    pub fn random_unblocked_point(&self) -> Point {
        self.random_unblocked_point_in_rectangle(self.rect())
    }

    // Returns random unblocked point within given `Rectangle`.
    pub fn random_unblocked_point_in_rectangle(&self, rect: Rectangle) -> Point {
        let bottom_right = rect.bottom_right();
        let point = Point {
            x: random_range(rect.x() as u32, bottom_right.x as u32) as i32,
            y: random_range(rect.y() as u32, bottom_right.y as u32) as i32,
        };

        if self.is_blocked(point) {
            self.random_unblocked_point_in_rectangle(rect)
        } else {
            point
        }
    }

    /// Is the point is in a blocked/solid tile.
    pub fn is_blocked(&self, point: Point) -> bool {
        let tile_x = point.x / self.tile_size.width;
        let tile_y = point.y / self.tile_size.height;
        self.get_tile(tile_x, tile_y)
            .map(|t| t.is_solid())
            .unwrap_or(false)
    }

    /// Are any of the corners of the rectangle in a solid tile?
    pub fn is_rect_blocked(&self, rect: &Rectangle) -> bool {
        self.is_blocked(rect.top_left())
            || self.is_blocked(rect.top_right())
            || self.is_blocked(rect.bottom_left())
            || self.is_blocked(rect.bottom_right())
    }

    /// Get tile at the `x` and `y` position.
    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        let index = self.convert_pos_to_index(x, y)?;
        self.tiles.get(index)
    }

    /// Convert an `x` and `y` position to a tile index.
    fn convert_pos_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.map_size.width && y >= 0 && y < self.map_size.height {
            Some((y * self.map_size.width + x) as usize)
        } else {
            None
        }
    }
}
