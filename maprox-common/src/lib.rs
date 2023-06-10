use geo_types::Geometry;
use log::info;
use matchbox_socket::{PeerId, PeerState, SingleChannel, WebRtcSocket};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const MAPROX_CONNECTION_URL: &str = "ws://127.0.0.1:3536/maprox";

#[derive(Serialize, Deserialize, Clone)]
pub enum Event {
    Increment,
    RenderGeometry(Geometry),
}

pub struct MaproxConnection {
    socket: WebRtcSocket<SingleChannel>,
    peers: HashSet<PeerId>,
}

impl MaproxConnection {
    pub fn new(url_str: &str) -> Self {
        let (socket, loop_fut) = WebRtcSocket::new_reliable(url_str);

        std::thread::spawn(move || {
            let executor = async_executor::Executor::new();
            let task = executor.spawn(loop_fut);
            futures_lite::future::block_on(executor.run(task))
                .expect("Failed to init maprox_connection");
        });

        Self {
            socket,
            peers: HashSet::with_capacity(1),
        }
    }

    pub fn send_event(&mut self, event: Event) {
        let mut payload = Vec::new();
        ciborium::ser::into_writer(&event, &mut payload).unwrap();
        for peer in &self.peers {
            self.socket.send(payload.clone().into(), *peer);
        }
    }

    pub fn receive_event(&mut self) -> Vec<Event> {
        self.socket
            .receive()
            .iter()
            .map(|(_, payload)| payload)
            .filter_map(|payload| ciborium::de::from_reader(&payload[..]).ok())
            .collect()
    }

    pub fn connected_peers_count(&self) -> usize {
        self.peers.len()
    }

    pub fn register_peers(&mut self) -> Result<(), &'static str> {
        for (peer_id, new_state) in self.socket.try_update_peers()? {
            match new_state {
                PeerState::Connected => {
                    info!("Connected with peer: {:?}", peer_id);
                    self.peers.insert(peer_id);
                }
                PeerState::Disconnected => {
                    info!("Disconnected with peer: {:?}", peer_id);
                    self.peers.remove(&peer_id);
                }
            }
        }

        Ok(())
    }
}
