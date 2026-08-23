use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    bind_address: IpAddr,
    port: u16,
    bootstrap_peers: Vec<SocketAddr>,
}

impl NetworkConfig {
    pub fn new(port: u16) -> Self {
        Self {
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
            bootstrap_peers: Vec::new(),
        }
    }

    pub fn with_bind_address(mut self, address: IpAddr) -> Self {
        self.bind_address = address;
        self
    }

    pub fn with_bootstrap_peer(mut self, address: SocketAddr) -> Self {
        self.bootstrap_peers.push(address);
        self
    }

    pub fn listen_address(&self) -> SocketAddr {
        SocketAddr::new(self.bind_address, self.port)
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn bootstrap_peers(&self) -> &[SocketAddr] {
        &self.bootstrap_peers
    }
}
