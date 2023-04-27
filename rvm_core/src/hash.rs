use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hash(String);

impl Hash {
    pub fn new(input: &str) -> Self{
        // let mut sha256 = Sha256::new();
        // sha256.input_str(input);

        // sha256.result_str()
        unimplemented!()
    }
}
