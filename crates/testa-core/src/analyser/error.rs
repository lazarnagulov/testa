use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::utils::Span;

#[derive(Debug, Clone)]
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
        template_chain: String,
        span: Span,
    },

    DuplicateDeclaration {
        span: Span,
        message: String,
    },
    InvalidParent {
        span: Span,
        message: String,
    },
    EmptyEnum {
        span: Span,
        message: String,
    },
    DuplicateVariant {
        span: Span,
        message: String,
    },
    InvalidContext {
        span: Span,
        message: String,
    },

    TypeMismatch {
        expected: String,
        found: String,
        span: Span,
    },

    InvalidBinaryOperator {
        operator: String,
        left_type: String,
        right_type: String,
        span: Span,
    },

    InvalidUnaryOperator {
        operator: String,
        operand_type: String,
        span: Span,
    },

    InvalidConstraintForType {
        constraint: String,
        type_name: String,
        span: Span,
    },

    InvalidConstraintValue {
        constraint: String,
        expected: String,
        found: String,
        span: Span,
    },
}

impl SemanticError {
    pub fn to_diagnostic(&self) -> Diagnostic {
        match self {
            SemanticError::UnknownType { name, span } => {
                Diagnostic::error(*span, format!("Unknown type '{}'", name))
                    .with_code(DiagnosticCode::UnknownType)
                    .with_hint("Ensure the type is declared or imported")
            }
            SemanticError::UnknownTemplate { name, span } => {
                Diagnostic::error(*span, format!("Unknown template '{}'", name))
                    .with_code(DiagnosticCode::UnknownTemplate)
                    .with_hint("Declare the template before using it")
            }
            SemanticError::UnknownEnum { name, span } => {
                Diagnostic::error(*span, format!("Unknown enum '{}'", name))
                    .with_code(DiagnosticCode::UnknownEnum)
                    .with_hint("Declare the enum before referencing it")
            }
            SemanticError::UnknownVariant {
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
            SemanticError::UnknownField {
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
            SemanticError::UnknownIdentifier { name, span } => {
                Diagnostic::error(*span, format!("Unknown identifier '{}'", name))
                    .with_code(DiagnosticCode::UnknownIdentifier)
                    .with_hint("Ensure the identifier refers to a declared symbol")
            }
            SemanticError::UnknownParentTemplate { name, span } => {
                Diagnostic::error(*span, format!("Unknown parent template '{}'", name))
                    .with_code(DiagnosticCode::UnknownParentTemplate)
                    .with_hint("Declare the parent template before extending it")
            }
            SemanticError::InheritanceCycle {
                template_chain,
                span,
            } => Diagnostic::error(
                *span,
                format!("Template inheritance cycle detected: {}", template_chain),
            )
            .with_code(DiagnosticCode::InheritanceCycle)
            .with_hint("Remove or restructure the circular inheritance"),
            SemanticError::DuplicateDeclaration { span, message } => {
                Diagnostic::error(*span, message.clone())
                    .with_code(DiagnosticCode::DuplicateDeclaration)
                    .with_hint("Each symbol name must be unique within its scope")
            }
            SemanticError::InvalidParent { span, message } => {
                Diagnostic::error(*span, message.clone())
                    .with_code(DiagnosticCode::InvalidParent)
                    .with_hint("Parent template must be declared before it can be extended")
            }
            SemanticError::EmptyEnum { span, message } => Diagnostic::error(*span, message.clone())
                .with_code(DiagnosticCode::EmptyEnum)
                .with_hint("Enums must have at least one variant"),
            SemanticError::DuplicateVariant { span, message } => {
                Diagnostic::error(*span, message.clone())
                    .with_code(DiagnosticCode::DuplicateVariant)
                    .with_hint("Each variant name must be unique within the enum")
            }
            SemanticError::InvalidContext { span, message } => {
                Diagnostic::error(*span, message.clone())
                    .with_code(DiagnosticCode::InvalidContext)
                    .with_hint("This declaration is not valid in the current scope")
            }
            SemanticError::TypeMismatch {
                expected,
                found,
                span,
            } => Diagnostic::error(
                *span,
                format!("Type mismatch: expected '{}', found '{}'", expected, found),
            )
            .with_code(DiagnosticCode::TypeMismatch)
            .with_hint("Ensure the expression matches the expected type"),

            SemanticError::InvalidBinaryOperator {
                operator,
                left_type,
                right_type,
                span,
            } => Diagnostic::error(
                *span,
                format!(
                    "Invalid binary operation: '{}' cannot be applied to '{}' and '{}'",
                    operator, left_type, right_type
                ),
            )
            .with_code(DiagnosticCode::InvalidBinaryOperator)
            .with_hint("Check the operator and operand types"),

            SemanticError::InvalidUnaryOperator {
                operator,
                operand_type,
                span,
            } => Diagnostic::error(
                *span,
                format!(
                    "Invalid unary operation: '{}' cannot be applied to '{}'",
                    operator, operand_type
                ),
            )
            .with_code(DiagnosticCode::InvalidUnaryOperator)
            .with_hint("Check the operator and operand type"),

            SemanticError::InvalidConstraintForType {
                constraint,
                type_name,
                span,
            } => Diagnostic::error(
                *span,
                format!(
                    "Invalid constraint '{}' for type '{}'",
                    constraint, type_name
                ),
            )
            .with_code(DiagnosticCode::InvalidConstraintForType)
            .with_hint("Remove the constraint or apply it to a compatible type"),

            SemanticError::InvalidConstraintValue {
                constraint,
                expected,
                found,
                span,
            } => Diagnostic::error(
                *span,
                format!(
                    "Invalid value for constraint '{}': expected {}, found {}",
                    constraint, expected, found
                ),
            )
            .with_code(DiagnosticCode::InvalidConstraintValue)
            .with_hint("Adjust the value to satisfy the constraint requirements"),
        }
    }
}
