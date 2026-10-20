//! A UI element to display text inside a box.
//!
//! The text can be aligned vertically and horizontally within the box.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::{rectangle::*, text_box::*};
//! let font = load_file_buf("font").unwrap().into_font();
//! let text_box = TextBox {
//!   font: Some(font),
//!   font_size: Size::new(8, 8),
//!   horizontal_alignment: HorizontalAlignment::Left,
//!   rect: Rectangle {
//!     position: Point::new(10, 10),
//!     size: Size::new(120, 60),
//!   },
//!   vertical_alignment: VerticalAlignment::Middle,
//!   color: Color::Red,
//!   ..TextBox::default()
//! };
//! text_box.draw(&["Hello, World!"]);
//! ```

use firefly_rust::*;

use crate::rectangle::Rectangle;

/// Vertical alignment of the text within the box.
pub enum VerticalAlignment {
    Top,
    Bottom,
    Middle,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Top
    }
}

/// Horizontal alignment of the text within the box.
pub enum HorizontalAlignment {
    Left,
    Right,
    Center,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Left
    }
}

/// A text box with lines of text that can be aligned.
pub struct TextBox {
    pub font: Option<FontBuf>,
    pub font_size: Size,
    pub rect: Rectangle,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub color: Color,
    pub padding: i32,
}

impl Default for TextBox {
    fn default() -> Self {
        Self {
            font: None,
            font_size: Size::new(8, 8),
            horizontal_alignment: HorizontalAlignment::Left,
            rect: Rectangle::default(),
            vertical_alignment: VerticalAlignment::Top,
            color: Color::Black,
            padding: 0,
        }
    }
}

impl TextBox {
    /// Draw a box with line of texts at a position.
    pub fn draw_at(&self, position: Point, lines: &[&str]) {
        let Some(font) = &self.font else {
            return;
        };

        let y_pos = match self.vertical_alignment {
            VerticalAlignment::Top => position.y + self.font_size.height + self.padding,
            VerticalAlignment::Bottom => position.y + self.rect.height() - self.padding,
            VerticalAlignment::Middle => {
                position.y + self.font_size.height / 2 + self.rect.height() / 2
            }
        };
        for (i, line) in lines.iter().enumerate() {
            let x_pos = match self.horizontal_alignment {
                HorizontalAlignment::Left => position.x + self.padding,
                HorizontalAlignment::Right => {
                    position.x + self.rect.width()
                        - self.padding
                        - line.len() as i32 * self.font_size.width
                }
                HorizontalAlignment::Center => {
                    position.x + self.rect.width() / 2
                        - line.len() as i32 * self.font_size.width / 2
                }
            };
            draw_text(
                line,
                font,
                Point {
                    x: x_pos,
                    y: y_pos + i as i32 * self.font_size.height,
                },
                self.color,
            );
        }
    }

    /// Draw the lines of text within the box at the default position.
    pub fn draw(&self, lines: &[&str]) {
        self.draw_at(self.rect.position, lines);
    }
}
