use std::io::{self, Read, Write};

const MAX_PAYLOAD_SIZE: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
    Handshake = 1,
    Data = 2,
}

impl FrameType {
    fn from_u8(value: u8) -> io::Result<Self> {
        match value {
            1 => Ok(Self::Handshake),
            2 => Ok(Self::Data),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown frame type")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    frame_type: FrameType,
    payload: Vec<u8>,
}

impl Frame {
    pub fn new(frame_type: FrameType, payload: Vec<u8>) -> Self {
        Self {
            frame_type,
            payload,
        }
    }

    pub fn frame_type(&self) -> FrameType {
        self.frame_type
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.payload.len() > MAX_PAYLOAD_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Frame payload too large"));
        }

        writer.write_all(&[self.frame_type as u8])?;
        let payload_length = self.payload.len() as u32;
        writer.write_all(&payload_length.to_be_bytes())?;
        writer.write_all(&self.payload)?;

        Ok(())
    }

    pub fn read_from<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut frame_type = [0u8; 1];
        reader.read_exact(&mut frame_type)?;

        let frame_type = FrameType::from_u8(frame_type[0])?;
        let mut payload_length = [0u8; 4];
        reader.read_exact(&mut payload_length)?;
        let payload_length = u32::from_be_bytes(payload_length) as usize;

        if payload_length > MAX_PAYLOAD_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Frame payload too large",
            ));
        }

        let mut payload = vec![0u8; payload_length];
        reader.read_exact(&mut payload)?;

        Ok(Self {
            frame_type,
            payload,
        })
    }
}
