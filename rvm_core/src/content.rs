use crate::blob::Blob;
use crate::tree::Tree;
use crate::hash::Hash;
use serde::{Deserialize, Serialize};
use crate::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Tree(Tree),
    Blob(Blob),
}

impl Content {
    pub fn from_path(path: Path) -> Self {
        match path {
            Path::Tree(_) => Self::Tree(todo!()),
            Path::Blob(_) => Self::Blob(todo!()),
            _ => panic!()
        }
    }

    pub fn get_hash(&self) -> Hash {
        use self::Content::*;
        match self {
            Tree(t) => t.hash.clone(),
            Blob(b) => b.hash.clone(),
        }
    }
}
