use serde_json::{Serializer, ser::PrettyFormatter};
use serde::Serialize;
use testa_interpreter::{generator::Record, object::Object};

use crate::{error::GeneratorError, generator::{FileConfig, FileGenerator}};

#[derive(Debug)]
pub struct JsonGenerator {
    pretty: bool,
    indent: usize,
}

impl JsonGenerator {
    pub fn new() -> Self {
        Self {
            pretty: false,
            indent: 2,
        }
    }
    pub fn from_config(config: &FileConfig) -> Self {
        let pretty = config.get("pretty")
            .and_then(|o| match o {
                Object::Boolean(b) => Some(*b),
                _ => None
            })
            .unwrap_or(false);
        
        let indent = config.get("indent")
            .and_then(|o| match o {
                Object::Int(i) => Some(*i as usize),
                _ => None
            })
            .unwrap_or(2);
        
        Self { pretty, indent }
    }

    fn object_to_json(obj: &Object) -> serde_json::Value {
        match obj {
            Object::Int(i) => serde_json::Value::Number((*i).into()),
            Object::Float(f) => serde_json::Number::from_f64(*f as f64)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            Object::String(s) => serde_json::Value::String(s.clone()),
            Object::Boolean(b) => serde_json::Value::Bool(*b),
            Object::List(items) => {
                serde_json::Value::Array(items.iter().map(Self::object_to_json).collect())
            }
            Object::NoReturn => serde_json::Value::Null,
            _ => serde_json::Value::Null,
        }
    }
    
    fn indent_block(&self, value: &str) -> String {
        let pad = " ".repeat(self.indent);
        value.lines()
            .map(|line| format!("{pad}{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl FileGenerator for JsonGenerator {
    fn generate(&self, record: &Record) -> Result<String, GeneratorError> {
        let json_map: serde_json::Map<String, serde_json::Value> = record
            .iter()
            .map(|(k, v)| (k.clone(), Self::object_to_json(v)))
            .collect();
        
        let json_value = serde_json::Value::Object(json_map);
        if self.pretty {
            let indent = vec![b' '; self.indent];
            let formatter = PrettyFormatter::with_indent(&indent);
            let mut buf = Vec::new();
            let mut serializer = Serializer::with_formatter(&mut buf, formatter);

            json_value
                .serialize(&mut serializer)
                .map_err(|e| GeneratorError::SerializationError(e.to_string()))?;

            let json_str =  String::from_utf8(buf)
                .map_err(|e| GeneratorError::SerializationError(e.to_string()))?;
            
            Ok(self.indent_block(&json_str))
        } else {
            serde_json::to_string(&json_value)
                .map_err(|e| GeneratorError::SerializationError(e.to_string()))
        }
    }

    fn extension(&self) -> &'static str {
        "json"
    }

    fn separator(&self) -> &str {
        ",\n"
    }

    fn needs_separator(&self) -> bool {
        true
    }

    fn generate_header(&self, _fields: &[String]) -> Option<String> {
        Some("[".to_string())
    }

    fn generate_footer(&self) -> Option<String> {
        Some("\n]".to_string())
    }
}

impl Default for JsonGenerator {
    fn default() -> Self {
        JsonGenerator::new()
    }
}
