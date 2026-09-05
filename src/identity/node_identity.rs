use crate::config::IdentityConfig;
use crate::crypto::{KeyPair, KeyStore, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};

use crate::peer::PeerId;
use std::io;

pub struct NodeIdentity {
    key_pair: KeyPair,
}

impl NodeIdentity {
    pub fn load_or_create(config: &IdentityConfig) -> io::Result<Self> {
        let key_store = KeyStore::new(config.key_path());
        let key_pair = key_store.load_or_create()?;
        Ok(Self { key_pair })
    }

    pub fn peer_id(&self) -> PeerId {
        PeerId::from_public_key(self.key_pair.public_key_bytes())
    }

    pub(crate) fn public_key_bytes(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.key_pair.public_key_bytes()
    }

    pub(crate) fn sign(&self, message: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        self.key_pair.sign(message)
    }
}
