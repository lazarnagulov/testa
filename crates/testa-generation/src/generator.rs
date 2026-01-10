use testa_interpreter::{
    evaluator::context::OutputFormat,
    generator::{FileConfig, FileGenerator},
};

use crate::format::{
    csv::CsvGenerator, json::JsonGenerator, sql_insert::SqlInsertGenerator, xml::XmlGenerator,
};

pub fn create_file_generator(format: &OutputFormat, config: &FileConfig) -> Box<dyn FileGenerator> {
    match format {
        OutputFormat::Csv => Box::new(CsvGenerator::from_config(config)),
        OutputFormat::Json => Box::new(JsonGenerator::from_config(config)),
        OutputFormat::Sql => Box::new(SqlInsertGenerator::from_config(config)),
        OutputFormat::Xml => Box::new(XmlGenerator::from_config(config)),
    }
}
