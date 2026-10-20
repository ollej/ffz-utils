//! Utilities that don't fit anywhere else.
//!
//! # Examples
//!
//! ```
//! use firefly_rust::*;
//! use ffz_utils::utilities::*;
//! let value = random_range(5, 10);
//! ```

use firefly_rust::*;

/// Generate a random number between `min` and `max`.
pub fn random_range(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    let range = max - min + 1;
    min + (get_random() % range)
}
