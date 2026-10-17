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
    pub fn new(position: Point, max_value: i32, size: Size) -> Self {
        Self {
            position,
            max_value,
            size,
            ..Default::default()
        }
    }

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

    pub fn set_current_value(&mut self, new_value: i32) {
        self.current_value = new_value.min(self.max_value).max(0);
    }

    pub fn inc(&mut self) {
        if self.current_value < self.max_value {
            self.current_value += 1;
        }
    }

    pub fn dec(&mut self) {
        if self.current_value > 0 {
            self.current_value -= 1;
        }
    }

    pub fn set_max(&mut self) {
        self.current_value = self.max_value;
    }

    pub fn set_zero(&mut self) {
        self.current_value = 0;
    }
}
