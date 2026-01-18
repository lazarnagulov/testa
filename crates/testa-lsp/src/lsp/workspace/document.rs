#![allow(unused)]

use std::sync::Arc;

use testa_core::analyser::symbol_table::SymbolTable;
use testa_core::ast::Program;
use testa_core::diagnostics::Diagnostic;
use tower_lsp::lsp_types::Url;

#[derive(Debug)]
pub struct Analysis {
    pub ast: Option<Program>,
    pub symbol_table: Option<SymbolTable>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub uri: Url,
    pub text: Arc<str>,
    pub version: i32,
    pub analysis: Option<Arc<Analysis>>,
}

impl Document {
    pub fn new(uri: Url, text: String, version: i32) -> Self {
        Self {
            uri,
            version,
            text: Arc::from(text),
            analysis: None,
        }
    }

    pub fn with_analysis(mut self, analysis: Analysis) -> Self {
        self.analysis = Some(Arc::new(analysis));
        self
    }
}
