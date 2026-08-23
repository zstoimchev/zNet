use crate::identity::NodeIdentity;
use crate::network::config::NetworkConfig;
use crate::network::connection::PeerConnection;
use crate::network::peer::Peer;
use crate::network::server::Server;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct NetworkManager {
    config: NetworkConfig,
    identity: NodeIdentity,
    peers: Mutex<HashMap<Vec<u8>, Peer>>,
    connections: Mutex<HashMap<Vec<u8>, Arc<PeerConnection>>>,
}

impl NetworkManager {
    pub fn new(config: NetworkConfig, identity: NodeIdentity) -> Self {
        Self {
            config,
            identity,
            peers: Mutex::new(HashMap::new()),
            connections: Mutex::new(HashMap::new()),
        }
    }

    pub fn start_network(self: &Arc<Self>) {
        self.start_server();
        self.connect_bootstrap();
    }

    fn start_server(self: &Arc<Self>) {
        let server = Server::new(self.config.socket_address(), Arc::clone(self));

        thread::spawn(move || {
            server.start();
        });
    }

    fn connect_bootstrap(self: &Arc<Self>) {
        if let Some(address) = self.config.bootstrap() {
            self.connect(address);
        }
    }

    pub fn connect(self: &Arc<Self>, address: SocketAddr) {
        match TcpStream::connect(address) {
            Ok(stream) => self.handle_connection(stream),
            Err(error) => println!("Failed to connect to {}: {}", address, error),
        }
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
        self.identity.public_key()
    }
}
