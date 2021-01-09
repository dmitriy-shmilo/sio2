use crate::{
    grid::Grid,
    input::{Tool, ToolState},
    physics::Particle,
    util::wrap,
};
use crate::{window_size_to_scale, FIELD_HEIGHT, FIELD_WIDTH};
use bevy::{prelude::*, window::WindowResized};
use lazy_static::lazy_static;

lazy_static! {
    static ref BACKGROUND_COLOR: Color = Color::rgb(0.11, 0.11, 0.11);
}

pub struct GridTexture;

pub fn grid_render(
    grid: Res<Grid>,
    tool: Res<ToolState>,
    materials: Res<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    particle_query: Query<(&Color, &Particle)>,
    texture_query: Query<(&GridTexture, &Handle<ColorMaterial>)>,
) {
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
    field_texture.data.copy_from_slice(&grid.texture.data);


    if tool.current_tool != Tool::None {
        let (cx, cy) = (tool.grid_x as i32, tool.grid_y as i32);

        for x in cx - tool.tool_size..=cx + tool.tool_size {
            for y in cy - tool.tool_size..=cy + tool.tool_size {
                let x = wrap(x, 0, FIELD_WIDTH as i32) as usize;
                let y = wrap(y, 0, FIELD_HEIGHT as i32) as usize;
                let offset = (x + (FIELD_HEIGHT - y - 1) * FIELD_WIDTH) * 4;

                for o in offset..offset + 3 {
                    field_texture.data[o] /= 2;
                }
            }
        }
    }
}

pub fn grid_scale(
    resize_event: Res<Events<WindowResized>>,
    mut query: Query<(&Sprite, &mut Transform)>,
) {
    let window_resize = resize_event
        .get_reader()
        .iter(&resize_event)
        .map(|event| (event.width, event.height))
        .last();

    if let Some((width, height)) = window_resize {
        let scale = Vec3::splat(window_size_to_scale(width, height));
        for (_, mut trans) in &mut query.iter_mut() {
            trans.scale = scale;
        }
    }
}
