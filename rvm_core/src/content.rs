use crate::blob::Blob;
use crate::tree::Tree;
use crate::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Tree(Tree),
    Blob(Blob),
}

impl Content {
    pub fn get_hash(&self) -> Hash {
        use self::Content::*;
        match self {
            Tree(t) => t.hash.clone(),
            Blob(b) => b.hash.clone(),
        }
    }
}
