use crate::blob::Blob;
use crate::tree::Tree;
use crate::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Tree(Tree),
    Blob(Blob),
}
