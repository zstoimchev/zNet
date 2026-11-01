use crate::wire::frame;
use std::io::Result;
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        println!("New connection from {}", stream.peer_addr().unwrap());
        Self { stream }
    }

    pub fn receive(&mut self) -> Result<Option<Vec<u8>>> {
        frame::read_frame(&mut self.stream)
    }

    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        frame::write_frame(&mut self.stream, data)
    }
}
