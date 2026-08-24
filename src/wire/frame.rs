use crate::wire::FrameType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    frame_type: FrameType,
    flags: u8,
    payload: Vec<u8>,
}

impl Frame {
    pub fn new(frame_type: FrameType, payload: Vec<u8>) -> Self {
        Self {
            frame_type,
            flags: 0,
            payload,
        }
    }

    pub fn with_flags(frame_type: FrameType, flags: u8, payload: Vec<u8>) -> Self {
        Self {
            frame_type,
            flags,
            payload,
        }
    }

    pub fn frame_type(&self) -> FrameType {
        self.frame_type
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
