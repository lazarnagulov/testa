use testa_interpreter::{generator::Record, object::Object};

use crate::generator::{FileConfig, FileGenerator};

#[derive(Debug)]
pub struct SqlInsertGenerator {
    table_name: String,
}

impl SqlInsertGenerator {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
        }
    }

    pub fn from_config(config: &FileConfig) -> Self {
        let table_name = config
            .get("table")
            .and_then(|o| match o {
                Object::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "generated_data".to_string());

        Self { table_name }
    }

    fn escape_sql_value(&self, object: &Object) -> String {
        match object {
            Object::Int(i) => i.to_string(),
            Object::Float(f) => f.to_string(),
            Object::Boolean(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
            Object::String(s) => format!("'{}'", s.replace("'", "''")),
            _ => "NULL".to_string(),
        }
    }
}

impl FileGenerator for SqlInsertGenerator {
    fn generate(&self, record: &Record) -> Result<String, crate::error::GeneratorError> {
        let columns: Vec<String> = record.keys().cloned().collect();
        let values: Vec<String> = record.values().map(|v| self.escape_sql_value(v)).collect();

        Ok(format!(
            "INSERT INTO {} ({}) VALUES ({});\n",
            self.table_name,
            columns.join(", "),
            values.join(", ")
        ))
    }

    fn extension(&self) -> &'static str {
        "sql"
    }

    fn generate_header(&self, _fields: &[String]) -> Option<String> {
        Some(format!(
            "-- Generated SQL inserts for table {}",
            self.table_name
        ))
    }

    fn generate_footer(&self) -> Option<String> {
        Some("-- End of generated data".to_string())
    }
}

impl Default for SqlInsertGenerator {
    fn default() -> Self {
        SqlInsertGenerator::new("generated_data")
    }
}
