use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct NetworkConfig {
    address: IpAddr,
    port: u16,
    public_key: Vec<u8>,
    bootstrap: Option<SocketAddr>,
}

impl NetworkConfig {
    pub fn new(port: u16, public_key: Vec<u8>, bootstrap: Option<SocketAddr>) -> Self {
        Self {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
            public_key,
            bootstrap,
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }

    pub fn bootstrap(&self) -> Option<SocketAddr> {
        self.bootstrap
    }
}
