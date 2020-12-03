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
    mut query: Query<(&mut Text, &mut Style, &FpsState)>) {

    let mut average_fps = None;
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        average_fps = fps.average();
    }

    for (mut text, mut style, state) in &mut query.iter_mut() {
        style.display = Display::None;
        // TODO: setting font size is a temporary hack
        text.style.font_size = 0.;
        if let Some(average_fps) = average_fps {
            text.value = format!("FPS: {:.2}", average_fps);
            if state.is_visible {
                style.display = Display::Flex;
                text.style.font_size = 30.;
            } else {
                style.display = Display::None;
                text.style.font_size = 0.;
            }
        }
    }
}