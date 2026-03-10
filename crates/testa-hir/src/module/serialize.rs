use std::{
    fs::File,
    io::{Read as _, Write},
    path::Path,
};

use crate::{Module, module::error::ResolveError};

impl Module {
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = rmp_serde::to_vec(self)?;
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, ResolveError> {
        let mut file = File::open(path).map_err(|err| ResolveError::Io("Failed to open {path}".to_string(), err))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|err| ResolveError::Io("Failed to read file at {path}".to_string(), err))?;
        let module = rmp_serde::from_slice(&buffer)
            .map_err(|err| ResolveError::Deserialize("Failed to deserialize".to_string(), Box::new(err)))?;
        Ok(module)
    }
}
