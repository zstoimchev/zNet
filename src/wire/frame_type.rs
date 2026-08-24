#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
    Handshake = 0x01,
    Data = 0x02,
    Discovery = 0x03,
    Heartbeat = 0x04,
}

impl TryFrom<u8> for FrameType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Handshake),
            0x02 => Ok(Self::Data),
            0x03 => Ok(Self::Discovery),
            0x04 => Ok(Self::Heartbeat),
            _ => Err(()),
        }
    }
}
