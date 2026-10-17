use alloc::vec::Vec;
use firefly_rust::*;

pub const CHARACTERS: [(Point, Size); 36] = [
    (
        Point { x: 0, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 35, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 71, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 105, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 141, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 177, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 212, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 247, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 283, y: 0 },
        Size {
            width: 18,
            height: 35,
        },
    ),
    (
        Point { x: 302, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 337, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 373, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 408, y: 0 },
        Size {
            width: 49,
            height: 35,
        },
    ),
    (
        Point { x: 458, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 494, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 0, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 35, y: 37 },
        Size {
            width: 35,
            height: 35,
        },
    ),
    (
        Point { x: 71, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 106, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 141, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 177, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 212, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 247, y: 37 },
        Size {
            width: 49,
            height: 35,
        },
    ),
    (
        Point { x: 298, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 333, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 368, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 404, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 438, y: 37 },
        Size {
            width: 21,
            height: 35,
        },
    ),
    (
        Point { x: 459, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 494, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 0, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 35, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 71, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 106, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 141, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    (
        Point { x: 177, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
];

pub struct Character {
    position: Point,
    size: Size,
}

pub struct ImageText {
    characters: Vec<Character>,
    texture: ImageBuf,
}

impl ImageText {
    pub fn draw(&self, text: &str, position: Point, letter_spacing: i32) {
        let mut point = position;
        for character in text
            .to_uppercase()
            .chars()
            .filter_map(|c| match c {
                'A'..'Z' => Some(c as usize - 65),
                '0'..'9' => Some(c as usize - 22),
                _ => None,
            })
            .filter_map(|index| self.characters.get(index))
        {
            let sub_image = self.texture.sub(character.position, character.size);
            draw_sub_image(&sub_image, point);
            point.x += character.size.width + letter_spacing;
        }
    }
}
