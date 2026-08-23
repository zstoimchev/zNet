use crate::network::manager::NetworkManager;
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;

pub struct Server {
    address: SocketAddr,
    network_manager: Arc<NetworkManager>,
}

impl Server {
    pub fn new(address: SocketAddr, network_manager: Arc<NetworkManager>) -> Self {
        Self {
            address,
            network_manager,
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to start server");

        println!("Server listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.network_manager.handle_connection(stream),
                Err(error) => self.handle_error(error),
            }
        }
    }

    fn handle_error(&self, error: std::io::Error) {
        println!("Failed to accept connection: {}", error);
    }
}
