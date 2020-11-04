use crate::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    FIELD_WIDTH_F32,
    FIELD_HEIGHT_F32
};

use crate::{
    grid::Grid,
    physics::Particle,
    Pixel
};

use bevy::{
    prelude::*
};

pub struct GridTexture;

pub fn grid_render(
    grid: Res<Grid>,
    materials: Res<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    particle_query: Query<(&Pixel, &Particle)>,
    mut texture_query: Query<(&GridTexture, &Handle<ColorMaterial>)>) {

    let mut handle = Handle::<Texture>::new();
    for (_, material) in &mut texture_query.iter() {
        if let Some(material) = materials.get(material) {
            if let Some(texture_handle) = material.texture {
                handle = texture_handle;
                break;
            }
        }
    }

    let field_texture = textures.get_mut(&handle).unwrap();

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let offset = (x + (FIELD_HEIGHT - y - 1) * FIELD_WIDTH) * 4;

            if let Some(entity) = grid[x][y] {
                if let Ok(pix) = particle_query.get::<Pixel>(entity) {
                    field_texture.data[offset] = pix.r;
                    field_texture.data[offset + 1] = pix.g;
                    field_texture.data[offset + 2] = pix.b;
                    field_texture.data[offset + 3] = pix.a;
                }
            } else {
                field_texture.data[offset] = 255;
                field_texture.data[offset + 1] = 255;
                field_texture.data[offset + 2] = 255;
                field_texture.data[offset + 3] = 120;
            }
        }
    }
}

pub fn grid_scale(windows: Res<Windows>,
    mut query: Query<(&Sprite, &mut Transform)>) {

    // TODO: don't run if window wasn't resized
    let window = windows.get_primary().unwrap();
    let scale = if window.width < window.height {
        window.width as f32 / FIELD_WIDTH_F32
    } else {
        window.height as f32 / FIELD_HEIGHT_F32
    };

    for (_, mut trans) in &mut query.iter() {
        trans.set_scale(scale);
    }
}