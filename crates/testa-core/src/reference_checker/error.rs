use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::utils::Span;

#[derive(Debug)]
pub enum SemanticError {
    UnknownType {
        name: String,
        span: Span,
    },

    UnknownTemplate {
        name: String,
        span: Span,
    },

    UnknownEnum {
        name: String,
        span: Span,
    },

    UnknownVariant {
        name: String,
        enum_name: Option<String>,
        span: Span,
    },

    UnknownField {
        field_name: String,
        template_name: Option<String>,
        span: Span,
    },

    UnknownIdentifier {
        name: String,
        span: Span,
    },

    UnknownParentTemplate {
        name: String,
        span: Span,
    },

    InheritanceCycle {
        template_chain: Vec<String>,
        span: Span,
    },
}

impl SemanticError {
    pub fn to_diagnostic(&self) -> Diagnostic {
        match self {
            Self::UnknownType { name, span } => {
                Diagnostic::error(*span, format!("Unknown type '{}'", name))
                    .with_code(DiagnosticCode::UnknownType)
                    .with_hint("Ensure the type is declared or imported")
            }

            Self::UnknownTemplate { name, span } => {
                Diagnostic::error(*span, format!("Unknown template '{}'", name))
                    .with_code(DiagnosticCode::UnknownTemplate)
                    .with_hint("Declare the template before using it")
            }

            Self::UnknownEnum { name, span } => {
                Diagnostic::error(*span, format!("Unknown enum '{}'", name))
                    .with_code(DiagnosticCode::UnknownEnum)
                    .with_hint("Declare the enum before referencing it")
            }
            Self::UnknownVariant {
                name,
                enum_name,
                span,
            } => {
                let msg = if let Some(e) = enum_name {
                    format!("Unknown variant '{}' in enum '{}'", name, e)
                } else {
                    format!("Unknown enum variant '{}'", name)
                };

                Diagnostic::error(*span, msg)
                    .with_code(DiagnosticCode::UnknownEnumVariant)
                    .with_hint("Check for typos or missing enum declarations")
            }
            Self::UnknownField {
                field_name,
                template_name,
                span,
            } => {
                let msg = if let Some(t) = template_name {
                    format!("Unknown field '{}' in template '{}'", field_name, t)
                } else {
                    format!("Unknown field '{}'", field_name)
                };

                Diagnostic::error(*span, msg)
                    .with_code(DiagnosticCode::UnknownField)
                    .with_hint("Verify that the field exists or is inherited")
            }

            Self::UnknownIdentifier { name, span } => {
                Diagnostic::error(*span, format!("Unknown identifier '{}'", name))
                    .with_code(DiagnosticCode::UnknownIdentifier)
                    .with_hint("Ensure the identifier refers to a declared symbol")
            }

            Self::UnknownParentTemplate { name, span } => {
                Diagnostic::error(*span, format!("Unknown parent template '{}'", name))
                    .with_code(DiagnosticCode::UnknownParentTemplate)
                    .with_hint("Declare the parent template before extending it")
            }

            Self::InheritanceCycle {
                template_chain,
                span,
            } => {
                let chain = template_chain.join(" -> ");

                Diagnostic::error(
                    *span,
                    format!("Template inheritance cycle detected: {}", chain),
                )
                .with_code(DiagnosticCode::InheritanceCycle)
                .with_hint("Remove or restructure the circular inheritance")
            }
        }
    }
}
