use crate::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    grid::Grid
};

use bevy::prelude::*;

#[derive(PartialEq)]
pub enum Behavior {
    Static,
    Solid,
    Liquid
}

pub struct Particle {
    pub behavior: Behavior,
    pub v: Vec2, // currently only used by liquids to track last direction,
    pub is_moved: bool // track if particle has moved this frame
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            behavior: Behavior::Static,
            v: Vec2::zero(),
            is_moved: false
        }
    }
}

pub fn particle_move(mut grid: ResMut<Grid>,
    particles: Query<&mut Particle>) {

    // TODO: replace this with double-buffered grid
    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            if let Some(entity) = grid[x][y] {
                if let Ok(mut particle) = particles.get_mut::<Particle>(entity) {
                    particle.is_moved = false;
                }
            }
        }
    }

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            if let Some(entity) = grid[x][y] {
                if let Ok(mut particle) = particles.get_mut::<Particle>(entity) {
                    if particle.behavior == Behavior::Static
                        || particle.is_moved {
                        continue;
                    }

                    if y > 0 {
                        // TODO: use Particle.v to decide next cell to occupy
                        let (mut nx, mut ny) = (x, y);

                        if grid[x][y - 1] == None {
                            ny = y - 1;
                            particle.v.set_x(0.);
                        } else if x > 0
                            && grid[x - 1][y - 1] == None {
                            ny = y - 1;
                            nx = x - 1;
                            particle.v.set_x(0.);
                        } else if x < FIELD_WIDTH - 1
                            && grid[x + 1][y - 1] == None {
                            ny = y - 1;
                            nx = x + 1;
                            particle.v.set_x(0.);
                        } else if particle.behavior == Behavior::Liquid {
                            // liquids can shift to the side if bottom cells are busy
                            if x > 0
                                && grid[x - 1][y] == None
                                && particle.v.x() <= 0. {
                                nx = x - 1;
                                particle.v.set_x(-1.);
                            } else if x < FIELD_WIDTH - 1
                                && grid[x + 1][y] == None {
                                nx = x + 1;
                                particle.v.set_x(1.);
                            }
                        }

                        if x != nx || y != ny {
                            particle.is_moved = true;
                            grid[x][y] = None;
                            grid[nx][ny] = Some(entity);
                        }
                    }
                }
            }
        }
    }
}