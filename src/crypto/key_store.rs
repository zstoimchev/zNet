use crate::crypto::key_pair::{KeyPair, PRIVATE_KEY_LENGTH};

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct KeyStore {
    path: PathBuf,
}

impl KeyStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load_or_create(&self) -> io::Result<KeyPair> {
        if self.path.exists() {
            return self.load();
        }

        let key_pair = KeyPair::generate();
        self.save(&key_pair)?;

        Ok(key_pair)
    }

    fn load(&self) -> io::Result<KeyPair> {
        let bytes = fs::read(&self.path)?;

        let private_key: [u8; PRIVATE_KEY_LENGTH] = bytes.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid node private key length",
            )
        })?;

        KeyPair::from_private_bytes(&private_key)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid node private key"))
    }

    fn save(&self, key_pair: &KeyPair) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, key_pair.private_key_bytes())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
