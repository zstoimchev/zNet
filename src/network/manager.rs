use crate::network::config::NetworkConfig;
use crate::network::connection::PeerConnection;
use crate::network::peer::Peer;
use crate::network::server::Server;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct NetworkManager {
    config: NetworkConfig,
    peers: Mutex<HashMap<Vec<u8>, Peer>>,
    connections: Mutex<HashMap<Vec<u8>, Arc<PeerConnection>>>,
}

impl NetworkManager {
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            peers: Mutex::new(HashMap::new()),
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
        match PeerConnection::new(stream) {
            Ok(connection) => Arc::new(connection).start(Arc::clone(self)),
            Err(error) => println!("Failed to create connection: {}", error),
        }
    }

    pub fn register_peer(&self, peer: Peer, connection: Arc<PeerConnection>) {
        let public_key = peer.public_key().to_vec();
        self.peers.lock().unwrap().insert(public_key.clone(), peer);
        self.connections
            .lock()
            .unwrap()
            .insert(public_key, connection);
    }

    pub fn remove_connection(&self, public_key: &[u8]) {
        self.connections.lock().unwrap().remove(public_key);
    }

    pub fn local_port(&self) -> u16 {
        self.config.port()
    }

    pub fn local_public_key(&self) -> &[u8] {
        self.config.public_key()
    }
}
