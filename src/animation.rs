extern crate alloc;

use alloc::vec::Vec;
use firefly_rust::*;

pub type Frame = i32;

pub struct Animations {
    pub animations: Vec<Animation>,
    pub texture: ImageBuf,
}

impl Animations {
    pub fn new(animations: Vec<Animation>, texture: ImageBuf) -> Self {
        Animations {
            animations,
            texture,
        }
    }

    pub fn update(&mut self) {
        self.animations
            .iter_mut()
            .for_each(|animation| animation.update());
        self.animations.retain(|animation| !animation.finished)
    }

    pub fn draw(&self) {
        for animation in self.animations.iter() {
            let sub_image = self
                .texture
                .sub(animation.frame_position(), animation.frame_size());
            draw_sub_image(&sub_image, animation.position);
        }
    }

    pub fn add(&mut self, animation: Animation) {
        self.animations.push(animation);
    }
}

#[derive(Clone, Copy)]
pub struct Animation {
    pub texture: usize,
    pub frames: i32,
    pub width: i32,
    pub height: i32,
    pub position: Point,
    pub current_frame: Frame,
    pub time_per_frame: i32,
    pub frame_timer: i32,
    pub looping: bool,
    pub finished: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            texture: 0,
            frames: 0,
            width: 0,
            height: 0,
            position: Point::MIN,
            current_frame: 0,
            time_per_frame: 1,
            frame_timer: 0,
            looping: true,
            finished: false,
        }
    }
}

impl Animation {
    pub fn new(
        texture: usize,
        frames: i32,
        width: i32,
        height: i32,
        position: Point,
        time_per_frame: i32,
        looping: bool,
    ) -> Self {
        Self {
            texture,
            frames,
            width,
            height,
            position,
            current_frame: 0,
            time_per_frame,
            frame_timer: 0,
            looping,
            finished: false,
        }
    }

    pub fn update(&mut self) {
        self.frame_timer += 1;
        if self.frame_timer > self.time_per_frame {
            self.next_frame();
        }
    }

    pub fn frame_position(&self) -> Point {
        Point::new(self.width * self.current_frame, 0)
    }

    pub fn frame_size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    pub fn middle(&self) -> Point {
        Point::new(self.width / 2, self.height / 2)
    }

    fn next_frame(&mut self) {
        self.frame_timer = 0;
        self.current_frame += 1;
        if self.current_frame >= self.frames {
            self.current_frame = 0;
            if !self.looping {
                self.finished = true;
            }
        }
    }
}
