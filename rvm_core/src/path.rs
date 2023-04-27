use std::fs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Path {
  Blob(String),
  Tree(String),
  Commit(String),
}

impl Path {
  pub fn open(&self) -> std::io::Result<File>{
    match self {
      Self::Blob(path) => File::open(path),
      Self::Tree(path) => File::open(path),
      Self::Commit(path) => File::open(path),
    }
  }
}
