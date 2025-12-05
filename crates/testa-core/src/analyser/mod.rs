use crate::{
    analyser::{
        result::AnalysisResult,
        symbol::{ReferenceMap, ScopeId, SymbolTable},
    },
    ast::{Program, Statement},
    diagnostics::Diagnostic,
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

    pub fn analyse(&mut self, program: Program) -> AnalysisResult {
        todo!()
    }
}
