use firefly_rust::*;

use crate::rectangle::Rectangle;

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

// TODO: Add border
// TODO: Add line_height
pub struct TextBox {
    font: Option<FontBuf>,
    font_size: Size,
    rect: Rectangle,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
    color: Color,
    padding: i32,
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
    pub fn draw(&self, lines: &[&str]) {
        let Some(font) = &self.font else {
            return;
        };

        let y_pos = match self.vertical_alignment {
            VerticalAlignment::Top => self.rect.y() + self.font_size.height + self.padding,
            VerticalAlignment::Bottom => self.rect.y() + self.rect.height() - self.padding,
            VerticalAlignment::Middle => {
                self.rect.y() + self.font_size.height / 2 + self.rect.height() / 2
            }
        };
        for (i, line) in lines.iter().enumerate() {
            let x_pos = match self.horizontal_alignment {
                HorizontalAlignment::Left => self.rect.x() + self.padding,
                HorizontalAlignment::Right => {
                    self.rect.x() + self.rect.width()
                        - self.padding
                        - line.len() as i32 * self.font_size.width
                }
                HorizontalAlignment::Center => {
                    self.rect.x() + self.rect.width() / 2
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
}
