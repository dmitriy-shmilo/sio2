use crate::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    FIELD_WIDTH_F32,
    FIELD_HEIGHT_F32
};
use crate::{
    grid::Grid,
    physics::Particle,
    input::{ Tool, ToolState },
    util::wrap
};
use bevy::prelude::*;
use lazy_static::lazy_static;
use std::cmp::max;

lazy_static! {
    static ref BACKGROUND_COLOR: Color = Color::rgb(0.11, 0.11, 0.11);
}
const BRUSH_SHADOW_MOD: u8 = 50;

pub struct GridTexture;

pub fn grid_render(
    grid: Res<Grid>,
    tool: Res<ToolState>,
    materials: Res<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    particle_query: Query<(&Color, &Particle)>,
    texture_query: Query<(&GridTexture, &Handle<ColorMaterial>)>) {

    let mut handle = None;
    for (_, material) in &mut texture_query.iter() {
        if let Some(material) = materials.get(material) {
            if let Some(texture_handle) = &material.texture {
                handle = Some(texture_handle);
                break;
            }
        }
    }

    let field_texture = textures.get_mut(handle.unwrap()).unwrap();

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let offset = (x + (FIELD_HEIGHT - y - 1) * FIELD_WIDTH) * 4;

            // TODO: grid[(x, y)] wraps coordinates, which is useless in this scenario
            if let Some(entity) = grid[(x as i32, y as i32)] {
                if let Ok(entity) = particle_query.get(entity) {
                    let pix = entity.0;
                    field_texture.data[offset] = (pix.r() * 255.99) as u8;
                    field_texture.data[offset + 1] = (pix.g() * 255.99) as u8;
                    field_texture.data[offset + 2] = (pix.b() * 255.99) as u8;
                    field_texture.data[offset + 3] = (pix.a() * 255.99) as u8;
                }
            } else {
                field_texture.data[offset] = (BACKGROUND_COLOR.r() * 255.99) as u8;
                field_texture.data[offset + 1] = (BACKGROUND_COLOR.g() * 255.99) as u8;
                field_texture.data[offset + 2] = (BACKGROUND_COLOR.b() * 255.99) as u8;
                field_texture.data[offset + 3] = (BACKGROUND_COLOR.a() * 255.99) as u8;
            }
        }
    }

    if tool.current_tool != Tool::None {
        let (cx, cy) = (tool.grid_x as i32, tool.grid_y as i32);

        for x in cx - tool.tool_size..=cx + tool.tool_size {
            for y in cy - tool.tool_size..=cy + tool.tool_size {
                let x = wrap(x, 0, FIELD_WIDTH as i32) as usize;
                let y = wrap(y, 0, FIELD_HEIGHT as i32) as usize;
                let offset = (x + (FIELD_HEIGHT - y - 1) * FIELD_WIDTH) * 4;

                for o in offset..offset + 3 {
                    field_texture.data[o] = max(0,
                        field_texture.data[o] - BRUSH_SHADOW_MOD);
                }
            }
        }
    }
}

pub fn grid_scale(windows: Res<Windows>,
    mut query: Query<(&Sprite, &mut Transform)>) {

    // TODO: don't run if window wasn't resized
    let window = windows.get_primary().unwrap();
    let scale = if window.width() < window.height() {
        window.width() as f32 / FIELD_WIDTH_F32
    } else {
        window.height() as f32 / FIELD_HEIGHT_F32
    };

    for (_, mut trans) in &mut query.iter_mut() {
        trans.scale = Vec3::splat(scale);
    }
}