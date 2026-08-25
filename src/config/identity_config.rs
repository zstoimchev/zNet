use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct IdentityConfig {
    key_path: PathBuf,
}

impl IdentityConfig {
    pub fn new(key_path: impl Into<PathBuf>) -> Self {
        Self {
            key_path: key_path.into(),
        }
    }

    pub fn key_path(&self) -> &Path {
        &self.key_path
    }
}
