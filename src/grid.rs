use crate::{util::wrap, FIELD_HEIGHT, FIELD_WIDTH};

use fxhash::FxHashMap;

use bevy::prelude::*;

pub struct Grid {
    particles: FxHashMap<(i32, i32), Entity>,
    pub xsize: usize,
    pub ysize: usize,
    pub background_color: Color,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            particles: Default::default(),
            xsize: FIELD_WIDTH,
            ysize: FIELD_HEIGHT,
            background_color: Color::rgb(0.11, 0.11, 0.11),
        }
    }
}

impl Grid {
    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32), &Entity)> + '_ {
        self.particles.iter()
    }
    fn wrap_index(&self, x: i32, y: i32) -> (i32, i32) {
        let x = wrap(x, 0, self.xsize as i32);
        let y = wrap(y, 0, self.ysize as i32);
        (x, y)
    }
    pub fn get_non_wrapping(&self, x: i32, y: i32) -> Option<Entity> {
        self.particles.get(&(x, y)).copied()
    }
    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        let (x, y) = self.wrap_index(x, y);
        self.get_non_wrapping(x, y)
    }
    pub fn set(&mut self, x: i32, y: i32, value: Option<Entity>) {
        let idx = self.wrap_index(x, y);
        if let Some(e) = value {
            self.particles.insert(idx, e);
        } else {
            self.particles.remove(&idx);
        }
    }
    pub fn remove(&mut self, x: i32, y: i32) -> Option<Entity> {
        self.particles.remove(&self.wrap_index(x, y))
    }
    pub fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        if (x1, y1) == (x2, y2) {
            return;
        }
        let e1 = self.get(x1, y1);
        let e2 = self.get(x2, y2);
        self.set(x2, y2, e1);
        self.set(x1, y1, e2);
    }
}
