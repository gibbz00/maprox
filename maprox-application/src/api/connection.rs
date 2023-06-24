use bevy::{ecs::prelude::Resource, prelude::*};
use maprox_api::{
    events::Event as MaproxEvent,
    handle::{MaproxHandle, MAPROX_CONNECTION_URL},
};
use std::ops::{Deref, DerefMut};

use super::events::{refresh_colors, render_geometries::render_geometry};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Connection(MaproxHandle::new(MAPROX_CONNECTION_URL)));
        app.add_system(register_peers);
        app.add_system(receive_events);
    }
}

#[derive(Resource)]
struct Connection(MaproxHandle);

impl Deref for Connection {
    type Target = MaproxHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn register_peers(connection: ResMut<Connection>) {
    connection
        .register_peers()
        .expect("Connection to signaling server.");
}

fn receive_events(
    connection: ResMut<Connection>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut materials_query: Query<&mut Handle<StandardMaterial>>,
) {
    if connection.connected_peers_count() == 0 {
        return;
    }

    let mut bevy_spawn_structs = super::events::BevySpawnStructs {
        commands,
        meshes,
        materials,
    };

    for event in connection.receive_event().into_iter() {
        match event {
            // TEMP: used for initial prototyping
            MaproxEvent::Increment => info!("Incrementing!"),
            // TEMP: used for initial prototyping
            MaproxEvent::RefreshColors => {
                refresh_colors(&mut materials_query, &mut bevy_spawn_structs.materials)
            }
            MaproxEvent::RenderGeometry((geometry, color)) => {
                render_geometry(geometry, &color, &mut bevy_spawn_structs)
            }
        }
    }
}
