use crate::peer::PeerId;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Peer {
    id: PeerId,
    address: SocketAddr,
}

impl Peer {
    pub fn new(id: PeerId, address: SocketAddr) -> Self {
        Self { id, address }
    }

    pub fn id(&self) -> PeerId {
        self.id
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }
}
