use std::{error::Error, fmt};

#[derive(Debug)]
pub enum GeneratorError {
    NotSupported(String),
    SerializationError(String),
    EvaluationError(String),
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::NotSupported(message)
            | GeneratorError::SerializationError(message)
            | GeneratorError::EvaluationError(message) => {
                write!(f, "{}", message)
            }
        }
    }
}

impl Error for GeneratorError {}
