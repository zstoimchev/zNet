pub struct Peer {
    host: String,
    port: u16,
    public_key: Vec<u8>,
}

impl Peer {
    pub fn new(host: String, port: u16, public_key: Vec<u8>) -> Self {
        Self {
            host,
            port,
            public_key,
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}
