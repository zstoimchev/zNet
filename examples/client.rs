use std::net::TcpStream;
use std::sync::Arc;
use std::thread;

use znet::network::config::NetworkConfig;
use znet::network::manager::NetworkManager;

fn main() {
    let config = NetworkConfig::new(12138, vec![4, 5, 6]);
    let network_manager = Arc::new(NetworkManager::new(config));

    network_manager.start_network();

    let stream = TcpStream::connect("127.0.0.1:12137")
        .expect("Failed to connect");

    network_manager.handle_connection(stream);

    loop {
        thread::park();
    }
}