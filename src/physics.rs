use crate::{
    FIELD_WIDTH_F32,
    FIELD_HEIGHT_F32,
    grid::Grid
};
use bevy::prelude::*;
use lazy_static::lazy_static;

#[derive(PartialEq)]
pub enum Behavior {
    Static,
    Solid,
    Liquid
}

pub struct Particle {
    pub behavior: Behavior,
    pub v: Vec2
}

pub struct Position {
    pub pos: Vec2
}

lazy_static! {
    static ref GRAVITY: Vec2 = Vec2::new(0., -2. / 60.);
    static ref MAX_V: Vec2 = Vec2::new(0., 8.);
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            pos: Vec2::new(x, y)
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            behavior: Behavior::Static,
            v: Vec2::zero()
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            pos: Vec2::new(f32::NAN, f32::NAN)
        }
    }
}

pub fn grid_update(mut grid: ResMut<Grid>,
    mut particles: Query<(&mut Particle, &mut Position, Entity)>) {
    for (mut particle, mut position, entity) in particles.iter_mut() {

        if particle.behavior == Behavior::Static {
            continue;
        }

        // floating point position for smooth falling
        let mut pos = position.pos;
        let mut v = particle.v;

        // source grid coordinates
        let (sx, sy) = (pos.x().round() as i32, pos.y().round() as i32);

        v += *GRAVITY;

        if v.y() < -MAX_V.y() {
            v.set_y(-MAX_V.y());
        } else if v.y() > MAX_V.y() {
            v.set_y(MAX_V.y());
        }

        pos += particle.v;

        // target grid coordinates, where the particle wants to go
        let (_tx, ty) = (pos.x().round() as i32, pos.y().round() as i32);
        // current grid coordinates, where the particle will actually end up
        let (mut cx, mut cy) = (sx, ty);

        // fall down, but check if there are any obstacles on the way
        // TODO: implement upward movement
        let mut obstacle = None;
        for y in (ty..sy).rev() {
            if let Some(e) = grid[(sx, y)] {
                obstacle = Some(e);
                cy = y + 1;
                break;
            }
        }

        if obstacle != None {
            // attempt to drop diagonally
            if grid[(cx - 1, cy - 1)] == None {
                cx -= 1;
                cy -= 1;
            } else if grid[(cx + 1, cy - 1)] == None {
                cx += 1;
                cy -= 1;
            } else if particle.behavior == Behavior::Liquid {
                // TODO: find a way to read and modify obstacle's velocity
                v = Vec2::zero();
                
                // liquids can attempt to move sideways
                if grid[(cx - 1, cy)] == None {
                    cx -= 1;
                } else if grid[(cx + 1, cy)] == None {
                    cx += 1;
                }
            } else {
                // TODO: find a way to read and modify obstacle's velocity
                v = Vec2::zero();
            }

            pos.set_x(cx as f32);
            pos.set_y(cy as f32);
        } else {
            if pos.x() < 0.0 {
                pos.set_x(FIELD_WIDTH_F32 + pos.x());
            } else if pos.x() > FIELD_WIDTH_F32 {
                pos.set_x(pos.x() - FIELD_WIDTH_F32);
            }

            if pos.y() < 0.0 {
                pos.set_y(FIELD_HEIGHT_F32 + pos.y());
            } else if pos.y() > FIELD_HEIGHT_F32 {
                pos.set_y(pos.y() - FIELD_HEIGHT_F32);
            }
        }

        particle.v = v;
        position.pos = pos;
        grid[(sx, sy)] = None;
        grid[(cx, cy)] = Some(entity);
    }
}