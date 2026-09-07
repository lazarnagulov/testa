#[derive(Debug)]
pub enum ResolveError {
    NotFound(String),
    Io(String, std::io::Error),
    Deserialize(String, Box<dyn std::error::Error>),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::NotFound(name) => {
                write!(f, "Cannot find module '{}' in search paths", name)
            }
            ResolveError::Io(name, err) => write!(f, "Failed to read module '{}': {}", name, err),
            ResolveError::Deserialize(name, err) => {
                write!(f, "Failed to deserialize module '{}': {}", name, err)
            }
        }
    }
}

impl std::error::Error for ResolveError {}
