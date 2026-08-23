use crate::crypto::PUBLIC_KEY_LENGTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerId([u8; PUBLIC_KEY_LENGTH]);

impl PeerId {
    pub fn from_public_key(public_key: [u8; PUBLIC_KEY_LENGTH]) -> Self {
        Self(public_key)
    }

    pub fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }
}
