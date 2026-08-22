use crate::network::peer::Peer;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;

pub struct HandshakeProtocol;

impl HandshakeProtocol {
    pub fn perform(
        stream: &mut TcpStream,
        local_port: u16,
        public_key: &[u8],
    ) -> std::io::Result<Peer> {
        Self::send(stream, local_port, public_key)?;
        Self::receive(stream)
    }

    fn send(stream: &mut TcpStream, port: u16, public_key: &[u8]) -> std::io::Result<()> {
        stream.write_all(b"ZNET")?;
        stream.write_all(&[1])?;
        stream.write_all(&port.to_be_bytes())?;
        stream.write_all(&(public_key.len() as u16).to_be_bytes())?;
        stream.write_all(public_key)?;

        Ok(())
    }

    fn receive(stream: &mut TcpStream) -> std::io::Result<Peer> {
        let mut magic = [0u8; 4];
        stream.read_exact(&mut magic)?;

        if &magic != b"ZNET" {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid handshake"));
        }

        let mut version = [0u8; 1];
        stream.read_exact(&mut version)?;

        if version[0] != 1 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unsupported protocol version",
            ));
        }

        let mut port = [0u8; 2];
        stream.read_exact(&mut port)?;

        let mut key_length = [0u8; 2];
        stream.read_exact(&mut key_length)?;

        let port = u16::from_be_bytes(port);
        let key_length = u16::from_be_bytes(key_length) as usize;

        let mut public_key = vec![0u8; key_length];
        stream.read_exact(&mut public_key)?;

        let host = stream.peer_addr()?.ip().to_string();

        Ok(Peer::new(host, port, public_key))
    }
}
