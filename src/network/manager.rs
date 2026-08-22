use crate::network::connection::PeerConnection;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

pub struct NetworkManager {
    connections: Mutex<HashMap<SocketAddr, TcpStream>>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            connections: Mutex::new(HashMap::new()),
        }
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
