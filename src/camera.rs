//! A camera to translate positions from world to screen.
//!
//! Allows having a level that is bigger than the screen.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::camera::*;
//! let camera = Camera::new(Size::new(500, 500));
//! let world_position = Point::new(250, 250);
//! let screen_position = camera.world_to_screen(world_position);
//! ```

use firefly_rust::{HEIGHT, Point, Size, WIDTH};

pub struct Camera {
    position: Point,
    world_size: Size,
}

impl Camera {
    /// Create a new camera with a given size.
    pub fn new(world_size: Size) -> Self {
        Self {
            position: Point::MIN,
            world_size,
        }
    }

    /// Translate a world position to a screen position.
    pub fn world_to_screen(&self, world_position: Point) -> Point {
        Point {
            x: world_position.x - self.position.x,
            y: world_position.y - self.position.y,
        }
    }

    /// Translate a screen position to a world position.
    pub fn screen_to_world(&self, screen_position: Point) -> Point {
        Point {
            x: screen_position.x + self.position.x,
            y: screen_position.y + self.position.y,
        }
    }

    /// Check whether `world_position` is within the screen.
    pub fn is_in_screen(&self, world_position: Point) -> bool {
        world_position.x >= self.position.x
            && world_position.x < self.position.x + WIDTH
            && world_position.y >= self.position.y
            && world_position.y < self.position.y + HEIGHT
    }

    /// Move camera position to a target position.
    ///
    /// The `smoothness` defines how fast the camera should move towards the position.
    pub fn follow_player(&mut self, target_position: Point, smoothness: f32) {
        let target_x = target_position.x - WIDTH / 2;
        let target_y = target_position.y - HEIGHT / 2;

        let diff_x = ((target_x - self.position.x) as f32 * smoothness) as i32;
        let diff_y = ((target_y - self.position.y) as f32 * smoothness) as i32;

        self.position.x += diff_x;
        self.position.y += diff_y;

        self.clamp_to_bounds();
    }

    fn clamp_to_bounds(&mut self) {
        if self.position.x < 0 {
            self.position.x = 0;
        }

        if self.position.y < 0 {
            self.position.y = 0;
        }

        let max_x = self.world_size.width - WIDTH;
        let max_y = self.world_size.height - HEIGHT;

        if max_x > 0 && self.position.x > max_x {
            self.position.x = max_x;
        }

        if max_y > 0 && self.position.y > max_y {
            self.position.y = max_y;
        }
    }
}
