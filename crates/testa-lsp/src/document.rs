use testa_core::ast::Program;
use testa_core::diagnostics::Diagnostic;

#[derive(Debug)]
pub struct Document {
    pub text: String,
    pub version: i32,
    pub ast: Option<Program>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Document {
    pub fn new(text: &str, version: i32) -> Self {
        Self {
            text: text.to_string(),
            version,
            ast: None,
            diagnostics: Vec::new(),
        }
    }

    pub fn with_ast(mut self, ast: Program) -> Self {
        self.ast = Some(ast);
        self
    }

    pub fn with_diagnostics(mut self, diagnostics: &[Diagnostic]) -> Self {
        self.diagnostics = diagnostics.to_vec();
        self
    }
}
