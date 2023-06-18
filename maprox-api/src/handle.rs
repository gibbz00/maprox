use crate::events::Event;
use log::info;
use matchbox_socket::{PeerId, PeerState, SingleChannel, WebRtcSocket};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub const MAPROX_CONNECTION_URL: &str = env!("MAPROX_CONNECTION_URL");

/// Handle can be cloned freely as it a wrapper for an
/// `Arc<Mutex<MaproxConnection>>`
#[derive(Clone)]
pub struct MaproxHandle {
    connection: Arc<Mutex<MaproxConnection>>,
}

struct MaproxConnection {
    socket: WebRtcSocket<SingleChannel>,
    peers: HashSet<PeerId>,
}

impl MaproxHandle {
    pub fn new(url_str: &str) -> Self {
        let (socket, loop_fut) = WebRtcSocket::new_reliable(url_str);

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async { loop_fut.await.unwrap() });

        #[cfg(not(target_arch = "wasm32"))]
        {
            let executor = async_executor::Executor::new();
            let task = executor.spawn(loop_fut);
            std::thread::spawn(move || {
                futures::executor::block_on(executor.run(task))
                    .expect("Failed to init maprox_connection");
            });
        }

        MaproxHandle {
            connection: Arc::new(Mutex::new(MaproxConnection {
                socket,
                peers: HashSet::with_capacity(1),
            })),
        }
    }

    pub fn send_event(&self, event: Event) {
        let mut connection = self.connection.lock().unwrap();
        let mut payload = Vec::new();
        ciborium::ser::into_writer(&event, &mut payload).unwrap();
        for peer in connection.peers.clone() {
            connection.socket.send(payload.clone().into(), peer);
        }
    }

    pub fn receive_event(&self) -> Vec<Event> {
        self.connection
            .lock()
            .unwrap()
            .socket
            .receive()
            .iter()
            .map(|(_, payload)| payload)
            .filter_map(|payload| ciborium::de::from_reader(&payload[..]).ok())
            .collect()
    }

    pub fn connected_peers_count(&self) -> usize {
        self.connection.lock().unwrap().peers.len()
    }

    pub fn register_peers(&self) -> Result<(), &'static str> {
        let mut connection = self.connection.lock().unwrap();
        for (peer_id, new_state) in connection.socket.try_update_peers()? {
            match new_state {
                PeerState::Connected => {
                    info!("Connected with peer: {:?}", peer_id);
                    connection.peers.insert(peer_id);
                }
                PeerState::Disconnected => {
                    info!("Disconnected with peer: {:?}", peer_id);
                    connection.peers.remove(&peer_id);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[futures_test::test]
    async fn send_and_recieve_events() {
        let handle_1 = MaproxHandle::new(MAPROX_CONNECTION_URL);
        let handle_2 = MaproxHandle::new(MAPROX_CONNECTION_URL);

        futures::join!(wait_for_peer(&handle_1), wait_for_peer(&handle_2));

        handle_1.send_event(Event::Increment);
        futures_timer::Delay::new(std::time::Duration::from_millis(500)).await;
        assert_eq!(
            &Event::Increment,
            handle_2.receive_event().first().expect("Received event")
        );
    }

    async fn wait_for_peer(maprox_handle: &MaproxHandle) {
        loop {
            maprox_handle.register_peers().unwrap();

            futures_timer::Delay::new(std::time::Duration::from_millis(500)).await;

            if maprox_handle.connected_peers_count() != 0 {
                info!("Found for a maprox connection.");
                break;
            }

            info!("Waiting for a maprox connection.");
        }
    }
}
