use alloc::vec;
use alloc::{string::String, vec::Vec};

use firefly_rust::*;

use crate::{camera::Camera, point_math::PointMath};

pub struct RisingTexts {
    font: FontBuf,
    texts: Vec<RisingText>,
    scatter: Option<u32>,
}

impl RisingTexts {
    pub fn new(font: FontBuf) -> Self {
        Self {
            font,
            texts: vec![],
            scatter: None,
        }
    }

    pub fn add(&mut self, text: String, color: Color, position: Point) {
        let scattered_position = self
            .scatter
            .map(|scatter| position.scatter(scatter))
            .unwrap_or(position);
        self.texts
            .push(RisingText::new(text, color, scattered_position));
    }

    pub fn draw(&self, camera: &Camera) {
        for text in self.texts.iter() {
            text.draw(camera, &self.font);
        }
    }
}

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
    pub fn new(content: String, color: Color, position: Point) -> Self {
        Self {
            content,
            color,
            position,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.age += 1;
        self.remainder += self.speed;
        let amount = math::floor(self.remainder + 0.5);
        self.remainder -= amount;
        self.position.y -= amount as i32;
    }

    pub fn draw(&self, camera: &Camera, font: &FontBuf) {
        let position = camera.world_to_screen(self.position);
        if let Some(shadow_color) = self.shadow {
            let shadow_position = position + Point::new(1, 1);
            draw_text(self.content.as_str(), font, shadow_position, shadow_color);
        }
        draw_text(self.content.as_str(), font, position, self.color);
    }

    pub fn remove(&self) -> bool {
        self.age > self.max_age
    }
}
