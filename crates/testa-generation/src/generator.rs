use core::fmt;

use testa_interpreter::object::Object;

pub trait FileGenerator: fmt::Debug {
    fn generate(&self, record: &Record) -> Result<String, GenerationError>;
    fn extension(&self) -> &'static str;
    fn generate_header(&self) -> Option<String>;
    fn generate_footer(&self) -> Option<String>;
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

// TODO: Should save names, hashmap is not ordered!
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
pub enum GenerationError {
    NotSupported(&'static str),
}
