pub type PeerId = u64;

pub struct Peer {
    id: PeerId,
    host: String,
    port: u16,
}

impl Peer {
    pub fn new(id: PeerId, host: String, port: u16) -> Self {
        Self { id, host, port }
    }

    pub fn id(&self) -> PeerId {
        self.id
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
