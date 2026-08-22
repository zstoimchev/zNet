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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_peer() {
        let peer = Peer::new(
            1,
            "127.0.0.1".to_string(),
            9000,
        );

        assert_eq!(peer.id(), 1);
        assert_eq!(peer.host(), "127.0.0.1");
        assert_eq!(peer.port(), 9000);
    }
}
