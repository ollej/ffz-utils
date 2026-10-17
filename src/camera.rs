use crate::*;
use firefly_rust::Point;

pub struct Camera {
    position: Point,
    world_width: i32,
    world_height: i32,
}

impl Camera {
    pub fn new(world_width: i32, world_height: i32) -> Self {
        Self {
            position: Point::MIN,
            world_width,
            world_height,
        }
    }

    pub fn world_to_screen(&self, world_pos: Point) -> Point {
        Point {
            x: world_pos.x - self.position.x,
            y: world_pos.y - self.position.y,
        }
    }

    pub fn screen_to_world(&self, screen_pos: Point) -> Point {
        Point {
            x: screen_pos.x + self.position.x,
            y: screen_pos.y + self.position.y,
        }
    }

    pub fn is_in_screen(&self, world_pos: Point) -> bool {
        world_pos.x >= self.position.x
            && world_pos.x < self.position.x + SCREEN_WIDTH
            && world_pos.y >= self.position.y
            && world_pos.y < self.position.y + SCREEN_HEIGHT
    }

    fn clamp_to_bounds(&mut self) {
        if self.position.x < 0 {
            self.position.x = 0;
        }

        if self.position.y < 0 {
            self.position.y = 0;
        }

        let max_x = self.world_width - SCREEN_WIDTH;
        let max_y = self.world_height - SCREEN_HEIGHT;

        if max_x > 0 && self.position.x > max_x {
            self.position.x = max_x;
        }

        if max_y > 0 && self.position.y > max_y {
            self.position.y = max_y;
        }
    }

    pub fn follow_player(&mut self, target_pos: Point, smoothness: f32) {
        let target_x = target_pos.x - SCREEN_WIDTH / 2;
        let target_y = target_pos.y - SCREEN_HEIGHT / 2;

        let diff_x = ((target_x - self.position.x) as f32 * smoothness) as i32;
        let diff_y = ((target_y - self.position.y) as f32 * smoothness) as i32;

        self.position.x += diff_x;
        self.position.y += diff_y;

        self.clamp_to_bounds();
    }
}
