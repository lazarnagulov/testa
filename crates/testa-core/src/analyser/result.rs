use crate::{
    analyser::symbol_table::SymbolTable, diagnostics::{Diagnostic, Severity}, utils::Span
};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub symbol_table: SymbolTable,
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

#[derive(Debug, Clone)]
pub struct HoverInfo {
    pub content: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CompletionKind {
    Template,
    Enum,
    Field,
    Keyword,
    Function,
    Type,
}
