
use bevy::{
    prelude::*,
    window::{PresentMode},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum VictoryOrDefeat {
	VICTORY,
	NONE,
	DEFEAT
}

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
    .add_startup_system(setup)
    .add_startup_system(create_board)
    .run();
}

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

fn create_board(
    mut _commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
}