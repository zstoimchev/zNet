use crate::config::NetworkConfig;
use crate::network::NetworkManager;
use crate::transport::ConnectionId;

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct ZNet {
    network_manager: Arc<NetworkManager>,
}

impl ZNet {
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            network_manager: Arc::new(NetworkManager::new(config)),
        }
    }

    pub fn start(&self) -> io::Result<()> {
        self.network_manager.start()
    }

    pub fn connect(&self, address: SocketAddr) -> io::Result<ConnectionId> {
        self.network_manager.connect(address)
    }

    pub fn send(&self, connection: ConnectionId, data: &[u8]) -> io::Result<()> {
        self.network_manager.send(connection, data)
    }

    pub fn connection_count(&self) -> usize {
        self.network_manager.connection_count()
    }
}
