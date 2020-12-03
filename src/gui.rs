use bevy::{
    diagnostic::{ FrameTimeDiagnosticsPlugin, Diagnostics },
    prelude::*
};

pub struct FpsState {
    pub is_visible: bool
}

impl Default for FpsState {
    fn default() -> Self {
        FpsState {
            is_visible: false
        }
    }
}

pub fn display_framerate(diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &mut Draw, &FpsState)>) {

    let mut average_fps = None;
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        average_fps = fps.average();
    }

    for (mut text, mut draw, state) in &mut query.iter_mut() {
        if let Some(average_fps) = average_fps {
            text.value = format!("FPS: {:.2}", average_fps);
            draw.is_visible = state.is_visible;
        }
    }
}