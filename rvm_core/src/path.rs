use std::fs::*;
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::path::Path as stdPath;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub enum Path<'a> {
  Repository(Box<&'a stdPath>),
  Blob(Box<&'a stdPath>),
  Tree(Box<&'a stdPath>),
  Commit(Box<&'a stdPath>),
}

impl<'a> Path<'a> {
  pub fn new_blob(path: &stdPath) -> Self {
    Path::Blob(Box::new(path))
  }

  pub fn new_tree(path: &stdPath) -> Self {
    Path::Tree(Box::new(path))
  }

  pub fn new_commit(path: &stdPath) -> Self {
    Path::Commit(Box::new(path))
  }

  pub fn new_repository(path: &stdPath) -> Self {
    Path::Repository(Box::new(path))
  }

  pub fn open(&self) -> std::io::Result<File>{
    File::open(self.get_path())
  }

  #[inline]
  pub fn get_path(&self) -> &stdPath{
    use self::Path::*;
    match self {
      Repository(path) => path,
      Blob(path) => path,
      Tree(path) => path,
      Commit(path) => path,
    }
  }

  pub fn create(&self) -> std::io::Result<File> {
    let mut file = File::create(self.get_path());
    return file;
  }

  pub fn save<T: Serialize>(&self, c: T) -> Result<(), Box<dyn std::error::Error>>{
    if let Ok(mut f) = self.open(){
      write!(f, "{}", toml::to_string(&c)?);
      Ok(())
    } else {
      let mut file = self.create()?;
      write!(file, "{}", toml::to_string(&c)?)?;
      file.flush()?;
      Ok(())
    }
  }

  pub fn read_as<T: Deserialize<'a>>(&self) -> std::io::Result<T> {
    todo!()
  }
}
