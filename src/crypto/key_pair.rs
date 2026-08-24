use p256::ecdsa::{SigningKey, VerifyingKey};
use p256::elliptic_curve::Generate;

pub const PRIVATE_KEY_LENGTH: usize = 32;
pub const PUBLIC_KEY_LENGTH: usize = 33;

pub struct KeyPair {
    signing_key: SigningKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        Self {
            signing_key: SigningKey::generate(),
        }
    }

    pub(crate) fn from_private_bytes(
        bytes: &[u8; PRIVATE_KEY_LENGTH],
    ) -> Result<Self, p256::ecdsa::Error> {
        let signing_key = SigningKey::from_bytes(bytes.into())?;
        Ok(Self { signing_key })
    }

    pub(crate) fn private_key_bytes(&self) -> [u8; PRIVATE_KEY_LENGTH] {
        self.signing_key.to_bytes().into()
    }

    pub fn public_key_bytes(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        let encoded = self.signing_key.verifying_key().to_sec1_point(true);
        let mut bytes = [0u8; PUBLIC_KEY_LENGTH];
        bytes.copy_from_slice(encoded.as_bytes());
        bytes
    }
}
