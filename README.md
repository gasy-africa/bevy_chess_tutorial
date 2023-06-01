# bevy_chess_tutorial

:round_pushpin: [Init](.docs/INIT.md)ializing the project

## Getting started

- [ ] Add to `src/main.rs`

```rust
use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .run();
}
```

## Changing Window settings

* Add the `Msaa` resource (default is 4)

```rust
    // Set antialiasing to use 4 samples
    .insert_resource(Msaa::default())
```
* Change the `DefaultPlugin` to add the `WindowPlugin`:

- the import 

```rust
use bevy::{
    prelude::*,
    window::{PresentMode},
};
```

- default plugin 

```rust
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
```

## Adding a camera and a plane

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(8.0).into()),
        material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
```

```rust
fn main() {
    App::new()
    // Set antialiasing to use 4 samples
    .insert_resource(Msaa::default())
    // Set WindowDescriptor Resource to change title and size
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
    .add_startup_system(setup)
    .run();
}
```

## [Making a game board](https://caballerocoll.com/blog/bevy-chess-tutorial/#:~:text="Making a game board")

note: `#[warn(unused_variables)]` on by default 
      prefix it with an underscore: `_commands`

```rust
fn create_board(
    mut _commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
}
```

# References

- [ ] [Chess game in Rust using Bevy](https://caballerocoll.com/blog/bevy-chess-tutorial)
- [ ] [bevy/examples/window
/window_settings.rs](https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs)
- [ ] [3D / 3d_scene](https://bevyengine.org/examples/3d/3d-scene/)
