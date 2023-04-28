use std::fs::*;
use serde::{Deserialize, Serialize};
use std::path::Path as stdPath;

#[derive(Debug, Serialize, Deserialize)]
pub enum Path {
  Repository(Box<stdPath>),
  Blob(Box<stdPath>),
  Tree(Box<stdPath>),
  Commit(Box<stdPath>),
}

impl Path {
  pub fn new_blob(path: String) -> Self {
    Blob(Box::new(stdPath::new(path)))
  }

  pub fn new_tree(path: String) -> Self {
    Tree(Box::new(stdPath::new(path)))
  }

  pub fn new_commit(path: String) -> Self {
    Commit(Box::new(stdPath::new(path)))
  }

  pub fn new_repository(path: String) -> Self {
    Repository(Box::new(stdPath::new(path)))
  }

  pub fn open(&self) -> std::io::Result<File>{
    use self::Path::*;
    match self {
      Repository(path) => File::open(path),
      Blob(path) => File::open(path),
      Tree(path) => File::open(path),
      Commit(path) => File::open(path),
    }
  }

  pub fn save<'a, T: Deserialize<'a>>(&self, c: T) -> Result<(), ()>{
    todo!()
  }

  pub fn read_as<T: Serialize>(&self) -> T {
    todo!()
  }
}
