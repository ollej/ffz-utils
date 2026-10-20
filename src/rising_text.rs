//! A UI element of texts that rises up.
//!
//! Can be used to show HP damage above another player.
//!
//! If a `scatter` number is defined, the starting position
//! will be randomly scattered by this value in `x`/`y`.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::{camera::*, rising_text::*};
//! let font = load_file_buf("font").unwrap().into_font();
//! let mut rising_texts = RisingTexts::new(font);
//! rising_texts.add("Hello World!".to_string(), Color::Red, Point::new(120, 80));
//! let camera = Camera::new(Size::new(500, 500));
//!
//! // In `update()`
//! rising_texts.update();
//!
//! // In `render()`
//! rising_texts.draw(&camera);
//! ```

use alloc::vec;
use alloc::{string::String, vec::Vec};

use firefly_rust::*;

use crate::{camera::Camera, point_math::PointMath};

/// Manager for small text snippets that rises up from a position.
pub struct RisingTexts {
    font: FontBuf,
    texts: Vec<RisingText>,
    pub scatter: Option<u32>,
}

impl RisingTexts {
    /// Create a new `RisingText` with a given font.
    pub fn new(font: FontBuf) -> Self {
        Self {
            font,
            texts: vec![],
            scatter: None,
        }
    }

    /// Create a new `RisingText` with a scatter.
    pub fn with_scatter(font: FontBuf, scatter: u32) -> Self {
        Self {
            font,
            texts: vec![],
            scatter: Some(scatter),
        }
    }

    /// Add a new text string at the given position.
    ///
    /// If `scatter` field is set, it will be applied to the position.
    pub fn add(&mut self, text: String, color: Color, position: Point) {
        let scattered_position = self
            .scatter
            .map(|scatter| position.scatter(scatter))
            .unwrap_or(position);
        self.texts
            .push(RisingText::new(text, color, scattered_position));
    }

    /// Updates all texts.
    ///
    /// Removes texts that are older than `max_age`.
    pub fn update(&mut self) {
        for text in self.texts.iter_mut() {
            text.update();
        }
        self.texts.retain(|text| !text.should_remove());
    }

    /// Draws all texts relative to the camera position.
    pub fn draw(&self, camera: &Camera) {
        for text in self.texts.iter() {
            text.draw(camera, &self.font);
        }
    }
}

/// A single text that is rising up on the screen.
pub struct RisingText {
    age: i32,
    color: Color,
    content: String,
    position: Point,
    remainder: f32,
    speed: f32,
    max_age: i32,
    shadow: Option<Color>,
}

impl Default for RisingText {
    fn default() -> Self {
        Self {
            age: 0,
            color: Color::White,
            content: String::new(),
            position: Point::MIN,
            remainder: 0.0,
            speed: 0.6,
            max_age: 30,
            shadow: Some(Color::Black),
        }
    }
}

impl RisingText {
    /// Create a new rising text in a given `color` at a given `position`.
    pub fn new(content: String, color: Color, position: Point) -> Self {
        Self {
            content,
            color,
            position,
            ..Default::default()
        }
    }

    /// Updates the text
    ///
    /// Should be called on every `update()` to move texts.
    pub fn update(&mut self) {
        self.age += 1;
        self.remainder += self.speed;
        let amount = math::floor(self.remainder + 0.5);
        self.remainder -= amount;
        self.position.y -= amount as i32;
    }

    /// Draw text relative to `Camera` with given `FontBuf`.
    pub fn draw(&self, camera: &Camera, font: &FontBuf) {
        let position = camera.world_to_screen(self.position);
        if let Some(shadow_color) = self.shadow {
            let shadow_position = position + Point::new(1, 1);
            draw_text(self.content.as_str(), font, shadow_position, shadow_color);
        }
        draw_text(self.content.as_str(), font, position, self.color);
    }

    /// Whether the text should be removed.
    pub fn should_remove(&self) -> bool {
        self.age > self.max_age
    }
}
