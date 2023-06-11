use bevy::prelude::*;
use maprox_application::{api::connection::ConnectionPlugin, map_camera::MapCameraPlugin};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(ConnectionPlugin)
        .add_plugin(MapCameraPlugin)
        .run();
}
