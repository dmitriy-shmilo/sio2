use crate::{
    FIELD_WIDTH,
    FIELD_HEIGHT
};

use bevy::prelude::*;
use std::ops::{ Index, IndexMut };

pub struct Grid {
    particles: Vec<Option<Entity>>
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            particles: vec![None; FIELD_WIDTH * FIELD_HEIGHT]
        }
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = Option<Entity>;
    fn index(&self, idx: (i32, i32)) -> &Option<Entity> {

        let x = wrap(idx.0, 0, FIELD_WIDTH as i32) as usize;
        let y = wrap(idx.1, 0, FIELD_HEIGHT as i32) as usize;

        &self.particles[y * FIELD_HEIGHT + x]
    }
}

impl IndexMut<(i32, i32)> for Grid {
    fn index_mut(&mut self, idx: (i32, i32)) -> &mut Option<Entity> {

        let x = wrap(idx.0, 0, FIELD_WIDTH as i32) as usize;
        let y = wrap(idx.1, 0, FIELD_HEIGHT as i32) as usize;

        &mut self.particles[y * FIELD_HEIGHT + x]
    }
}

// Fits `i` into `lower..upper` range, wrapping its value around
// if necessary.
// See https://stackoverflow.com/a/707426/575979
#[inline]
fn wrap(mut i: i32, lower: i32, upper: i32) -> i32 {
    let range_size = upper - lower;

    if i < lower {
        i += range_size * ((lower - i) / range_size + 1);
    }

    lower + (i - lower) % range_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_wrap() {
        assert_eq!(wrap(-1, 0, 100), 99);
    }

    #[test]
    fn test_positive_wrap() {
        assert_eq!(wrap(100, 0, 100), 0);
    }

    #[test]
    fn test_no_wrap() {
        assert_eq!(wrap(10, 0, 100), 10);
    }
}