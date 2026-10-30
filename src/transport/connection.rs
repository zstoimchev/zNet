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

    pub fn receive(&mut self) -> Result<()> {
        loop {
            let Some(data) = frame::read_frame(&mut self.stream)? else {
                println!("\nClient ({}) disconnected", self.stream.peer_addr()?);
                break;
            };

            let peer_addr = self.stream.peer_addr()?;
            let bytes_read = data.len();
            let message = String::from_utf8_lossy(&data);
            print!("{} sent {} bytes: {}", peer_addr, bytes_read, message);
            self.send(b"Automatic reply. Message received. Thanks!\n")?;
        }

        Ok(())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        frame::write_frame(&mut self.stream, data)
    }
}
