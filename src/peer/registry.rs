use crate::peer::{Peer, PeerId};
use crate::transport::ConnectionId;

use std::collections::HashMap;
use std::sync::Mutex;

struct PeerEntry {
    peer: Peer,
    connection: Option<ConnectionId>,
}

pub struct PeerRegistry {
    peers: Mutex<HashMap<PeerId, PeerEntry>>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self {
            peers: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, peer: Peer, connection: ConnectionId) {
        let id = peer.id();

        self.peers.lock().unwrap().insert(
            id,
            PeerEntry {
                peer,
                connection: Some(connection),
            },
        );
    }

    pub fn peer(&self, id: PeerId) -> Option<Peer> {
        self.peers
            .lock()
            .unwrap()
            .get(&id)
            .map(|entry| entry.peer.clone())
    }

    pub fn connection(&self, id: PeerId) -> Option<ConnectionId> {
        self.peers
            .lock()
            .unwrap()
            .get(&id)
            .and_then(|entry| entry.connection)
    }

    pub fn disconnect(&self, id: PeerId) {
        if let Some(entry) = self.peers.lock().unwrap().get_mut(&id) {
            entry.connection = None;
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.lock().unwrap().len()
    }
}
