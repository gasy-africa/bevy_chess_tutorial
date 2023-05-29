
use bevy::{
    prelude::*,
    window::{PresentMode},
};

fn main() {
    App::new()
    // Set antialiasing to use 4 samples
    .insert_resource(Msaa::default())
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Chess!".into(),
            resolution: (1600., 1600.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .run();
}