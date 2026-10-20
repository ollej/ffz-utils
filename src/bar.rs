//! Display a UI bar on the screen
//!
//! The bar will have a stroke color and a fill color.
//! It is also possible to set a corner radius.
//! Useful to draw health or mana bars.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::bar::*;
//! let mut bar = Bar::new(Point::new(10, 10), 10, Size::new(20, 5));
//! bar.set_current_value(5);
//! bar.draw();
//! ```

use firefly_rust::{Color, Point, Size, Style, draw_rounded_rect};

pub struct Bar {
    position: Point,
    current_value: i32,
    max_value: i32,
    style: Style,
    size: Size,
    radius: Size,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            position: Point::MIN,
            current_value: 0,
            max_value: 1,
            style: Style::default(),
            size: Size {
                width: 20,
                height: 5,
            },
            radius: Size {
                width: 2,
                height: 2,
            },
        }
    }
}

impl Bar {
    /// Create a new bar at a `position`, with a `max_value` and a `size`.
    pub fn new(position: Point, max_value: i32, size: Size) -> Self {
        Self {
            position,
            max_value,
            size,
            ..Default::default()
        }
    }

    /// Draw the bar, should be called during `render()`
    pub fn draw(&self) {
        draw_rounded_rect(
            self.position,
            Size {
                width: self.size.width * self.current_value / self.max_value,
                ..self.size
            },
            self.radius,
            Style {
                stroke_color: self.style.fill_color,
                ..self.style
            },
        );
        draw_rounded_rect(
            self.position,
            self.size,
            self.radius,
            Style {
                fill_color: Color::None,
                ..self.style
            },
        );
    }

    /// Update the current value of bar.
    ///
    /// The value will be clamped to `0..max_value`
    pub fn set_current_value(&mut self, new_value: i32) {
        self.current_value = new_value.min(self.max_value).max(0);
    }

    /// Increase `current_value` by `1`.
    ///
    /// The value will never increase above `max_value`.
    pub fn inc(&mut self) {
        if self.current_value < self.max_value {
            self.current_value += 1;
        }
    }

    /// Decrease `current_value` by `1`.
    ///
    /// The value will never decrease below `0`.
    pub fn dec(&mut self) {
        if self.current_value > 0 {
            self.current_value -= 1;
        }
    }

    /// Set `current_value` to the `max_value`.
    pub fn set_max(&mut self) {
        self.current_value = self.max_value;
    }

    /// Sets the `current_value` to `0`.
    pub fn set_zero(&mut self) {
        self.current_value = 0;
    }
}
