use std::io::{Read, Result};
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        println!("New connection from {:?}", stream.peer_addr());
        Self { stream }
    }

    pub fn receive(&mut self) -> Result<()> {
        let mut buffer = [0u8; 1024];

        loop {
            let bytes_read = self.stream.read(&mut buffer)?;

            if bytes_read == 0 {
                println!("Client disconnected");
                break;
            }

            println!("Received: {:?}", &buffer[..bytes_read]);
        }

        Ok(())
    }
}
