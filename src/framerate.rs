use bevy::{
    diagnostic::{ FrameTimeDiagnosticsPlugin, Diagnostics },
    prelude::*
};

pub fn display_framerate(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}