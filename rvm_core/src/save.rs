use serde::Serialize;
use serde_json;
use std::{fs::File, path::Path, io::Write};


pub trait Save {
    fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>>;
}

impl<T: Serialize> Save for T {
    fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let c = serde_json::to_string(self)?;
        if let Ok(mut f) = File::open(&path) {
            write!(f, "{}", c)?;
            Ok(())
        } else {
            let mut f = File::create(&path)?;
            write!(f, "{}", c)?;
            Ok(())
        }
    }
}