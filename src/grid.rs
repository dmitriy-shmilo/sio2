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

impl Index<usize> for Grid {
    type Output = [Option<Entity>];
    fn index(&self, idx: usize) -> &[Option<Entity>] {
        &self.particles[idx * FIELD_HEIGHT .. idx * FIELD_HEIGHT + FIELD_HEIGHT]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, idx: usize) -> &mut [Option<Entity>] {
        &mut self.particles[idx * FIELD_HEIGHT .. idx * FIELD_HEIGHT + FIELD_HEIGHT]
    }
}