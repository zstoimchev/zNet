use crate::wire::Frame;
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(u64);

impl ConnectionId {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

pub struct Connection {
    address: SocketAddr,
    stream: Mutex<TcpStream>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let address = stream.peer_addr()?;

        Ok(Self {
            address,
            stream: Mutex::new(stream),
        })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub(crate) fn send(&self, frame: &Frame) -> io::Result<()> {
        let mut stream = self.stream.lock().unwrap();
        frame.write_to(&mut *stream)
    }

    pub(crate) fn run(&self) -> io::Result<()> {
        let mut stream = self.stream.lock().unwrap().try_clone()?;
        loop {
            let frame = Frame::read_from(&mut stream)?;
            println!(
                "Received {:?} frame from {}: {:?}",
                frame.frame_type(),
                self.address,
                String::from_utf8_lossy(frame.payload())
            );
        }
    }
}
