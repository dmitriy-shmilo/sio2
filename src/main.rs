mod grid;
mod gui;
mod input;
mod physics;
mod render;
mod util;

use crate::{
    grid::Grid,
    gui::{display_framerate, FpsState},
    input::{handle_input, spawn_particle, Tool, ToolState},
    physics::grid_update,
    render::{grid_render, grid_scale, GridTexture},
    util::window_size_to_scale,
};

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::texture::TextureFormat};

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;
const FIELD_WIDTH: usize = 200;
const FIELD_HEIGHT: usize = 200;
const TEXTURE_TYPE: TextureFormat = TextureFormat::Rgba8Unorm;
const TEXTURE_STRIDE: usize = 4;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "SiO2".to_string(),
            width: WINDOW_HEIGHT as _,
            height: WINDOW_WIDTH as _,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(grid_render.system())
        .add_system(grid_scale.system())
        .add_system(grid_update.system())
        .add_system(display_framerate.system())
        .add_system(handle_input.system())
        .add_system(spawn_particle.system())
        .run();
}
use bevy::render::texture::{Extent3d, TextureDimension};
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let texture = Texture::new_fill(
        Extent3d::new(FIELD_WIDTH as _, FIELD_HEIGHT as _, 1),
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TEXTURE_TYPE,
    );
    let th = textures.add(texture);

    let scale = Vec3::splat(window_size_to_scale(
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
    ));

    commands
        // .spawn(Camera2dComponents::default())
        .spawn_bundle(UiCameraBundle::default())
        .commands();
    // .spawn_bundle(TextBundle {
    //     style: Style {
    //         align_self: AlignSelf::FlexEnd,
    //         ..Default::default()
    //     },
    //     text: Text {
    //         value: "FPS:".to_string(),
    //         font,
    //         style: TextStyle {
    //             font_size: 30.0,
    //             color: Color::WHITE,
    //         },
    //     },
    //     ..Default::default()
    // })
    // .with(FpsState::default())
    commands.insert_resource(Grid::default());
    commands.insert_resource(ToolState {
        current_tool: Tool::Sand,
        ..Default::default()
    });
    // .insert_resource(InputState::default())
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(th.into()),
            transform: Transform {
                scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GridTexture);
}
