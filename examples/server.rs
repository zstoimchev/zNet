use std::path::Path;
use std::sync::Arc;
use std::thread;
use znet::identity::NodeIdentity;
use znet::network::config::NetworkConfig;
use znet::network::manager::NetworkManager;

fn main() {
    let identity = NodeIdentity::load_or_generate(Path::new("server.identity.pem")).unwrap();
    let config = NetworkConfig::new(12137, None);
    let network_manager = Arc::new(NetworkManager::new(config, identity));
    network_manager.start_network();
    loop {
        thread::park();
    }
}
