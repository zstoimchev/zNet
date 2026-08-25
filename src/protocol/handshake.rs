use crate::crypto::PUBLIC_KEY_LENGTH;
use std::io;

pub(crate) const PROTOCOL_VERSION: u16 = 1;

const HANDSHAKE_PAYLOAD_LENGTH: usize = 1 + 2 + 2 + PUBLIC_KEY_LENGTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum HandshakeKind {
    Hello = 1,
    Ack = 2,
}

impl HandshakeKind {
    fn from_u8(value: u8) -> io::Result<Self> {
        match value {
            1 => Ok(Self::Hello),
            2 => Ok(Self::Ack),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown handshake type",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Handshake {
    kind: HandshakeKind,
    version: u16,
    listen_port: u16,
    public_key: [u8; PUBLIC_KEY_LENGTH],
}

impl Handshake {
    pub(crate) fn new(
        kind: HandshakeKind,
        listen_port: u16,
        public_key: [u8; PUBLIC_KEY_LENGTH],
    ) -> Self {
        Self {
            kind,
            version: PROTOCOL_VERSION,
            listen_port,
            public_key,
        }
    }

    pub(crate) fn kind(&self) -> HandshakeKind {
        self.kind
    }

    pub(crate) fn version(&self) -> u16 {
        self.version
    }

    pub(crate) fn listen_port(&self) -> u16 {
        self.listen_port
    }

    pub(crate) fn public_key(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.public_key
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(HANDSHAKE_PAYLOAD_LENGTH);
        payload.push(self.kind as u8);
        payload.extend_from_slice(&self.version.to_be_bytes());
        payload.extend_from_slice(&self.listen_port.to_be_bytes());
        payload.extend_from_slice(&self.public_key);
        payload
    }

    pub(crate) fn decode(payload: &[u8]) -> io::Result<Self> {
        if payload.len() != HANDSHAKE_PAYLOAD_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid handshake payload length",
            ));
        }

        let kind = HandshakeKind::from_u8(payload[0])?;
        let version = u16::from_be_bytes([payload[1], payload[2]]);
        let listen_port = u16::from_be_bytes([payload[3], payload[4]]);
        let mut public_key = [0u8; PUBLIC_KEY_LENGTH];
        public_key.copy_from_slice(&payload[5..5 + PUBLIC_KEY_LENGTH]);

        Ok(Self {
            kind,
            version,
            listen_port,
            public_key,
        })
    }
}
