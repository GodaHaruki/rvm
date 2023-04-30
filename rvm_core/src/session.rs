use crate::{read_as, path};
use crate::repository::Repository;
use crate::branch::Branch;
use crate::error::Error;
// use crate::path::Path;
use serde::{Serialize, Deserialize};
use crate::save::Save;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub repository: Option<Repository>,
    pub target: Option<String>
}



impl Session{
    pub fn new() -> Self{
        Self {
            repository: None,
            target: None,
        }
    }

    pub fn from_current_dir() -> Result<Self, Box<dyn std::error::Error>>{
        let p = std::env::current_dir().unwrap().join("/.rvm/main.rvm");
        Ok(
            Self{
                repository: Some(read_as!(Repository, p.to_str().unwrap())?), // Path::new_repository(&path)
                target: None,
        })
    }

    pub fn init() -> Self {
        let path = std::env::current_dir().unwrap();
        Self {
            repository: Some(Repository::new("main".to_string())),
            target: None,
        }
    }
    
    pub fn checkout(&mut self, repo_name: String) -> Result<(), Box<dyn std::error::Error>>{ // git checkout
        if let Some(repo) = &self.repository {
            let new_target: Vec<&Branch> = repo.branches.iter().filter(|r| r.name == repo_name).collect();
            if new_target.len() == 0{
                return Err(Box::new(Error::BranchNotFound));
            } else {
                self.target = Some(new_target[0].name.clone());
                return Ok(());
            }
        } else {
            return Err(Box::new(Error::BranchNotFound))
        }
    }
}
