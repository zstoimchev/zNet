use crate::network::peer::Peer;
use crate::protocol::frame::Frame;
use crate::protocol::message::MessageType;
use std::io::{Error, ErrorKind};
use std::net::TcpStream;

pub struct HandshakeProtocol;

struct HandshakePayload {
    port: u16,
    public_key: Vec<u8>,
}

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
        let payload = Self::encode_payload(port, public_key)?;
        let frame = Frame::new(MessageType::Handshake, 0, payload);
        frame.write(stream)
    }

    fn receive(stream: &mut TcpStream) -> std::io::Result<Peer> {
        let frame = Frame::read(stream)?;
        let (message_type, _, payload) = frame.into_parts();

        match message_type {
            MessageType::Handshake => {}
            _ => return Err(Error::new(ErrorKind::InvalidData, "Expected handshake")),
        }

        let handshake = Self::decode_payload(&payload)?;
        let host = stream.peer_addr()?.ip().to_string();

        Ok(Peer::new(host, handshake.port, handshake.public_key))
    }

    fn encode_payload(port: u16, public_key: &[u8]) -> std::io::Result<Vec<u8>> {
        if public_key.len() > u16::MAX as usize {
            return Err(Error::new(ErrorKind::InvalidData, "Public key too large"));
        }

        let mut payload = Vec::new();

        payload.extend_from_slice(&port.to_be_bytes());
        payload.extend_from_slice(&(public_key.len() as u16).to_be_bytes());
        payload.extend_from_slice(public_key);

        Ok(payload)
    }

    fn decode_payload(payload: &[u8]) -> std::io::Result<HandshakePayload> {
        if payload.len() < 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid handshake payload",
            ));
        }

        let port = u16::from_be_bytes([payload[0], payload[1]]);

        let key_length = u16::from_be_bytes([payload[2], payload[3]]) as usize;

        if key_length != payload.len() - 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid public key length",
            ));
        }

        let public_key = payload[4..].to_vec();

        Ok(HandshakePayload { port, public_key })
    }
}
