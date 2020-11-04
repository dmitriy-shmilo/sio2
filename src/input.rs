use crate::{
    FIELD_WIDTH_F32,
    FIELD_HEIGHT_F32
};

use crate::{
    physics::{ 
        Particle,
        Behavior
    },
    grid::Grid,
    Pixel
};

use bevy::{
    input::mouse::{ MouseButtonInput },
    input::keyboard::{ KeyboardInput, ElementState },
    prelude::*
};

#[derive(PartialEq)]
pub enum Tool {
    None,
    Concrete,
    Sand,
    Water
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
                _ => Tool::None
            }
        }
    }
}

pub fn spawn_particle(mut commands: Commands,
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
            tool.grid_x as i32,
            tool.grid_y as i32
            );
    }
}


fn add_particle(commands: &mut Commands,
    grid: &mut Grid,
    particle: Particle,
    pixel: Pixel,
    x: i32,
    y: i32) {
    if grid[(x, y)].is_some() {
        return;
    }

    commands
        .spawn((pixel, particle));

    grid[(x, y)] = commands.current_entity();
}