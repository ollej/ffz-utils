use alloc::vec::Vec;

use firefly_rust::*;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH, camera::Camera, rectangle::Rectangle, tile::Tile,
    utilities::random_range,
};

pub struct TileMap {
    tiles: Vec<Tile>,
    map_size: Size,
    tile_size: Size,
    spritesheet: ImageBuf,
}

impl TileMap {
    pub fn new(tiles: Vec<Tile>, map_size: Size, tile_size: Size, spritesheet: ImageBuf) -> Self {
        Self {
            tiles,
            map_size,
            tile_size,
            spritesheet,
        }
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle {
            position: Point::MIN,
            size: self.map_size,
        }
    }

    pub fn new_from_2d_array(data: &[&[i32]], tile_size: Size, spritesheet: ImageBuf) -> Self {
        let height = data.len() as i32;
        let width = data.first().map_or(0, |row| row.len() as i32);

        let mut tiles = Vec::with_capacity((width * height) as usize);

        for (y, row) in data.iter().enumerate() {
            for (x, &sprite_index) in row.iter().enumerate() {
                let mut solid = true;
                if sprite_index == 0
                    || sprite_index == 16
                    || sprite_index == 17
                    || sprite_index == 24
                    || sprite_index == 25
                {
                    solid = false
                }

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

    pub fn draw(&self, camera: &Camera) {
        let screen_start = camera.screen_to_world(Point { x: 0, y: 0 });
        let screen_end = camera.screen_to_world(Point {
            x: SCREEN_WIDTH,
            y: SCREEN_HEIGHT,
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

    pub fn draw_all_without_camera(&self) {
        // For testing / debug
        for tile in self.tiles.iter() {
            tile.draw(tile.position(), &self.spritesheet);
        }
    }

    pub fn random_unblocked_point(&self) -> Point {
        self.random_unblocked_point_in_rectangle(self.rect())
    }

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

    pub fn is_blocked(&self, point: Point) -> bool {
        let tile_x = point.x / self.tile_size.width;
        let tile_y = point.y / self.tile_size.height;
        self.get_tile(tile_x, tile_y)
            .map(|t| t.is_solid())
            .unwrap_or(false)
    }

    pub fn is_rect_blocked(&self, rect: &Rectangle) -> bool {
        self.is_blocked(rect.top_left())
            || self.is_blocked(rect.top_right())
            || self.is_blocked(rect.bottom_left())
            || self.is_blocked(rect.bottom_right())
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        let index = self.convert_pos_to_index(x, y)?;
        self.tiles.get(index)
    }

    fn convert_pos_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.map_size.width && y >= 0 && y < self.map_size.height {
            Some((y * self.map_size.width + x) as usize)
        } else {
            None
        }
    }
}
