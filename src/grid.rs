use crate::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    util::wrap
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

impl Index<usize> for Grid {
    type Output = [Option<Entity>];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.particles[idx * FIELD_HEIGHT..idx * FIELD_HEIGHT + FIELD_WIDTH]
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = Option<Entity>;
    fn index(&self, idx: (i32, i32)) -> &Self::Output {

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