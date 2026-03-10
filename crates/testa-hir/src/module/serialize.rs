use std::{
    fs::File,
    io::{Read as _, Write},
};

use crate::Module;

impl Module {
    pub fn save(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = rmp_serde::to_vec(self)?;
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let module = rmp_serde::from_slice(&buffer)?;
        Ok(module)
    }
}
