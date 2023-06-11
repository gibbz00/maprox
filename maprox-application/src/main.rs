use bevy::prelude::*;
use maprox_application::{api::connection::ConnectionPlugin, map_camera::MapCameraPlugin};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                // Don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ConnectionPlugin)
        .add_plugin(MapCameraPlugin)
        .run();
}
