use crate::core::ast::nodes::{InfixOperator, PrefixOperator};

use super::object::Object;

#[derive(Debug)]
pub enum EvalError {
    UnsupportedPrefixOperator {
        operator: PrefixOperator,
        object: Object,
    },
    UnsupportedInfixOperand {
        left: Object,
        operator: InfixOperator,
        right: Object,
    },
    TypeMismatch {
        expected: String,
        got: String,
    },
    NotDefined(String),
    UncompatibleConstraint {
        data_type: String,
        constraint: String,
    },
    InvalidTarget(String),
    FileError(String),
    // TODO: Better name?
    MiscellaneousError(String),
}

impl EvalError {
    pub fn unsupported_prefix_operator<T: Into<Object>>(
        operator: PrefixOperator,
        object: T,
    ) -> Self {
        EvalError::UnsupportedPrefixOperator {
            operator,
            object: object.into(),
        }
    }

    pub fn incompatible_constraint(data_type: &str, constraint: &str) -> Self {
        EvalError::UncompatibleConstraint {
            constraint: constraint.to_owned(),
            data_type: data_type.to_owned(),
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
        }
    }

    pub fn type_mismatch(expected: String, got: String) -> Self {
        EvalError::TypeMismatch { expected, got }
    }
}
