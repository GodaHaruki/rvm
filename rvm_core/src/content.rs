use crate::{blob::Blob, read_as};
use crate::tree::Tree;
use crate::hash::Hash;
use serde::{Deserialize, Serialize};
use crate::path::Path;
use crate::save::Save;

#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Tree(Tree),
    Blob(Blob),
}

impl Content {
    pub fn from_path(path: Path) -> Self {
        match path {
            Path::Tree(_) => Self::Tree(read_as!(Tree, path.get_path_as_str().unwrap()).unwrap()),
            Path::Blob(_) => Self::Blob(read_as!(Blob, path.get_path_as_str().unwrap()).unwrap()),
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
