use std::io::{Read, Result, Write};
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

            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
            let peer_addr = self.stream.peer_addr()?;
            print!("{} sent {} bytes: {}", peer_addr, bytes_read, message);
            self.send(b"Automatic reply. Message received. Thanks!\n")?;
        }

        Ok(())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data)
    }
}
