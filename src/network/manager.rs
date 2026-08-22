use crate::network::config::NetworkConfig;
use crate::network::connection::PeerConnection;
use crate::network::server::Server;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct NetworkManager {
    config: NetworkConfig,
    connections: Mutex<HashMap<SocketAddr, TcpStream>>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            config: NetworkConfig::new(12137),
            connections: Mutex::new(HashMap::new()),
        }
    }

    pub fn start_network(self: &Arc<Self>) {
        self.start_server();
    }

    fn start_server(self: &Arc<Self>) {
        let server = Server::new(self.config.socket_address(), Arc::clone(self));

        thread::spawn(move || {
            server.start();
        });
    }

    pub fn handle_connection(self: &Arc<Self>, stream: TcpStream) {
        let address = stream.peer_addr().unwrap();
        self.register_connection(address, &stream);
        PeerConnection::spawn(stream, Arc::clone(self));
    }

    fn register_connection(&self, address: SocketAddr, stream: &TcpStream) {
        let stream = stream.try_clone().expect("Failed to clone TCP stream");
        self.connections.lock().unwrap().insert(address, stream);
    }

    pub fn remove_connection(&self, address: SocketAddr) {
        self.connections.lock().unwrap().remove(&address);
    }

    pub fn connection_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }
}
