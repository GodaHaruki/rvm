use std::ffi::OsStr;
use std::fs::{self, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf as stdPath;
use std::io::{Write};
use toml;


#[derive(Debug, Serialize, Deserialize)]
pub enum Path{
  Repository(Box<stdPath>),
  Blob(Box<stdPath>),
  Tree(Box<stdPath>),
  Commit(Box<stdPath>),
}

impl Path {
  pub fn new_blob<S: AsRef<std::path::Path>>(path: &S) -> Self {
    let mut path = stdPath::new();
    path.push(path);
    Path::Blob(Box::new(path))
  }

  pub fn new_tree<S: AsRef<OsStr> + ?Sized>(path: &S) -> Self {
    let mut path = stdPath::new();
    path.push(path);
    Path::Tree(Box::new(path))
  }

  pub fn new_commit<S: AsRef<OsStr> + ?Sized>(path: &S) -> Self {
    let mut path = stdPath::new();
    path.push(path);
    Path::Commit(Box::new(path))
  }

  pub fn new_repository<S: AsRef<OsStr> + ?Sized>(path: &S) -> Self {
    let mut path = stdPath::new();
    path.push(path);
    Path::Repository(Box::new(path))
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

  pub fn create(&self) -> std::io::Result<File> {
    let mut file = File::create(self.get_path());
    return file;
  }

  pub fn save<T: Serialize>(&self, c: T) -> Result<(), Box<dyn std::error::Error>>{
    if let Ok(mut f) = self.open() {
      write!(f, "{}", toml::to_string(&c)?)?;
      f.flush()?;
      Ok(())
    } else {
      let mut f = self.create()?;
      write!(f, "{}", toml::to_string(&c)?)?;
      f.flush()?;
      Ok(())
    }
  }

  pub fn read_as<'a, T: Deserialize<'a>>(&self) -> Result<T, Box<dyn std::error::Error>>{
    let s = fs::read_to_string(self.get_path())?;
    let res: T = toml::from_str(&s)?;
    Ok(res)
  }
}
