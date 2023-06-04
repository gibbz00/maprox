use bevy::{prelude::*, tasks::IoTaskPool};
use maprox_common::{Event as MaproxEvent, MaproxConnection, MAPROX_CONNECTION_URL};
use matchbox_socket::WebRtcSocket;
use std::ops::{Deref, DerefMut};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (socket, message_loop_fut) = WebRtcSocket::new_reliable(MAPROX_CONNECTION_URL);
        let task_pool = IoTaskPool::get();
        task_pool.spawn(message_loop_fut).detach();

        app.insert_resource(Connection(MaproxConnection::new(socket)));

        app.add_system(register_peers);
        app.add_system(emit_events);
        app.add_system(receive_events);
    }
}

#[derive(Resource)]
struct Connection(MaproxConnection);

impl Deref for Connection {
    type Target = MaproxConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn register_peers(mut connection: ResMut<Connection>) {
    connection.register_peers();
}

fn emit_events(mut connection: ResMut<Connection>) {
    connection.send_event(MaproxEvent::Increment);
}

fn receive_events(mut connection: ResMut<Connection>) {
    if connection.connected_peers_count() == 0 {
        return;
    }

    for event in connection.receive_event().iter() {
        match event {
            MaproxEvent::Increment => info!("Incrementing!"),
        }
    }
}
