#![allow(unused)]

use std::collections::HashMap;
use std::sync::Arc;

use testa_core::analyser::reference_tracker::Reference;
use testa_core::analyser::symbol_table::SymbolTable;
use testa_core::ast::Program;
use testa_core::diagnostics::Diagnostic;
use testa_hir::Module;
use tower_lsp::jsonrpc::{self, Error};
use tower_lsp::lsp_types::Url;

#[derive(Debug)]
pub struct Analysis {
    pub ast: Option<Program>,
    pub symbol_table: Option<SymbolTable>,
    pub references: HashMap<String, Vec<Reference>>,
    pub diagnostics: Vec<Diagnostic>,
    pub imported_modules: HashMap<String, Module>,
}

impl Analysis {
    pub fn find_reference_at(&self, line: u32, column: u32) -> Option<&Reference> {
        for refs in self.references.values() {
            for reference in refs {
                if reference.span.contains_position(line, column) {
                    return Some(reference);
                }
            }
        }
        None
    }

    pub fn get_references(&self, name: &str) -> Option<&Vec<Reference>> {
        self.references.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub uri: Url,
    pub text: Arc<str>,
    pub version: i32,
    analysis: Option<Arc<Analysis>>,
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

    pub fn get_analysis(&self) -> Result<&Arc<Analysis>, jsonrpc::Error> {
        self.analysis.as_ref().ok_or_else(Error::invalid_request)
    }
}
