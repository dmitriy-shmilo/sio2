use crate::{
    FIELD_WIDTH_F32,
    FIELD_HEIGHT_F32,
    physics::{ 
        Particle,
        Behavior
    },
    grid::Grid
};
use bevy::{
    input::{
        ElementState,
        mouse::MouseButtonInput,
        keyboard::KeyboardInput
    },
    prelude::*
};
use rand::seq::SliceRandom;
use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq)]
pub enum Tool {
    None,
    Concrete,
    Sand,
    Water,
    Eraser
}

#[derive(Default)]
pub struct InputState {
    mouse_button: EventReader<MouseButtonInput>,
    mouse_move: EventReader<CursorMoved>,
    keyboard: EventReader<KeyboardInput>
}

pub struct ToolState {
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

pub fn handle_input(
    mut tool_state: ResMut<ToolState>,
    mut input: ResMut<InputState>,
    windows: Res<Windows>,
    cursor_moved: Res<Events<CursorMoved>>,
    mouse_button: Res<Events<MouseButtonInput>>,
    key_pressed: Res<Events<KeyboardInput>>) {

    let window = windows.get_primary().unwrap();
    let scale = if window.width() < window.height() {
        window.width() as f32 / FIELD_WIDTH_F32
    } else {
        window.height() as f32 / FIELD_HEIGHT_F32
    };
    let (gw, gh) = (scale * FIELD_WIDTH_F32, scale * FIELD_HEIGHT_F32);
    let (pw, ph) = ((window.width() as f32 - gw) / 2.,
        (window.height() as f32 - gh) / 2.);

    let (left, top, right, bottom) = (
        pw,
        window.height() as f32 - ph,
        window.width() as f32 - pw,
        ph
    );

    for event in input.mouse_button.iter(&mouse_button) {
        tool_state.is_spawning = event.state == ElementState::Pressed;
    }

    for event in input.mouse_move.iter(&cursor_moved) {
        let x = event.position.x();
        let y = event.position.y();
        if x > left && x < right && y > bottom && y < top {
            let x = ((event.position.x() - left) / scale) as usize;
            let y = ((event.position.y() - bottom) / scale) as usize;
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
                Some(k) if k == KeyCode::Key0 => Tool::Eraser,
                _ => Tool::None
            }
        }
    }
}

pub fn spawn_particle(mut commands: Commands,
    mut grid: ResMut<Grid>,
    tool: Res<ToolState>) {
    if tool.is_spawning {
        let (x, y) = (tool.grid_x as i32, tool.grid_y as i32);
        if tool.current_tool == Tool::Eraser {
            if let Some(entity) = grid[(x, y)] {
                commands.despawn(entity);
                grid[(x, y)] = None;
            }
            return;
        }

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
            get_color(tool.current_tool),
            x,
            y);
    }
}

lazy_static!{
    static ref SAND_COLORS: Vec<Color> = vec![
        Color::rgb(0.96, 0.74, 0.53),
        Color::rgb(0.95, 0.89, 0.5),
        Color::rgb(0.92, 0.91, 0.72)
    ];

    static ref WATER_COLORS: Vec<Color> = vec![
        Color::rgb(0.21, 0.58, 0.74),
        Color::rgb(0.3, 0.67, 0.81)
    ];
}

fn get_color(tool: Tool) -> Color {

    let mut rng = rand::thread_rng();
    match tool {
        Tool::Concrete => Color::rgb(0.58, 0.6, 0.59),
        Tool::Water => *WATER_COLORS.choose(&mut rng).unwrap(),
        _ => *SAND_COLORS.choose(&mut rng).unwrap()
    }
}

fn add_particle(commands: &mut Commands,
    grid: &mut Grid,
    particle: Particle,
    color: Color,
    x: i32,
    y: i32) {
    if grid[(x, y)].is_some() {
        return;
    }

    commands
        .spawn((color, particle));

    grid[(x, y)] = commands.current_entity();
}