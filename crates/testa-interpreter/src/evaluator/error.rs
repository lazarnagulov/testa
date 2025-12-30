use core::fmt;
use std::error::Error;

use testa_core::{ast::{InfixOperator, PrefixOperator}, utils::Span};

use crate::object::Object;

#[derive(Debug)]
pub enum EvalError {
    UnsupportedPrefixOperator {
        operator: PrefixOperator,
        object: Object,
        span: Span
    },
    UnsupportedInfixOperand {
        left: Object,
        operator: InfixOperator,
        right: Object,
        span: Span
    },
    TypeMismatch {
        expected: String,
        got: String,
        span: Span
    },
    NotDefined(String, Span),
    UncompatibleConstraint {
        data_type: String,
        constraint: String,
        span: Span
    },
    InvalidTarget(String, Span),
    FileError(String, Span),
    // TODO: Better name?
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
            span: Span::default()
        }
    }

    pub fn incompatible_constraint(data_type: &str, constraint: &str) -> Self {
        EvalError::UncompatibleConstraint {
            constraint: constraint.to_owned(),
            data_type: data_type.to_owned(),
            span: Span::default()
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
            span: Span::default()
        }
    }

    pub fn type_mismatch(expected: String, got: String) -> Self {
        EvalError::TypeMismatch { expected, got, span: Span::default() }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UnsupportedPrefixOperator { operator, span, .. } => {
                write!(f, "Unsupported prefix operator '{}' on object at {:?}", operator, span)
            }
            EvalError::UnsupportedInfixOperand { left, operator, right, span } => {
                write!(f, "Unsupported infix operation: '{}' between {:?} and {:?} at {:?}", operator, left, right, span)
            }
            EvalError::TypeMismatch { expected, got, span } => {
                write!(f, "Type mismatch: expected '{}' but got '{}' at {:?}", expected, got, span)
            }
            EvalError::NotDefined(name, span) => {
                write!(f, "Variable '{}' not defined at {:?}", name, span)
            }
            EvalError::UncompatibleConstraint { data_type, constraint, span } => {
                write!(f, "Incompatible constraint '{}' for type '{}' at {:?}", constraint, data_type, span)
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
        }
    }
}



impl Error for EvalError {}
