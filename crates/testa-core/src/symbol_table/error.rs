use crate::{
    diagnostics::{Diagnostic, DiagnosticCode},
    utils::Span,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolError {
    DuplicateDeclaration { span: Span, message: String },
    InvalidParent { span: Span, message: String },
    EmptyEnum { span: Span, message: String },
    DuplicateVariant { span: Span, message: String },
    InvalidContext { span: Span, message: String },
}

impl SymbolError {
    pub fn to_diagnostic(&self) -> Diagnostic {
        match self {
            Self::DuplicateDeclaration { span, message } => {
                Diagnostic::error(*span, message.clone())
                    .with_code(DiagnosticCode::DuplicateDeclaration)
                    .with_hint("Each symbol name must be unique within its scope")
            }

            Self::InvalidParent { span, message } => Diagnostic::error(*span, message.clone())
                .with_code(DiagnosticCode::InvalidParent)
                .with_hint("Parent template must be declared before it can be extended"),

            Self::EmptyEnum { span, message } => Diagnostic::error(*span, message.clone())
                .with_code(DiagnosticCode::EmptyEnum)
                .with_hint("Enums must have at least one variant"),

            Self::DuplicateVariant { span, message } => Diagnostic::error(*span, message.clone())
                .with_code(DiagnosticCode::DuplicateVariant)
                .with_hint("Each variant name must be unique within the enum"),

            Self::InvalidContext { span, message } => Diagnostic::error(*span, message.clone())
                .with_code(DiagnosticCode::InvalidContext)
                .with_hint("This declaration is not valid in the current scope"),
        }
    }
}
