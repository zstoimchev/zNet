pub struct Peer {
    id: u64,
    host: String,
    port: u16,
}

impl Peer {
    pub fn new(id: u64, host: String, port: u16) -> Self {
        Self { id, host, port }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
