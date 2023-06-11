use crate::events::Event;
use log::info;
use matchbox_socket::{PeerId, PeerState, SingleChannel, WebRtcSocket};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub const MAPROX_CONNECTION_URL: &str = "ws://127.0.0.1:3536/maprox";

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
                futures_lite::future::block_on(executor.run(task))
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
