use testa_interpreter::{generator::Record, object::Object};

use crate::{
    error::GeneratorError,
    generator::{FileConfig, FileGenerator},
};

#[derive(Debug, Clone)]
pub struct CsvGenerator {
    delimiter: String,
    quote: String,
    include_header: bool,
}

impl CsvGenerator {
    pub fn new() -> Self {
        Self {
            delimiter: ",".to_string(),
            quote: "\"".to_string(),
            include_header: true,
        }
    }

    pub fn from_config(config: &FileConfig) -> Self {
        let delimiter = config
            .get("delimiter")
            .and_then(|o| match o {
                Object::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| ",".to_string());

        let quote = config
            .get("quote")
            .and_then(|o| match o {
                Object::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "\"".to_string());

        let include_header = config
            .get("header")
            .and_then(|o| match o {
                Object::Boolean(b) => Some(*b),
                _ => None,
            })
            .unwrap_or(true);

        Self {
            delimiter,
            quote,
            include_header,
        }
    }

    fn escape_csv_value(&self, value: &str) -> String {
        if value.contains(&self.delimiter) || value.contains('\n') || value.contains(&self.quote) {
            format!(
                "{}{}{}",
                self.quote,
                value.replace(&self.quote, &format!("{}{}", self.quote, self.quote)),
                self.quote
            )
        } else {
            value.to_string()
        }
    }
}

impl FileGenerator for CsvGenerator {
    fn generate(&self, record: &Record) -> Result<String, GeneratorError> {
        let values: Vec<String> = record
            .values()
            .map(|v| self.escape_csv_value(&format!("{}", v)))
            .collect();

        Ok(values.join(&self.delimiter))
    }

    fn extension(&self) -> &'static str {
        "csv"
    }

    fn generate_header(&self, fields: &[String]) -> Option<String> {
        if self.include_header {
            Some(fields.join(&self.delimiter))
        } else {
            None
        }
    }

    fn generate_footer(&self) -> Option<String> {
        None
    }
}

impl Default for CsvGenerator {
    fn default() -> Self {
        Self::new()
    }
}
