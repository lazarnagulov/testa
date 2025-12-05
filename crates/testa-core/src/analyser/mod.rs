use crate::{
    analyser::{
        result::AnalysisResult,
        symbol::{ReferenceMap, ScopeId, SymbolTable},
    },
    ast::Program,
    diagnostics::Diagnostic,
    utils::Span,
};

pub mod result;
pub mod symbol;

#[derive(Default, Debug)]
pub struct SemanticAnalyser {
    symbol_table: SymbolTable,
    scopes: Vec<ScopeId>,
    diagnostics: Vec<Diagnostic>,
    references: ReferenceMap,
}

impl SemanticAnalyser {
    pub fn new() -> Self {
        let symbol_table = SymbolTable::new();
        let global_scope = symbol_table.global_scope();

        Self {
            symbol_table,
            scopes: vec![global_scope],
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

    fn _error(&mut self, message: String, span: Span) {
        self.diagnostics.push(Diagnostic::error(span, message));
    }
}
