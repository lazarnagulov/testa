use core::fmt;
use std::collections::HashMap;

use testa_interpreter::{
    evaluator::{Record, context::OutputFormat},
    object::Object,
};

use crate::{csv::CsvGenerator, error::GenerationError};

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

pub fn create_file_generator(
    format: &OutputFormat,
    config: &HashMap<String, Object>,
) -> Box<dyn FileGenerator> {
    match format {
        OutputFormat::Csv => Box::new(CsvGenerator::from_config(config)),
        OutputFormat::Json => todo!("implement json generator"),
        OutputFormat::Xml => todo!("implement xml generator"),
        OutputFormat::Sql => todo!("implement sql generator"),
    }
}
