//! A particle system to easily animate particles
//!
//! Particles can be either simple pixels, or animations from an image.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::{ animation::*, particles::*, utilities::* };
//! let image = firefly_rust::load_file_buf("particle_animation").unwrap().into_image();
//! let animation = Animation::new(4, Size::new(16, 16), Point::MIN, 5);
//! let visualization = Visualization::Animation(animation, image);
//! let mut particles = ParticleSystem::new(50, Size::new(100, 100), visualization);
//! particles.spawn_radial_burst(
//!     Point::new(120, 60),
//!     random_range(8, 14) as i32,
//!     || random_range(1, 2) as i32,
//!     2,
//!     Color::Red,
//! );
//!
//! // In `update()`
//! particles.update();
//!
//! // In `render()`
//! particles.draw();
//! ```

extern crate alloc;
use crate::animation::Animation;
use crate::utilities::random_range;
use alloc::vec;
use alloc::vec::Vec;
use firefly_rust::*;

const FIXED_POINT_SHIFT: i16 = 4;

const SIN_TABLE: [i32; 16] = [
    0, 6, 11, 15, 16, 15, 11, 6, 0, -6, -11, -15, -16, -15, -11, -6,
];
const COS_TABLE: [i32; 16] = [
    16, 15, 11, 6, 0, -6, -11, -15, -16, -15, -11, -6, 0, 6, 11, 15,
];

/// Whether particle should be displayed as a single pixel or a sprite animation.
pub enum Visualization {
    Point,
    Animation(Animation, ImageBuf),
}

impl Default for Visualization {
    fn default() -> Self {
        Visualization::Point
    }
}

impl Visualization {
    /// Draw particle based on `Visualization` type.
    fn draw(&self, position: Point, color: Color) {
        match self {
            Visualization::Point => draw_point(position, color),
            Visualization::Animation(animation, texture) => {
                let animation_position = position - animation.middle();
                let sub_image =
                    texture.sub(animation.current_frame_position(), animation.frame_size());
                draw_sub_image(&sub_image, animation_position);
            }
        }
    }

    /// Call on each `update()` to handle particle animations.
    fn update(&mut self) {
        match self {
            Visualization::Point => (),
            Visualization::Animation(animation, _texture) => animation.update(),
        }
    }
}

/// A single moving particle
#[derive(Default, Clone, Copy)]
pub struct Particle {
    pub position: Point,
    pub vx: i32,
    pub vy: i32,
    pub lifetime: i32,
    pub max_lifetime: i32,
    pub color: Color,
    pub size: u8,
    pub active: bool,
}

impl Particle {
    /// Current position of the particle.
    fn position(&self) -> Point {
        self.position
    }

    /// X position of the particle
    pub fn x(&self) -> i32 {
        self.position.x
    }

    /// Y position of the particle
    pub fn y(&self) -> i32 {
        self.position.y
    }

    /// Deactivate the particle so that it won't display or update.
    ///
    /// The `ParticleSystem` keeps a set number of particles and
    /// deactivates those that shouldn't be used.
    fn deactivate_particle(&mut self) {
        self.active = false;
        self.lifetime = 0;
    }
}

/// Keeps track of all particles and allows spawning new ones.
pub struct ParticleSystem {
    size: Size,
    particles: Vec<Particle>,
    visualization: Visualization,
}

impl ParticleSystem {
    /// Create a new particle system with `max_particles` number of particles.
    ///
    /// - `max_particles` - Maximum number of particles to show at once.
    /// - `size` - When particles move out of the size (+/- 5) they are deactivated.
    /// - `visualization` - Show particle as Pixel or Animation.
    pub fn new(max_particles: usize, size: Size, visualization: Visualization) -> Self {
        Self {
            size,
            visualization,
            particles: vec![Particle::default(); max_particles],
        }
    }

    /// Should be called on every `update()`
    ///
    /// Moves particles according to their speed and deactivates
    /// particles that are out of bounds or have reached their liftime.
    pub fn update(&mut self) {
        for particle in self.particles.iter_mut() {
            if !particle.active {
                continue;
            }
            self.visualization.update();

            particle.position.x += (particle.vx >> FIXED_POINT_SHIFT) as i32;
            particle.position.y += (particle.vy >> FIXED_POINT_SHIFT) as i32;

            particle.lifetime = particle.lifetime.saturating_sub(1);

            let out_of_bounds = particle.x() < -5
                || particle.x() > self.size.width + 5
                || particle.y() < -5
                || particle.y() > self.size.height + 5;

            if particle.lifetime == 0 || out_of_bounds {
                particle.deactivate_particle();
            }
        }
    }

    /// Draw all the particles.
    ///
    /// Should be called during `render()`
    pub fn draw(&self) {
        for particle in self.particles.iter() {
            if !particle.active {
                continue;
            }

            self.visualization.draw(particle.position(), particle.color);
        }
    }

    /// Spawn a new particle unless the maximum has been reached.
    pub fn spawn(
        &mut self,
        position: Point,
        vx: i32,
        vy: i32,
        lifetime: i32,
        color: Color,
        size: u8,
    ) {
        if let Some(particle) = self.particles.iter_mut().find(|p| !p.active) {
            particle.position = position;
            particle.vx = vx;
            particle.vy = vy;
            particle.lifetime = lifetime;
            particle.max_lifetime = lifetime;
            particle.color = color;
            particle.size = size;
            particle.active = true;
        }
    }

    /// Spawn a radial burst of particles
    pub fn spawn_radial_burst(
        &mut self,
        position: Point,
        count: i32,
        speed: impl Fn() -> i32,
        lifetime: i32,
        color: Color,
    ) {
        let mut spawned_particles = 0;

        for particle in self.particles.iter_mut() {
            if spawned_particles >= count {
                break;
            }

            if particle.active {
                continue;
            }

            let dir = random_range(0, 15) as usize;
            let particle_speed = speed();
            let vx = particle_speed * COS_TABLE[dir];
            let vy = particle_speed * SIN_TABLE[dir];

            particle.position = position;
            particle.vx = vx;
            particle.vy = vy;
            particle.lifetime = lifetime;
            particle.max_lifetime = lifetime;
            particle.color = color;
            particle.size = 1;
            particle.active = true;

            spawned_particles += 1;
        }
    }

    /// Deactivate all particles
    pub fn clear(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.deactivate_particle();
        }
    }

    /// The number of currently active particles
    pub fn count(&self) -> usize {
        self.particles.iter().filter(|p| p.active).count()
    }
}
