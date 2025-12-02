use std::fmt;

use crate::core::ast::constraint::ConstraintExpression;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DataType {
    pub kind: DataTypeKind,
    pub constraints: Option<Vec<ConstraintExpression>>,
}

impl DataType {
    pub fn new(kind: DataTypeKind, constraints: Option<Vec<ConstraintExpression>>) -> Self {
        DataType { kind, constraints }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataTypeKind {
    Int,
    Str,
    Float,
    Boolean,
    List(Box<DataType>),
    Custom(String),
}

impl fmt::Display for DataTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypeKind::Int => write!(f, "int"),
            DataTypeKind::Str => write!(f, "string"),
            DataTypeKind::Float => write!(f, "float"),
            DataTypeKind::Boolean => write!(f, "bool"),
            DataTypeKind::List(data_type) => write!(f, "[{}]", data_type.kind),
            DataTypeKind::Custom(name) => write!(f, "type: {}", name),
        }
    }
}
