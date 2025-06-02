use core::fmt;

use crate::evaluator::object::Object;

pub trait FileGenerator : fmt::Debug + Default {
    fn generate(&self, record: &Record) -> Result<String, GenerationError>;
    fn extension(&self) -> &'static str;
    fn generate_header(&self) -> Option<String>;
    fn generate_footer(&self) -> Option<String>;
}

// TODO: Add plugin system?
#[derive(Debug)]
pub enum Target {
    Csv, 
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Csv => write!(f, ".csv"),
        }
    }
}


impl Default for Target {
    fn default() -> Self {
        Target::Csv
    }
}


#[derive(Debug)]
pub struct Record {
    pub fields: Vec<Object>,
}

impl Record {
    pub fn new(fields: Vec<Object>) -> Self {
        Record { fields }
    }
}

#[derive(Debug)]
pub struct Config {}

#[derive(Debug)]
pub enum GenerationError {
    NotSupported(&'static str),
}
