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
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Handle<StandardMaterial>>,
) {
    use crate::api::{events::render_geometries::*, BevySpawnStructs};
    use maprox_api::events::Event as MaproxApiEvent;

    if connection.connected_peers_count() == 0 {
        return;
    }

    let mut bevy_spawn_structs = BevySpawnStructs {
        commands,
        meshes,
        materials,
    };

    for event in connection.receive_event().into_iter() {
        match event {
            // TEMP: used for initial prototyping
            MaproxApiEvent::Increment => info!("Incrementing!"),
            MaproxApiEvent::RefreshColors => {
                refresh_colors(&mut query, &mut bevy_spawn_structs.materials)
            }
            MaproxApiEvent::RenderPoint((point, color)) => {
                render_point(point, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderMultiPoint((multi_point, color)) => {
                render_multi_point(multi_point, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderLine((line, color)) => {
                render_line(line, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderLineString((line_string, color)) => {
                render_line_string(line_string, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderMultiLineString((multi_linestring, color)) => {
                render_multi_linestring(multi_linestring, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderPolygon((polygon, color)) => {
                render_polygon(polygon, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderMultiPolygon((multi_polygon, color)) => {
                render_multi_polygon(multi_polygon, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderRect((rect, color)) => {
                render_rect(rect, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderGeometry((geometry, color)) => {
                render_geometry(geometry, &color, &mut bevy_spawn_structs)
            }
            MaproxApiEvent::RenderGeometryCollection((geometry_collection, color)) => {
                render_geometry_collection(geometry_collection, &color, &mut bevy_spawn_structs)
            }
        }
    }
}
