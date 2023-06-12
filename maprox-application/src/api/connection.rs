use bevy::{ecs::prelude::Resource, prelude::*};
use maprox_api::handle::{MaproxHandle, MAPROX_CONNECTION_URL};
use std::ops::{Deref, DerefMut};

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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Handle<StandardMaterial>>,
) {
    use crate::api::events::*;
    use maprox_api::events::Event as MaproxApiEvent;

    if connection.connected_peers_count() == 0 {
        return;
    }

    for event in connection.receive_event().into_iter() {
        match event {
            MaproxApiEvent::Increment => info!("Incrementing!"),
            MaproxApiEvent::RenderGeometry(geometry) => {
                render_geometry(geometry, &mut commands, &mut meshes, &mut materials)
            }
            MaproxApiEvent::RefreshColors => refresh_colors(&mut query, &mut materials),
        }
    }
}
