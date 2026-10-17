use firefly_rust::*;

use crate::rectangle::Rectangle;

pub type Sprite = i32;

#[derive(Clone, Copy)]
pub struct Tile {
    pub rect: Rectangle,
    pub sprite_index: Sprite,
    solid: bool,
}

impl Tile {
    pub fn new(rect: Rectangle, sprite_index: i32, is_solid: bool) -> Self {
        Self {
            rect,
            sprite_index,
            solid: is_solid,
        }
    }

    pub fn position(&self) -> Point {
        self.rect.position
    }

    pub fn is_solid(&self) -> bool {
        self.solid
    }

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
