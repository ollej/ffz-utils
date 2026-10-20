//! A struct to animate sprites from an ImageBuf.
//!
//! The [`Animations`] struct wraps an `ImageBuf` and keeps track of
//! active [`Animation`] objects from frames in the image.
//!
//! The frames for an animation should be placed in horizontal rows
//! in the image.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::animation::*;
//! // Load a file with a spritesheet and create an empty `Animations` struct.
//! let image = firefly_rust::load_file_buf("animations").unwrap().into_image();
//! let mut animations = Animations::new(image);
//! // Setup an animation with 2 frames, where each sprite is
//! // 16 pixels wide, and the first frame starts in the top
//! // left corner of the image. The animation will be drawn in
//! // the middle of the screen. The animation will have three
//! // frames per second and will loop.
//! let animation = Animation {
//!     frames: 3,
//!     size: Size::new(16, 16),
//!     time_per_frame: 10,
//!     frame_position: Point { x: 0, y: 0 },
//!     position: Point::new(WIDTH / 2 - 8, HEIGHT / 2 - 8),
//!     ..Animation::default()
//! };
//! animations.add(animation);
//!
//! // In update():
//! // The update method needs to be called in every update.
//! animations.update();
//!
//! // In render():
//! // Loops through all active animations and draws them.
//! animations.draw();
//! ```
//!

extern crate alloc;

use alloc::vec::Vec;
use firefly_rust::*;

pub type Frame = i32;

/// Container for active `Animation` objects for an `ImageBuf`
pub struct Animations {
    pub animations: Vec<Animation>,
    pub texture: ImageBuf,
}

impl Animations {
    /// Create an empty animation list from an image file.
    pub fn new(texture: ImageBuf) -> Self {
        Animations {
            animations: Vec::new(),
            texture,
        }
    }

    /// Create a struct from a vec of `Animation` objects
    pub fn with(animations: Vec<Animation>, texture: ImageBuf) -> Self {
        Animations {
            animations,
            texture,
        }
    }

    /// Loop through all `Animation` objects and call update on them.
    ///
    /// Afterwards, all finished animations will be removed.
    /// An animation is finished if it isn't looping, and the last frame
    /// has been shown.
    pub fn update(&mut self) {
        self.animations
            .iter_mut()
            .for_each(|animation| animation.update());
        self.animations.retain(|animation| !animation.finished)
    }

    /// Draws the current frame of each animation.
    pub fn draw(&self) {
        for animation in self.animations.iter() {
            let sub_image = self
                .texture
                .sub(animation.current_frame_position(), animation.frame_size());
            draw_sub_image(&sub_image, animation.position);
        }
    }

    /// Add a new active animation.
    pub fn add(&mut self, animation: Animation) {
        self.animations.push(animation);
    }
}

/// An active animation
#[derive(Clone, Copy)]
pub struct Animation {
    pub frames: i32,
    pub size: Size,
    pub position: Point,
    pub frame_position: Point,
    pub current_frame: Frame,
    pub time_per_frame: i32,
    pub frame_timer: i32,
    pub looping: bool,
    pub finished: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            frames: 0,
            size: Size::new(16, 16),
            position: Point::MIN,
            frame_position: Point::MIN,
            current_frame: 0,
            time_per_frame: 1,
            frame_timer: 0,
            looping: true,
            finished: false,
        }
    }
}

impl Animation {
    /// A new looping animation at a specific position.
    pub fn new(frames: i32, size: Size, position: Point, time_per_frame: i32) -> Self {
        Self {
            frames,
            size,
            position,
            time_per_frame,
            ..Animation::default()
        }
    }

    /// An animation that only plays once.
    pub fn once(frames: i32, size: Size, position: Point, time_per_frame: i32) -> Self {
        Self {
            frames,
            size,
            position,
            time_per_frame,
            looping: false,
            ..Animation::default()
        }
    }

    /// Update method that should be called in every `update()`
    ///
    /// It keeps track of the `current_frame`.
    pub fn update(&mut self) {
        self.frame_timer += 1;
        if self.frame_timer > self.time_per_frame {
            self.next_frame();
        }
    }

    /// Move the draw position of the animation to an absolute point.
    pub fn move_to(&mut self, new_position: Point) {
        self.position = new_position;
    }

    /// Move `x` position by a delta.
    pub fn move_x(&mut self, x: i32) {
        self.position.x += x;
    }

    /// Move `y` position by a delta.
    pub fn move_y(&mut self, y: i32) {
        self.position.y += y;
    }

    /// Returns the frame `size`.
    pub fn frame_size(&self) -> Size {
        self.size
    }

    /// Returns the middle point of the animation.
    pub fn middle(&self) -> Point {
        Point::new(self.size.width / 2, self.size.height / 2)
    }

    /// The position in the image of the current frame
    pub fn current_frame_position(&self) -> Point {
        Point {
            x: self.frame_position.x + self.size.width * self.current_frame,
            y: self.frame_position.y,
        }
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
