use std::collections::HashMap;

use crate::{
    analyser::{
        reference_tracker::Reference, symbol_table::SymbolTable, type_checker::types::Type,
    },
    diagnostics::{Diagnostic, Severity},
};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub symbol_table: SymbolTable,
    pub references: HashMap<String, Vec<Reference>>,
    pub type_map: HashMap<String, Type>,
    pub diagnostics: Vec<Diagnostic>,
}

impl AnalysisResult {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
    }
}
