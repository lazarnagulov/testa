use std::{collections::HashMap, path::PathBuf};

use testa_core::analyser::symbol_table::SymbolTable;

use crate::object::Object;

use super::error::EvalError;

pub trait Visitor<T>: std::fmt::Debug {
    fn visit(&self, context: &Context) -> Result<T, EvalError>;
}

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

#[derive(Debug, Clone, Default)]
pub enum OutputFormat {
    #[default]
    Csv,
    Json,
    Xml,
}
