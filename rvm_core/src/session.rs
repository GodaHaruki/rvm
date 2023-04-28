use crate::repository::Repository;
use crate::branch::Branch;
use crate::error::Error;
use crate::path::Path;

struct Session {
    repository: Option<Repository>,
    target: Option<&Branch>
}

impl Session{
    pub fn new() -> Self{
        Self {
            repository: Repository::new(),
            target: None,
        }
    }

    pub fn from_current_dir() -> Self{
        let path = std::env::current_dir()?;
    }

    pub fn init() -> Self {
        let path = env::current_dir()?;
        Self {
            repository: deselizrize(Path::new_repository(path.to_string())),
            target: todo!()
        }
    }
    
    pub fn checkout(&mut self, repo_name: String) -> Result<(), Box<dyn std::error::error>{ // git checkout
        let new_target = self.repository.branches.filter(|r| -> bool r.name == repo_name).collect();
        if new_target.len() == 0 {
            return Err(Error::BranchNotFound)
        }
        target = new_target[0];
        Ok()
    }
}
