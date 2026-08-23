use std::io::{Error, ErrorKind};

pub enum MessageType {
    Handshake,
    PeerDiscoveryRequest,
    PeerDiscoveryResponse,
    Heartbeat,
    Data,
}

impl MessageType {
    pub fn to_byte(&self) -> u8 {
        match self {
            MessageType::Handshake => 1,
            MessageType::PeerDiscoveryRequest => 2,
            MessageType::PeerDiscoveryResponse => 3,
            MessageType::Heartbeat => 4,
            MessageType::Data => 5,
        }
    }

    pub fn from_byte(value: u8) -> std::io::Result<Self> {
        match value {
            1 => Ok(MessageType::Handshake),
            2 => Ok(MessageType::PeerDiscoveryRequest),
            3 => Ok(MessageType::PeerDiscoveryResponse),
            4 => Ok(MessageType::Heartbeat),
            5 => Ok(MessageType::Data),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknown message type")),
        }
    }
}
