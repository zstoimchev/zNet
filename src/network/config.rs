use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct NetworkConfig {
    address: IpAddr,
    port: u16,
}

impl NetworkConfig {
    pub fn new(port: u16) -> Self {
        Self {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
        }
    }

    pub fn socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }
}
