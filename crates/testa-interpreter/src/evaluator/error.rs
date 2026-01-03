use core::fmt;
use std::error::Error;

use testa_core::{
    ast::{InfixOperator, PrefixOperator}, diagnostics::{Diagnostic, DiagnosticCode}, utils::Span
};

use crate::object::Object;

#[derive(Debug)]
pub enum EvalError {
    UnsupportedPrefixOperator {
        operator: PrefixOperator,
        object: Object,
        span: Span,
    },
    UnsupportedInfixOperand {
        left: Object,
        operator: InfixOperator,
        right: Object,
        span: Span,
    },
    TypeMismatch {
        expected: String,
        got: String,
        span: Span,
    },
    DivisionByZero {
        span: Span,
    },
    NotDefined(String, Span),
    UncompatibleConstraint {
        data_type: String,
        constraint: String,
        span: Span,
    },
    InvalidTarget(String, Span),
    FileError(String, Span),
    MiscellaneousError(String, Span),
}

impl EvalError {
    pub fn unsupported_prefix_operator<T: Into<Object>>(
        operator: PrefixOperator,
        object: T,
    ) -> Self {
        EvalError::UnsupportedPrefixOperator {
            operator,
            object: object.into(),
            span: Span::default(),
        }
    }

    pub fn incompatible_constraint(data_type: &str, constraint: &str) -> Self {
        EvalError::UncompatibleConstraint {
            constraint: constraint.to_owned(),
            data_type: data_type.to_owned(),
            span: Span::default(),
        }
    }

    pub fn unsupported_infix_operator<T: Into<Object>>(
        left: T,
        operator: InfixOperator,
        right: T,
    ) -> Self {
        EvalError::UnsupportedInfixOperand {
            left: left.into(),
            operator,
            right: right.into(),
            span: Span::default(),
        }
    }

    pub fn type_mismatch(expected: String, got: String) -> Self {
        EvalError::TypeMismatch {
            expected,
            got,
            span: Span::default(),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UnsupportedPrefixOperator { operator, span, .. } => {
                write!(
                    f,
                    "Unsupported prefix operator '{}' on object at {:?}",
                    operator, span
                )
            }
            EvalError::UnsupportedInfixOperand {
                left,
                operator,
                right,
                span,
            } => {
                write!(
                    f,
                    "Unsupported infix operation: '{}' between {:?} and {:?} at {:?}",
                    operator, left, right, span
                )
            }
            EvalError::TypeMismatch {
                expected,
                got,
                span,
            } => {
                write!(
                    f,
                    "Type mismatch: expected '{}' but got '{}' at {:?}",
                    expected, got, span
                )
            }
            EvalError::NotDefined(name, span) => {
                write!(f, "Variable '{}' not defined at {:?}", name, span)
            }
            EvalError::UncompatibleConstraint {
                data_type,
                constraint,
                span,
            } => {
                write!(
                    f,
                    "Incompatible constraint '{}' for type '{}' at {:?}",
                    constraint, data_type, span
                )
            }
            EvalError::InvalidTarget(target, span) => {
                write!(f, "Invalid target '{}' at {:?}", target, span)
            }
            EvalError::FileError(message, span) => {
                write!(f, "File error: '{}' at {:?}", message, span)
            }
            EvalError::MiscellaneousError(message, span) => {
                write!(f, "Miscellaneous error: '{}' at {:?}", message, span)
            }
            EvalError::DivisionByZero { span } => {
                write!(f, "Cannot divide by zero at {:?}", span)
            },
        }
    }
}

impl EvalError {
    pub fn to_diagnostic(&self) -> Diagnostic {
        match self {
            Self::UnsupportedPrefixOperator { operator, object, span } => {
                Diagnostic::error(
                    *span,
                    format!(
                        "Unsupported prefix operator '{}' for {}",
                        operator, object
                    ),
                )
                .with_code(DiagnosticCode::UnsupportedPrefixOperator)
                .with_hint("Check the operand type or use a supported operator")
            }

            Self::UnsupportedInfixOperand {
                left,
                operator,
                right,
                span,
            } => {
                Diagnostic::error(
                    *span,
                    format!(
                        "Unsupported operation: {} {} {}",
                        left, operator, right
                    ),
                )
                .with_code(DiagnosticCode::UnsupportedInfixOperand)
                .with_hint("Verify that both operands support this operator")
            }

            Self::TypeMismatch { expected, got, span } => {
                Diagnostic::error(
                    *span,
                    format!("Type mismatch: expected {}, got {}", expected, got),
                )
                .with_code(DiagnosticCode::TypeMismatch)
                .with_hint("Ensure the expression evaluates to the expected type")
            }

            Self::DivisionByZero { span } => {
                Diagnostic::error(*span, "Division by zero".to_string())
                    .with_code(DiagnosticCode::DivisionByZero)
                    .with_hint("Ensure the divisor is not zero")
            }

            Self::NotDefined(name, span) => {
                Diagnostic::error(*span, format!("'{}' is not defined", name))
                    .with_code(DiagnosticCode::NotDefined)
                    .with_hint("Define the value before using it")
            }

            Self::UncompatibleConstraint {
                data_type,
                constraint,
                span,
            } => {
                Diagnostic::error(
                    *span,
                    format!(
                        "Constraint '{}' is incompatible with type '{}'",
                        constraint, data_type
                    ),
                )
                .with_code(DiagnosticCode::UncompatibleConstraint)
                .with_hint("Check that the constraint matches the declared type")
            }

            Self::InvalidTarget(target, span) => {
                Diagnostic::error(*span, format!("Invalid assignment target '{}'", target))
                    .with_code(DiagnosticCode::InvalidTarget)
                    .with_hint("Only variables and object fields can be assigned to")
            }

            Self::FileError(message, span) => {
                Diagnostic::error(*span, format!("File error: {}", message))
                    .with_code(DiagnosticCode::FileError)
                    .with_hint("Verify that the file exists and is accessible")
            }

            Self::MiscellaneousError(message, span) => {
                Diagnostic::error(*span, message.clone())
                    .with_code(DiagnosticCode::MiscellaneousError)
            }
        }
    }
}


impl Error for EvalError {}
