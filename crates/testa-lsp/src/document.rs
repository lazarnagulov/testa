use testa_core::analyser::symbol_table::SymbolTable;
use testa_core::ast::Program;
use testa_core::diagnostics::Diagnostic;

#[derive(Debug)]
pub struct Document {
    pub text: String,
    pub version: i32,
    pub ast: Option<Program>,
    pub diagnostics: Vec<Diagnostic>,
    pub symbol_table: Option<SymbolTable>,
}

impl Document {
    pub fn new(text: &str, version: i32) -> Self {
        Self {
            text: text.to_string(),
            version,
            ast: None,
            diagnostics: Vec::new(),
            symbol_table: None,
        }
    }

    pub fn with_ast(mut self, ast: Option<Program>) -> Self {
        self.ast = ast;
        self
    }

    pub fn with_diagnostics(mut self, diagnostics: &[Diagnostic]) -> Self {
        self.diagnostics = diagnostics.to_vec();
        self
    }

    pub fn with_symbol_table(mut self, symbol_table: Option<SymbolTable>) -> Self {
        self.symbol_table = symbol_table;
        self
    }
}
