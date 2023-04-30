use crate::{commit::Commit, save::Save};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commits: Commit, // it should be newest commit
}

impl Branch {
    pub fn new(name: String) -> Self {
        Branch {
            name,
            commits: unimplemented!()
        }
    }

    pub fn is_same_name(&self, b: &Self) -> bool {
        self.name == b.name
    }
    
    pub fn merge() -> Result<(), Box<dyn std::error::Error>>{
        unimplemented!()
    }
}

impl Save for Branch {}