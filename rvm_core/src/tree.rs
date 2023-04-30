use crate::{content::Content, save::Save};
use serde::{Deserialize, Serialize};
use crate::hash::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub hash: Hash,
    pub name: String,
    pub contents: Vec<Content>,
}

impl Save for Tree {}

impl Tree {
    pub fn gen_contents_hash(&self) -> String{
        let mut contents_hashes = String::new();
        self.contents.iter().for_each(
            |c| contents_hashes += &c.get_hash().to_string());
        return contents_hashes;
    }

    pub fn new(name: String, contents: Vec<Content>) -> Self{
        Self{
            hash: Hash::new(&name),
            name,
            contents,
        }
    }
}