use std::{collections::HashMap, path::PathBuf, str::FromStr};

use rand::{SeedableRng, rngs::StdRng};
use testa_core::analyser::symbol_table::{
    SymbolTable,
    symbol::{Scope, ScopeKind},
};

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct GenerateOptions {
    pub input: PathBuf,
    pub output_path: Option<PathBuf>,
    pub format: Option<OutputFormat>,
    pub count: Option<usize>,
    pub seed: Option<u64>,
}

#[derive(Debug)]
pub struct Context {
    pub output_path: Option<PathBuf>,
    pub output_options: HashMap<String, Object>,
    pub symbol_table: SymbolTable,
    pub output_format: OutputFormat,
}

pub struct State {
    pub rng: StdRng,
}

impl State {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = seed
            .map(StdRng::seed_from_u64)
            .unwrap_or_else(|| StdRng::from_rng(&mut rand::rng()));
        Self { rng }
    }
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

    pub fn with_options(mut self, options: GenerateOptions) -> Self {
        self.output_path = options.output_path;
        if let Some(format) = options.format {
            self.output_format = format;
        }
        self
    }

    pub fn find_template_scope(&self, template_name: &str) -> Option<&Scope> {
        self.symbol_table.scopes().iter().find(
            |scope| matches!(&scope.kind, ScopeKind::Template { name } if name == template_name),
        )
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
