use std::{collections::HashMap, path::PathBuf, str::FromStr};

use testa_core::analyser::symbol_table::SymbolTable;

use crate::object::Object;

#[derive(Debug)]
pub struct Context {
    pub output_path: Option<PathBuf>,
    pub output_options: HashMap<String, Object>,
    pub symbol_table: SymbolTable,
    pub output_format: OutputFormat,
}

impl Context {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Self {
            symbol_table,
            output_path: None,
            output_options: HashMap::new(),
            output_format: OutputFormat::default(),
        }
    }

    pub fn with_output_path(mut self, path: PathBuf) -> Self {
        self.output_path = Some(path);
        self
    }

    pub fn with_output_options(mut self, config: HashMap<String, Object>) -> Self {
        self.output_options = config;
        self
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum OutputFormat {
    #[default]
    Csv,
    Json,
    Sql,
    Xml,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Csv => "csv",
            OutputFormat::Json => "json",
            OutputFormat::Sql => "sql",
            OutputFormat::Xml => "xml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("csv") => Ok(OutputFormat::Csv),
            s if s.eq_ignore_ascii_case("json") => Ok(OutputFormat::Json),
            s if s.eq_ignore_ascii_case("sql") => Ok(OutputFormat::Sql),
            s if s.eq_ignore_ascii_case("xml") => Ok(OutputFormat::Xml),
            other => Err(format!("Unknown output format: {}", other)),
        }
    }
}
