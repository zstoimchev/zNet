use p256::ecdsa::signature::{Signer, Verifier};
use p256::ecdsa::{Signature, SigningKey, VerifyingKey};
use p256::elliptic_curve::Generate;

pub const PRIVATE_KEY_LENGTH: usize = 32;
pub const PUBLIC_KEY_LENGTH: usize = 33;
pub const SIGNATURE_LENGTH: usize = 64;

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

    pub(crate) fn sign(&self, message: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        let signature: Signature = self.signing_key.sign(message);

        signature.to_bytes().into()
    }

    pub(crate) fn verify(
        public_key: &[u8; PUBLIC_KEY_LENGTH],
        message: &[u8],
        signature: &[u8; SIGNATURE_LENGTH],
    ) -> Result<(), p256::ecdsa::Error> {
        let verifying_key = VerifyingKey::from_sec1_bytes(public_key)?;

        let signature = Signature::from_slice(signature)?;

        verifying_key.verify(message, &signature)
    }
}
