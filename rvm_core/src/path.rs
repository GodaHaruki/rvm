use std::fs::File;
use serde::{Deserialize, Serialize};
use std::path::PathBuf as stdPath;
use std::io::Write;
use serde_json;

use crate::save::Save;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Path{
  Repository(Box<stdPath>),
  Blob(Box<stdPath>),
  Tree(Box<stdPath>),
  Commit(Box<stdPath>),
}

impl Path {
  pub fn new_blob<S: AsRef<std::path::Path>>(path: &S) -> Self {
    let mut p = stdPath::new();
    p.push(path);
    Path::Blob(Box::new(p))
  }

  pub fn new_tree<S: AsRef<std::path::Path>>(path: &S) -> Self {
    let mut p = stdPath::new();
    p.push(path);
    Path::Tree(Box::new(p))
  }

  pub fn new_commit<S: AsRef<std::path::Path>>(path: &S) -> Self {
    let mut p = stdPath::new();
    p.push(path);
    Path::Commit(Box::new(p))
  }

  pub fn new_repository<S: AsRef<std::path::Path>>(path: &S) -> Self {
    let mut p = stdPath::new();
    p.push(path);
    Path::Repository(Box::new(p))
  }

  pub fn open(&self) -> std::io::Result<File>{
    File::open(self.get_path())
  }

  #[inline]
  pub fn get_path(&self) -> &stdPath {
    use self::Path::*;
    match self {
      Repository(path) => path.as_ref(),
      Blob(path) => path.as_ref(),
      Tree(path) => path.as_ref(),
      Commit(path) => path.as_ref(),
    }
  }

  pub fn get_path_as_str(&self) -> Option<&str> {
    self.get_path().to_str()
  }

  pub fn create(&self) -> std::io::Result<File> {
    let mut file = File::create(self.get_path());
    return file;
  }

  pub fn save<T: Serialize>(&self, c: &T) -> Result<(), Box<dyn std::error::Error>>{
    if let Ok(mut f) = self.open() {
      write!(f, "{}", serde_json::to_string(c)?)?;
      f.flush()?;
      Ok(())
    } else {
      let mut f = self.create()?;
      write!(f, "{}", serde_json::to_string(c)?)?;
      f.flush()?;
      Ok(())
    }
  }
}

