//! A utility library for Firefly Zero
//!
//! A library with utilities to write games in Rust for the
//! [Firefly Zero](https://fireflyzero.com) handheld retro gaming
//! console.
//!
//! Includes utilities for common graphics and calculations
//! often needed in games.

#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

pub mod animation;
pub mod audio;
pub mod bar;
pub mod camera;
pub mod image_text;
pub mod movable;
pub mod particles;
pub mod point_math;
pub mod rectangle;
pub mod rising_text;
pub mod text_box;
pub mod tile;
pub mod tile_map;
pub mod utilities;
pub mod vec2;
pub mod vector;
