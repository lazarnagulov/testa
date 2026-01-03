use core::fmt;
use std::error::Error;

use testa_interpreter::evaluator::Record;

pub trait FileGenerator: fmt::Debug {
    fn generate(&self, record: &Record) -> Result<String, GenerationError>;
    fn extension(&self) -> &'static str;
    fn generate_header(&self, fields: &[String]) -> Option<String>;
    fn generate_footer(&self) -> Option<String>;
    fn needs_separator(&self) -> bool {
        false
    }
    fn separator(&self) -> &str {
        ""
    }
}

// TODO: Add plugin system?
#[derive(Debug, Default)]
pub enum Target {
    #[default]
    Csv,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Csv => write!(f, ".csv"),
        }
    }
}

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
