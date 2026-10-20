//! Render text from an image of characters.
//!
//! Use an image of an alphabet to display colorful texts on screen.
//! Can be used to show the current score or a title screen.
//!
//! You need to supply an image containing all used characters,
//! and a character `Vec` containing position and size of each character.
//!
//! The `characters` `Vec` should be 36 elements long, starting with
//! 26 elements for the letters followed by 10 elements for the numbers.
//! See the `CHARACTERS` const for an example using the
//! `assets/title-font.png` image.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::image_text::*;
//! let image = firefly_rust::load_file_buf("title_font").unwrap().into_image();
//! let image_text = ImageText::new(CHARACTERS.to_vec(), image);
//! image_text.draw("Hello", Point::MIN, 1);
//! ```

use alloc::vec::Vec;
use firefly_rust::*;

pub static CHARACTERS: [Character; 36] = [
    Character::new(
        Point { x: 0, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 35, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 71, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 105, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 141, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 177, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 212, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 247, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 283, y: 0 },
        Size {
            width: 18,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 302, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 337, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 373, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 408, y: 0 },
        Size {
            width: 49,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 458, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 494, y: 0 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 0, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 35, y: 37 },
        Size {
            width: 35,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 71, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 106, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 141, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 177, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 212, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 247, y: 37 },
        Size {
            width: 49,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 298, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 333, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 368, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 404, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 438, y: 37 },
        Size {
            width: 21,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 459, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 494, y: 37 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 0, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 35, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 71, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 106, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 141, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
    Character::new(
        Point { x: 177, y: 74 },
        Size {
            width: 34,
            height: 35,
        },
    ),
];

/// Contains position and size of a character in image.
#[derive(Copy, Clone)]
pub struct Character {
    position: Point,
    size: Size,
}

impl Character {
    pub const fn new(position: Point, size: Size) -> Self {
        Self { position, size }
    }
}

/// A wrapper for the list of character positions and an image.
pub struct ImageText {
    pub characters: Vec<Character>,
    pub texture: ImageBuf,
}

impl ImageText {
    pub fn new(characters: Vec<Character>, texture: ImageBuf) -> Self {
        Self {
            characters,
            texture,
        }
    }

    /// Draw a text on screen at a given position.
    ///
    /// There is no text wrapping, as this is mainly meant for shorter texts.
    /// Only the letters `a` to `z` and the numbers `0` to `9` are supported.
    ///
    /// - `letter_spacing` is how many pixels space there should be between each character.
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
