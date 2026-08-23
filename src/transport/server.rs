use crate::transport::ConnectionManager;
use std::io;
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;

pub struct Server {
    listener: TcpListener,
    connection_manager: Arc<ConnectionManager>,
}

impl Server {
    pub fn bind(
        address: SocketAddr,
        connection_manager: Arc<ConnectionManager>,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(Self {
            listener,
            connection_manager,
        })
    }

    pub fn run(self) {
        println!(
            "Server listening on {}",
            self.listener.local_addr().unwrap()
        );

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(error) = self.connection_manager.accept(stream) {
                        println!("Failed to register connection: {}", error);
                    }
                }

                Err(error) => {
                    println!("Failed to accept connection: {}", error);
                }
            }
        }
    }
}
