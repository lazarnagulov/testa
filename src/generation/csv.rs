use std::collections::HashMap;

use crate::interpreter::object::Object;

use super::generator::{FileGenerator, GenerationError, Record};

#[derive(Debug)]
pub struct CsvGenerator<'a> {
    delimiter: String,
    header: bool,
    quote: bool,

    field_names: Vec<&'a str>,
}

impl Default for CsvGenerator<'_> {
    fn default() -> Self {
        Self {
            delimiter: ",".to_owned(),
            header: true,
            quote: false,
            field_names: Vec::new(),
        }
    }
}

impl<'a> CsvGenerator<'a> {
    pub fn with_field_names(mut self, field_names: Vec<&'a str>) -> Self {
        self.field_names = field_names;
        self
    }
    pub fn with_config(mut self, config: &HashMap<String, Object>) -> Self {
        config.iter().for_each(|(key, value)| match key.as_str() {
            "delimiter" => {
                if let Object::String(s) = value {
                    self.delimiter = s.clone();
                } else {
                    eprintln!("WARNING: Expected string for 'delimiter'");
                }
            }
            "header" => {
                if let Object::Boolean(b) = value {
                    self.header = *b;
                } else {
                    eprintln!("WARNING: Expected boolean for 'header'");
                }
            }
            "quote" => {
                if let Object::Boolean(b) = value {
                    self.quote = *b;
                } else {
                    eprintln!("WARNING: Expected boolean for 'quote'");
                }
            }
            _ => {
                eprintln!("WARNING: Unknown config key: {}", key);
            }
        });
        self
    }
}

impl FileGenerator for CsvGenerator<'_> {
    fn generate(&self, record: &Record) -> Result<String, GenerationError> {
        Ok(record
            .fields
            .iter()
            .map(|object| match object {
                Object::Int(_) | Object::Float(_) | Object::Boolean(_) | Object::List(_) => {
                    Ok(format!("{}", object))
                }
                Object::String(value) => {
                    if self.quote {
                        Ok(format!("\"{}\"", value))
                    } else {
                        Ok(value.to_string())
                    }
                }
                Object::Range(_, _) => Err(GenerationError::NotSupported("Range")),
                Object::NoReturn => Err(GenerationError::NotSupported("NoReturn")),
            })
            .collect::<Result<Vec<String>, GenerationError>>()?
            .join(&self.delimiter))
    }

    fn extension(&self) -> &'static str {
        "csv"
    }

    fn generate_header(&self) -> Option<String> {
        self.header.then(|| self.field_names.join(&self.delimiter))
    }

    fn generate_footer(&self) -> Option<String> {
        None
    }
}
