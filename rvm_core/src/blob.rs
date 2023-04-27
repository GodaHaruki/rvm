use crate::hash::Hash;
use crate::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blob {
    pub name: String,
    pub hash: Hash, // hash.blob is file contents
    pub path: Path,
}

impl Blob {
    pub fn new(name: String, hash: Hash, path: Path) -> Self {
        Blob { name, hash, path }
    }
}
