use std::fmt;

use crate::ast::{DataType, DataTypeKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Str,
    Boolean,
    Range,
    List(Box<Type>),
    Custom(String),
    Unknown,
}

impl Type {
    pub fn display(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Str => "string".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::Range => "range".to_string(),
            Type::List(inner) => format!("[{}]", inner.display()),
            Type::Custom(name) => name.clone(),
            Type::Unknown => "unknown".to_string(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Type::Unknown)
    }

    pub fn is_compatible_with(&self, other: &Type) -> bool {
        if self.is_unknown() || other.is_unknown() {
            return true;
        }

        match (self, other) {
            (Type::List(e), Type::List(a)) => e.is_compatible_with(a),
            (Type::Custom(e), Type::Custom(a)) => e == a,
            _ => self == other,
        }
    }

    pub fn from_data_type(data_type: &DataType) -> Self {
        match &data_type.kind {
            DataTypeKind::Int => Type::Int,
            DataTypeKind::Float => Type::Float,
            DataTypeKind::Str => Type::Str,
            DataTypeKind::Boolean => Type::Boolean,
            DataTypeKind::List(inner) => Type::List(Box::new(Type::from_data_type(inner))),
            DataTypeKind::Custom(name) => Type::Custom(name.clone()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl From<&DataType> for Type {
    fn from(value: &DataType) -> Self {
        Type::from_data_type(value)
    }
}
