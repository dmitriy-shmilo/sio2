use bevy::{
    diagnostic::{ FrameTimeDiagnosticsPlugin, Diagnostics },
    input::mouse::{ MouseButtonInput },
    input::keyboard::{ KeyboardInput, ElementState },
    prelude::*
};
use std::ops::{ Index, IndexMut };

const FIELD_WIDTH : usize = 200;
const FIELD_HEIGHT : usize = 200;
const FIELD_WIDTH_F32 : f32 = FIELD_WIDTH as f32;
const FIELD_HEIGHT_F32 : f32 = FIELD_HEIGHT as f32;

#[derive(PartialEq)]
enum Behavior {
    Static,
    Solid
}

#[derive(PartialEq)]
enum Tool {
    None,
    Concrete,
    Sand
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

struct SandMaterial(Handle<ColorMaterial>);
struct WaterMaterial(Handle<ColorMaterial>);
struct ConcreteMaterial(Handle<ColorMaterial>);

struct Particle {
    behavior: Behavior
}

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
            width: 800,
            height: 800,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(particle_translate.system())
        .add_system(particle_scale.system())
        .add_system(particle_move.system())
        .add_system(display_framerate.system())
        .add_system(handle_input.system())
        .add_system(spawn_particle.system())
        .run();
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    let font = asset_server
        .load("assets/fonts/FiraSans-Bold.ttf")
        .unwrap();

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
        .insert_resource(SandMaterial(
            materials.add(Color::rgb(0.8, 0.6, 0.3).into())
        ))
        .insert_resource(ConcreteMaterial(
            materials.add(Color::rgb(0.7, 0.8, 0.8).into())
        ))
        .insert_resource(WaterMaterial(
            materials.add(Color::rgb(0.3, 0.6, 0.9).into())
        ))
        .insert_resource(Grid::default())
        .insert_resource(ToolState::default())
        .insert_resource(InputState::default());
}

fn particle_move(mut grid: ResMut<Grid>,
    particles: Query<&Particle>) {

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            if let Some(entity) = grid[x][y] {
                if let Ok(particle) = particles.get::<Particle>(entity) {
                    if particle.behavior == Behavior::Static {
                        continue;
                    }

                    if y > 0 {
                        let new_y = y - 1;

                        if grid[x][new_y] == None {
                            grid[x][y] = None;
                            grid[x][new_y] = Some(entity);
                        } else if x > 0 && grid[x - 1][new_y] == None {
                            grid[x][y] = None;
                            grid[x - 1][new_y] = Some(entity);
                        } else if x < FIELD_WIDTH - 1 && grid[x + 1][new_y] == None {
                            grid[x][y] = None;
                            grid[x + 1][new_y] = Some(entity);
                        }
                    }
                }
            }
        }
    }
}

fn particle_translate(windows: Res<Windows>, grid: Res<Grid>, particles: Query<(&Particle, &mut Transform)>) {
    fn convert(p: f32, bound_window: f32, bound_game: f32) -> f32 {
        p / bound_game * bound_window - (bound_window / 2.)
    }

    let window = windows.get_primary().unwrap();

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            if let Some(particle) = grid[x][y] {
                if let Ok(mut transform) = particles.get_mut::<Transform>(particle) {
                    transform.set_translation(Vec3::new(
                        convert(x as f32, window.width as f32, FIELD_WIDTH as f32),
                        convert(y as f32, window.height as f32, FIELD_HEIGHT as f32),
                        0.0,
                    ))
                }
            }
        }
    }
}

fn particle_scale(windows: Res<Windows>,
    mut particles: Query<(&Particle, &mut Sprite)>) {

    // TODO: don't run if window wasn't resized
    let window = windows.get_primary().unwrap();

    for (_, mut sprite) in &mut particles.iter() {
        sprite.size = Vec2::new(
            window.width as f32 / FIELD_WIDTH as f32,
            window.height as f32 / FIELD_HEIGHT as f32
        );
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

    for event in input.mouse_button.iter(&mouse_button) {
        tool_state.is_spawning = event.state == ElementState::Pressed;
    }

    for event in input.mouse_move.iter(&cursor_moved) {
        let x = ((event.position.x() / window.width as f32) * FIELD_WIDTH_F32) as usize;
        let y = ((event.position.y() / window.height as f32) * FIELD_HEIGHT_F32) as usize;
        tool_state.grid_x = x;
        tool_state.grid_y = y;
    }

    for event in input.keyboard.iter(&key_pressed) {
        if event.state.is_pressed() {
            tool_state.current_tool = match event.key_code {
                Some(k) if k == KeyCode::Key1 => Tool::Concrete,
                Some(k) if k == KeyCode::Key2 => Tool::Sand,
                _ => Tool::None
            }
        }
    }
}

fn spawn_particle(mut commands: Commands,
    mut grid: ResMut<Grid>,
    tool: Res<ToolState>,
    (concrete, sand): (Res<ConcreteMaterial>, Res<SandMaterial>)) {

    if tool.is_spawning {
        add_particle(&mut commands,
            &mut grid,
            Particle { behavior: 
                match tool.current_tool {
                    Tool::Concrete => Behavior::Static,
                    _ => Behavior::Solid
                }
            },
            match tool.current_tool {
                Tool::Concrete => concrete.0,
                _ => sand.0
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
    material: Handle<ColorMaterial>,
    x: usize,
    y: usize) {
    if let Some(_) = grid[x][y] {
        return;
    }

    commands
        .spawn(SpriteComponents {
            material: material,
            ..Default::default()
        })
        .with(particle);

    grid[x][y] = commands.current_entity();
}