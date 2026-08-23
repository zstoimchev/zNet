use p256::ecdsa::SigningKey;
use p256::elliptic_curve::Generate;
use p256::pkcs8::{DecodePrivateKey, EncodePrivateKey, LineEnding};
use std::error::Error;
use std::path::Path;

pub struct NodeIdentity {
    signing_key: SigningKey,
    public_key: Vec<u8>,
}

impl NodeIdentity {
    pub fn load_or_generate(path: &Path) -> Result<Self, Box<dyn Error>> {
        let signing_key = if path.exists() {
            SigningKey::read_pkcs8_pem_file(path)?
        } else {
            let signing_key = SigningKey::generate();

            signing_key.write_pkcs8_pem_file(
                path,
                LineEnding::LF,
            )?;

            signing_key
        };

        Ok(Self::new(signing_key))
    }

    fn new(signing_key: SigningKey) -> Self {
        let public_key = signing_key
            .verifying_key()
            .to_sec1_point(false)
            .as_bytes()
            .to_vec();

        Self {
            signing_key,
            public_key,
        }
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}