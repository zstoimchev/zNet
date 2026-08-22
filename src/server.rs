use crate::connection::PeerConnection;
use std::net::TcpListener;

#[derive(Clone)]
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn start(&self) {
        let tcp_listener = TcpListener::bind(&self.address).expect("Failed to start server");

        println!("Server listening on {}", self.address);

        for stream in tcp_listener.incoming() {
            match stream {
                Ok(stream) => PeerConnection::spawn(stream),
                Err(error) => println!("Failed to accept connection: {}", error),
            }
        }
    }
}
