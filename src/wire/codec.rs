use crate::wire::{Frame, FrameType};

use std::io::{self, Read, Write};

const MAGIC: &[u8; 4] = b"ZNET";
const VERSION: u8 = 1;

const MAX_PAYLOAD_SIZE: usize = 16 * 1024 * 1024;

impl Codec {
    pub fn write<W: Write>(writer: &mut W, frame: &Frame) -> io::Result<()> {
        let payload = frame.payload();

        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Frame payload too large",
            ));
        }

        writer.write_all(MAGIC)?;
        writer.write_all(&[VERSION])?;
        writer.write_all(&[frame.frame_type() as u8])?;
        writer.write_all(&[frame.flags()])?;
        writer.write_all(&(payload.len() as u32).to_be_bytes())?;
        writer.write_all(payload)?;

        Ok(())
    }

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Frame> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if &magic != MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid frame magic",
            ));
        }

        let mut version = [0u8; 1];
        reader.read_exact(&mut version)?;

        if version[0] != VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported protocol version",
            ));
        }

        let mut frame_type = [0u8; 1];
        reader.read_exact(&mut frame_type)?;

        let frame_type = FrameType::try_from(frame_type[0])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unknown frame type"))?;

        let mut flags = [0u8; 1];
        reader.read_exact(&mut flags)?;

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

        Ok(Frame::with_flags(frame_type, flags[0], payload))
    }
}
