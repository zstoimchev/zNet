use crate::network::manager::NetworkManager;
use crate::network::server::Server;
use std::sync::Arc;
use std::thread;

pub struct ZNet {
    network_manager: Arc<NetworkManager>,
}

impl ZNet {
    pub fn new(network_manager: Arc<NetworkManager>) -> Self {
        Self { network_manager }
    }
}
