use crate::config::NetworkConfig;
use crate::transport::{ConnectionId, ConnectionManager, Server};

use crate::identity::NodeIdentity;
use crate::peer::{PeerId, PeerRegistry};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;

pub struct NetworkManager {
    config: NetworkConfig,
    identity: NodeIdentity,
    peers: PeerRegistry,
    connections: Arc<ConnectionManager>,
}

impl NetworkManager {
    pub fn new(config: NetworkConfig, identity: NodeIdentity) -> Self {
        Self {
            config,
            identity,
            peers: PeerRegistry::new(),
            connections: Arc::new(ConnectionManager::new()),
        }
    }

    pub fn start(self: &Arc<Self>) -> io::Result<()> {
        self.start_server()?;
        self.connect_bootstrap_peers();

        Ok(())
    }

    fn start_server(self: &Arc<Self>) -> io::Result<()> {
        let server = Server::bind(self.config.listen_address(), Arc::clone(&self.connections))?;

        thread::spawn(move || {
            server.run();
        });

        Ok(())
    }

    fn connect_bootstrap_peers(self: &Arc<Self>) {
        for address in self.config.bootstrap_peers() {
            if let Err(error) = self.connect(*address) {
                println!("Failed to connect to bootstrap {}: {}", address, error);
            }
        }
    }

    pub fn connect(self: &Arc<Self>, address: SocketAddr) -> io::Result<ConnectionId> {
        self.connections.connect(address)
    }

    pub fn send(&self, connection: ConnectionId, data: &[u8]) -> io::Result<()> {
        self.connections.send(connection, data)
    }

    pub(crate) fn local_peer_id(&self) -> PeerId {
        self.identity.peer_id()
    }
}
