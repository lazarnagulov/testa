use crate::{
    analyser::result::AnalysisResult,
    ast::Program,
    diagnostics::Diagnostic,
    symbol_table::{
        SymbolTable,
        symbol::{ReferenceMap, ScopeId},
    },
    utils::Span,
};

pub mod result;

#[derive(Default, Debug)]
pub struct SemanticAnalyser {
    symbol_table: SymbolTable,
    _scopes: Vec<ScopeId>,
    diagnostics: Vec<Diagnostic>,
    references: ReferenceMap,
}

impl SemanticAnalyser {
    pub fn new(symbol_table: SymbolTable) -> Self {
        let global_scope = symbol_table.global_scope();

        Self {
            symbol_table,
            _scopes: vec![global_scope],
            diagnostics: Vec::new(),
            references: ReferenceMap::new(),
        }
    }

    pub fn analyse(&mut self, _program: Program) -> AnalysisResult {
        // self.visit_program(&program);

        AnalysisResult {
            symbol_table: self.symbol_table.clone(),
            diagnostics: self.diagnostics.clone(),
            references: self.references.clone(),
        }
    }

    fn _error(&mut self, message: &str, span: Span) {
        self.diagnostics.push(Diagnostic::error(span, message));
    }
}
