use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

use crate::save::Save;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hash(String);

impl Save for Hash {}


impl Hash {
    pub fn new(_input: &String) -> Self{
        // let mut sha256 = Sha256::new();
        // sha256.input_str(input);

        // sha256.result_str()
        unimplemented!()
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
