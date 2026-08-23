use crate::config::IdentityConfig;
use crate::crypto::{KeyPair, KeyStore};

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

    pub fn public_key(&self) -> [u8; 33] {
        self.key_pair.public_key_bytes()
    }
}
