use crate::tree::Tree;
use crate::path::Path;
use crate::hash::Hash;
use serde::{Deserialize, Serialize};
use rand::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
 //   pub hash: Hash,
    pub parent: Option<Path>,
    pub contents: Tree,
}

impl Commit {
    pub fn new(contents: Tree) -> Self {
        let mut rng = rand::thread_rng(); 
        let mut gen_id = |digits| -> String {
            let mut res = String::new();
            for _ in 0..digits{
                res.push(rng.gen());
            }
            let res = res;
            return res;
        };
        Self {
            id: gen_id(6),
            parent: None,
            contents: Tree::new("top".to_string(), Vec::new())
        }
    }
}
