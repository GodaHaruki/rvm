use crate::branch::Branch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub branches: Vec<Branch>,
}

#[derive(Debug)]
struct SameNameBranchExistError;

impl std::error::Error for SameNameBranchExistError{}

impl std::fmt::Display for SameNameBranchExistError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SameFileExistError")
    }
}

impl Repository {
    pub fn new(name: String) -> Self {
        Self {
            name,
            branches: Vec::<Branch>::new(),
        }
    }

    pub fn add_branch(&mut self, branch: Branch) -> Result<(), Box<dyn std::error::Error>>{
        {
            for b in &self.branches {
                if Branch::is_same_name(b, &branch){ return Err(Box::new(SameNameBranchExistError));}
            }
        }

        self.branches.push(branch);
        unimplemented!()
    }
}
