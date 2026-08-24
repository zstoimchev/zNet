use crate::config::{IdentityConfig, NetworkConfig};
use crate::identity::NodeIdentity;
use crate::network::NetworkManager;
use crate::transport::ConnectionId;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct ZNet {
    network_manager: Arc<NetworkManager>,
}

impl ZNet {
    pub fn new(network_config: NetworkConfig, identity_config: IdentityConfig) -> io::Result<Self> {
        let identity = NodeIdentity::load_or_create(&identity_config)?;
        let network_manager = Arc::new(NetworkManager::new(network_config, identity));
        Ok(Self { network_manager })
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
}
