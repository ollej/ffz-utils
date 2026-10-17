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
    fn draw(&self, position: Point, color: Color) {
        match self {
            Visualization::Point => draw_point(position, color),
            Visualization::Animation(animation, texture) => {
                let animation_position = position - animation.middle();
                let sub_image = texture.sub(animation.frame_position(), animation.frame_size());
                draw_sub_image(&sub_image, animation_position);
            }
        }
    }

    fn update(&mut self) {
        match self {
            Visualization::Point => (),
            Visualization::Animation(animation, _texture) => animation.update(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
    pub lifetime: i32,
    pub max_lifetime: i32,
    pub color: Color,
    pub size: u8,
    pub active: bool,
}

impl Particle {
    fn position(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    fn deactivate_particle(&mut self) {
        self.active = false;
        self.lifetime = 0;
    }
}

pub struct ParticleSystem {
    max_width: i32,
    max_height: i32,
    particles: Vec<Particle>,
    visualization: Visualization,
}

impl ParticleSystem {
    pub fn new(
        max_particles: usize,
        max_width: i32,
        max_height: i32,
        visualization: Visualization,
    ) -> Self {
        Self {
            max_width,
            max_height,
            visualization,
            particles: vec![Particle::default(); max_particles],
        }
    }

    pub fn update(&mut self) {
        for particle in self.particles.iter_mut() {
            if !particle.active {
                continue;
            }
            self.visualization.update();

            particle.x += (particle.vx >> FIXED_POINT_SHIFT) as i32;
            particle.y += (particle.vy >> FIXED_POINT_SHIFT) as i32;

            particle.lifetime = particle.lifetime.saturating_sub(1);

            let out_of_bounds = particle.x < -5
                || particle.x > self.max_width + 5
                || particle.y < -5
                || particle.y > self.max_height + 5;

            if particle.lifetime == 0 || out_of_bounds {
                particle.deactivate_particle();
            }
        }
    }

    pub fn draw(&self) {
        for particle in self.particles.iter() {
            if !particle.active {
                continue;
            }

            self.visualization.draw(particle.position(), particle.color);
        }
    }

    pub fn spawn(
        &mut self,
        x: i32,
        y: i32,
        vx: i32,
        vy: i32,
        lifetime: i32,
        color: Color,
        size: u8,
    ) {
        if let Some(particle) = self.particles.iter_mut().find(|p| !p.active) {
            particle.x = x;
            particle.y = y;
            particle.vx = vx;
            particle.vy = vy;
            particle.lifetime = lifetime;
            particle.max_lifetime = lifetime;
            particle.color = color;
            particle.size = size;
            particle.active = true;
        }
    }

    pub fn spawn_radial_burst(
        &mut self,
        x: i32,
        y: i32,
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

            particle.x = x;
            particle.y = y;
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

    pub fn clear(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.deactivate_particle();
        }
    }

    pub fn count(&self) -> usize {
        self.particles.iter().filter(|p| p.active).count()
    }
}
