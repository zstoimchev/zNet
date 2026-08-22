use crate::network::manager::NetworkManager;
use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::thread;

pub struct PeerConnection {
    stream: TcpStream,
    network_manager: Arc<NetworkManager>,
}

impl PeerConnection {
    pub fn new(stream: TcpStream, network_manager: Arc<NetworkManager>) -> Self {
        Self {
            stream,
            network_manager,
        }
    }

    pub fn spawn(stream: TcpStream, network_manager: Arc<NetworkManager>) {
        thread::spawn(move || {
            let mut connection = Self::new(stream, network_manager);
            connection.run();
        });
    }

    fn run(&mut self) {
        let address: SocketAddr = self.stream.peer_addr().unwrap();
        println!("Connection opened: {}", address);

        let mut buffer = [0; 1024];

        loop {
            match self.stream.read(&mut buffer) {
                Ok(0) => return self.handle_close(address),
                Ok(bytes) => println!("{}: {}", address, String::from_utf8_lossy(&buffer[..bytes])),
                Err(error) => return self.handle_error(address, error),
            }
        }
    }

    fn handle_close(&self, address: SocketAddr) {
        self.network_manager.remove_connection(address);
        println!("Connection closed: {}", address);
    }

    fn handle_error(&self, address: SocketAddr, error: std::io::Error) {
        println!("Connection error {}: {}", address, error);
        self.handle_close(address);
    }
}
