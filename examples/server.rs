use std::sync::Arc;
use std::thread;
use znet::network::config::NetworkConfig;
use znet::network::manager::NetworkManager;
use znet::network::server::Server;

fn main() {
    let config = NetworkConfig::new(12137, vec![1, 2, 3]);
    let network_manager = Arc::new(NetworkManager::new(config));
    network_manager.start_network();
    loop {
        thread::park();
    }
}
