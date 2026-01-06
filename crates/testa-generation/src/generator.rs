use core::fmt;
use std::collections::HashMap;

use testa_interpreter::{evaluator::context::OutputFormat, generator::Record, object::Object};

use crate::{csv::CsvGenerator, error::GeneratorError, json::JsonGenerator, sql_insert::SqlInsertGenerator};

pub type FileConfig = HashMap<String, Object>;

pub trait FileGenerator: fmt::Debug {
    fn generate(&self, record: &Record) -> Result<String, GeneratorError>;
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

pub fn create_file_generator(format: &OutputFormat, config: &FileConfig) -> Box<dyn FileGenerator> {
    match format {
        OutputFormat::Csv => Box::new(CsvGenerator::from_config(config)),
        OutputFormat::Json => Box::new(JsonGenerator::from_config(config)),
        OutputFormat::Sql => Box::new(SqlInsertGenerator::from_config(config)),
        OutputFormat::Xml => todo!("implement xml generator"),
    }
}
