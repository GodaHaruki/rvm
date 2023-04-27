use crate::tree::Tree;
use crate::path::Path;
use crate::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub hash: Hash,
    pub parent: Option<Path>,
    pub contents: Tree,
}

impl Commit {
    pub fn new(contents: Tree) -> Self {
        unimplemented!()
    }
}
