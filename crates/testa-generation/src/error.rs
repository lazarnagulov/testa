use std::{error::Error, fmt};

#[derive(Debug)]
pub enum GenerationError {
    NotSupported(&'static str),
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationError::NotSupported(message) => write!(f, "{}", message),
        }
    }
}

impl Error for GenerationError {}
