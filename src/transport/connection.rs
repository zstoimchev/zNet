use std::io;
use std::io::{Read, Write};
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
    id: ConnectionId,
    address: SocketAddr,
    stream: Mutex<TcpStream>,
}

impl Connection {
    pub fn new(id: ConnectionId, stream: TcpStream) -> io::Result<Self> {
        let address = stream.peer_addr()?;

        Ok(Self {
            id,
            address,
            stream: Mutex::new(stream),
        })
    }

    pub fn id(&self) -> ConnectionId {
        self.id
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn send(&self, data: &[u8]) -> io::Result<()> {
        self.stream.lock().unwrap().write_all(data)
    }

    pub(crate) fn run(&self) -> io::Result<()> {
        let mut stream = self.stream.lock().unwrap().try_clone()?;

        let mut buffer = [0u8; 4096];

        loop {
            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read == 0 {
                return Ok(());
            }

            println!("Received {} bytes from {}", bytes_read, self.address);
        }
    }
}
