use crate::protocol::message::MessageType;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;

const MAGIC: &[u8; 4] = b"ZNET";
const VERSION: u8 = 1;
const MAX_PAYLOAD_SIZE: usize = 1024 * 1024;

pub struct Frame {
    message_type: MessageType,
    flags: u8,
    payload: Vec<u8>,
}

impl Frame {
    pub fn new(message_type: MessageType, flags: u8, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            flags,
            payload,
        }
    }

    pub fn write(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        if self.payload.len() > MAX_PAYLOAD_SIZE {
            return Err(Error::new(ErrorKind::InvalidData, "Payload too large"));
        }

        let length = self.payload.len() as u32;
        stream.write_all(MAGIC)?;
        stream.write_all(&[VERSION])?;
        stream.write_all(&[self.message_type.to_byte()])?;
        stream.write_all(&[self.flags])?;
        stream.write_all(&length.to_be_bytes())?;
        stream.write_all(&self.payload)?;
        Ok(())
    }

    pub fn read(stream: &mut TcpStream) -> std::io::Result<Self> {
        let mut magic = [0u8; 4];
        stream.read_exact(&mut magic)?;

        if &magic != MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid frame magic"));
        }

        let mut version = [0u8; 1];
        stream.read_exact(&mut version)?;

        if version[0] != VERSION {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unsupported protocol version",
            ));
        }

        let mut message_type = [0u8; 1];
        stream.read_exact(&mut message_type)?;

        let message_type = MessageType::from_byte(message_type[0])?;

        let mut flags = [0u8; 1];
        stream.read_exact(&mut flags)?;

        let mut length = [0u8; 4];
        stream.read_exact(&mut length)?;

        let length = u32::from_be_bytes(length) as usize;

        if length > MAX_PAYLOAD_SIZE {
            return Err(Error::new(ErrorKind::InvalidData, "Payload too large"));
        }

        let mut payload = vec![0u8; length];
        stream.read_exact(&mut payload)?;

        Ok(Self {
            message_type,
            flags: flags[0],
            payload,
        })
    }

    pub fn into_parts(self) -> (MessageType, u8, Vec<u8>) {
        (self.message_type, self.flags, self.payload)
    }
}
