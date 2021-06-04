use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SolDef {
    pub bytecode: String,
    #[serde(rename = "deployedBytecode")]
    pub deployed_bytecode: String,
}

impl SolDef {
    pub fn from_path(p: &str) -> Self {
        let mut file = File::open(p).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_serde() {
        let mut file = File::open("contracts/Box.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let sd: SolDef = serde_json::from_str(&contents).unwrap();
        println!("{:?}", sd);
    }
}
