use crate::network::manager::NetworkManager;
use crate::network::server::Server;
use std::sync::Arc;
use std::thread;

pub struct ZNet {
    server: Server,
    network_manager: Arc<NetworkManager>,
}

impl ZNet {
    pub fn new(address: String) -> Self {
        let network_manager = Arc::new(NetworkManager::new());

        Self {
            server: Server::new(address, Arc::clone(&network_manager)),
            network_manager,
        }
    }

    pub fn start(&self) {
        let server = self.server.clone();

        thread::spawn(move || {
            server.start();
        });
    }

    pub fn connection_count(&self) -> usize {
        self.network_manager.connection_count()
    }
}
