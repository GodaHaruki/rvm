use crate::content::Content;
use serde::{Deserialize, Serialize};
use crate::hash::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub hash: Hash,
    pub name: String,
    pub contents: Vec<Content>,
}
