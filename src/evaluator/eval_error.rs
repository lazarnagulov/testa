use crate::parser::ast::{InfixOperator, PrefixOperator};

use super::object::Object;


#[derive(Debug)]
pub enum EvalError {
    UnsupportedPrefixOperator {
        operator: PrefixOperator,
        object: Object
    },
    UnsupportedInfixOperand {
        left: Object,
        operator: InfixOperator,
        right: Object
    },
    TypeError {
        expected: String,
        got: String
    },
    NotDefined(String),
    // TODO: Better name?
    General(String)
}

impl EvalError {
    pub fn unsupported_prefix_operator<T : Into<Object>>(operator: PrefixOperator, object: T) -> Self {
        EvalError::UnsupportedPrefixOperator { operator, object: object.into() }
    }
    
    pub fn unsupported_infix_operator<T : Into<Object>>(left: T, operator: InfixOperator, right: T) -> Self {
        EvalError::UnsupportedInfixOperand { left: left.into(), operator, right: right.into() }
    }
    
    pub fn type_error(expected: String, got: String) -> Self {
        EvalError::TypeError { expected, got }
    }
}