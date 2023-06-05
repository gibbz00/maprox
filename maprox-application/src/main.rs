use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use maprox_application::connection::ConnectionPlugin;
use std::f32::consts::PI;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(ConnectionPlugin)
        .add_startup_system(camera)
        .add_system(clamp_zoom)
        .run();
}

fn camera(mut commands: Commands) {
    const ORBIT_CLAMP: f32 = (PI / 2.) * 0.95;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
            ..default()
        },
        PanOrbitCamera {
            alpha_lower_limit: Some(-ORBIT_CLAMP),
            alpha_upper_limit: Some(ORBIT_CLAMP),
            beta_lower_limit: Some(-ORBIT_CLAMP),
            beta_upper_limit: Some(ORBIT_CLAMP),
            ..Default::default()
        },
    ));
}

fn clamp_zoom(mut orbit_cameras: Query<&mut PanOrbitCamera>) {
    for mut orbit_camera in orbit_cameras.iter_mut() {
        orbit_camera.radius = Some(f32::min(
            orbit_camera.radius.expect("Initialized value"),
            200.0,
        ))
    }
}
