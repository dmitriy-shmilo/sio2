
use bevy::{
    diagnostic::{ FrameTimeDiagnosticsPlugin, Diagnostics },
    input::mouse::{ MouseButtonInput },
    input::keyboard::{ KeyboardInput, ElementState },
    prelude::*,
    render::texture::{ TextureFormat }
};

use std::ops::{ Index, IndexMut };

const WINDOW_HEIGHT : u32 = 800;
const WINDOW_WIDTH : u32 = 800;
const FIELD_WIDTH : usize = 200;
const FIELD_HEIGHT : usize = 200;
const FIELD_WIDTH_F32 : f32 = FIELD_WIDTH as f32;
const FIELD_HEIGHT_F32 : f32 = FIELD_HEIGHT as f32;

#[derive(PartialEq)]
enum Behavior {
    Static,
    Solid,
    Liquid
}

#[derive(PartialEq)]
enum Tool {
    None,
    Concrete,
    Sand,
    Water
}

#[derive(Default)]
struct InputState {
    mouse_button: EventReader<MouseButtonInput>,
    mouse_move: EventReader<CursorMoved>,
    keyboard: EventReader<KeyboardInput>
}

struct ToolState {
    current_tool: Tool,
    is_spawning: bool,
    grid_x: usize,
    grid_y: usize
}

impl Default for ToolState {
    fn default() -> Self {
        ToolState {
            current_tool: Tool::None,
            is_spawning: false,
            grid_x: 0,
            grid_y: 0
        }
    }
}

struct Particle {
    behavior: Behavior,
    v: Vec2, // currently only used by liquids to track last direction,
    is_moved: bool // track if particle has moved this frame
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

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel {
            r,
            g,
            b,
            a
        }
    }
}

struct GridTexture;

struct Grid {
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

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "SiO2".to_string(),
            width: WINDOW_HEIGHT,
            height: WINDOW_WIDTH,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(particle_render.system())
        .add_system(particle_scale.system())
        .add_system(particle_move.system())
        .add_system(display_framerate.system())
        .add_system(handle_input.system())
        .add_system(spawn_particle.system())
        .run();
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>) {

    let font = asset_server
        .load("assets/fonts/FiraSans-Bold.ttf")
        .unwrap();

    let texture = Texture::new_fill(
        Vec2::new(FIELD_WIDTH_F32, FIELD_HEIGHT_F32),
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm
    );
    let th = textures.add(texture);

    commands.spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: font,
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .insert_resource(Grid::default())
        .insert_resource(ToolState::default())
        .insert_resource(InputState::default())
        .spawn(SpriteComponents {
            material: materials.add(th.into()),
            transform: Transform::from_translation_rotation_scale(
                Vec3::new(0., 0., 0.),
                Quat::identity(),
                WINDOW_WIDTH as f32 / FIELD_WIDTH_F32
            ),
            ..Default::default()
        })
        .with(GridTexture);
}

fn particle_move(mut grid: ResMut<Grid>,
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

fn particle_render(
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

fn particle_scale(windows: Res<Windows>,
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

fn handle_input(
    mut tool_state: ResMut<ToolState>,
    mut input: ResMut<InputState>,
    windows: Res<Windows>,
    cursor_moved: Res<Events<CursorMoved>>,
    mouse_button: Res<Events<MouseButtonInput>>,
    key_pressed: Res<Events<KeyboardInput>>) {

    let window = windows.get_primary().unwrap();
    let scale = if window.width < window.height {
        window.width as f32 / FIELD_WIDTH_F32
    } else {
        window.height as f32 / FIELD_HEIGHT_F32
    };
    let (gw, gh) = (scale * FIELD_WIDTH_F32, scale * FIELD_HEIGHT_F32);
    let (pw, ph) = ((window.width as f32 - gw) / 2.,
        (window.height as f32 - gh) / 2.);

    let (left, top, right, bottom) = (
        pw,
        window.height as f32 - ph,
        window.width as f32 - pw,
        ph
    );

    for event in input.mouse_button.iter(&mouse_button) {
        tool_state.is_spawning = event.state == ElementState::Pressed;
    }

    for event in input.mouse_move.iter(&cursor_moved) {
        let x = event.position.x();
        let y = event.position.y();
        if x > left && x < right && y > bottom && y < top {
            let x = (((event.position.x() - left) / scale)) as usize;
            let y = (((event.position.y() - bottom) / scale)) as usize;
            tool_state.grid_x = x;
            tool_state.grid_y = y;
        } else {
            tool_state.is_spawning = false;
        }
    }

    for event in input.keyboard.iter(&key_pressed) {
        if event.state.is_pressed() {
            tool_state.current_tool = match event.key_code {
                Some(k) if k == KeyCode::Key1 => Tool::Concrete,
                Some(k) if k == KeyCode::Key2 => Tool::Sand,
                Some(k) if k == KeyCode::Key3 => Tool::Water,
                _ => Tool::None
            }
        }
    }
}

fn spawn_particle(mut commands: Commands,
    mut grid: ResMut<Grid>,
    tool: Res<ToolState>) {
    if tool.is_spawning {
        add_particle(&mut commands,
            &mut grid,
            Particle { behavior: 
                match tool.current_tool {
                    Tool::Concrete => Behavior::Static,
                    Tool::Water => Behavior::Liquid,
                    _ => Behavior::Solid
                },
                ..Default::default()
            },
            match tool.current_tool {
                Tool::Concrete => Pixel::new(210, 204, 204, 255),
                Tool::Water => Pixel::new(73, 153, 230, 255),
                _ => Pixel::new(204, 153, 73, 255)
            },
            tool.grid_x,
            tool.grid_y
            );
    }
}

fn display_framerate(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn add_particle(commands: &mut Commands,
    grid: &mut Grid,
    particle: Particle,
    pixel: Pixel,
    x: usize,
    y: usize) {
    if let Some(_) = grid[x][y] {
        return;
    }

    commands
        .spawn((pixel, particle));

    grid[x][y] = commands.current_entity();
}