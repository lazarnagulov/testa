use std::fmt;

use crate::core::ast::Expression;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConstraintExpression {
    pub expression: Expression,
    pub kind: ConstraintKind,
}

impl ConstraintExpression {
    pub fn new(expression: Expression, constraint_kind: ConstraintKind) -> Self {
        ConstraintExpression {
            expression,
            kind: constraint_kind,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum ConstraintKind {
    Range,
    MultipleOf,
    Length,
    Bias,
    Custom,
    Min,
    Max,
    Count,
}

impl fmt::Display for ConstraintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintKind::Range => write!(f, "range"),
            ConstraintKind::MultipleOf => write!(f, "multiple_of"),
            ConstraintKind::Length => write!(f, "lenght"),
            ConstraintKind::Bias => write!(f, "bias"),
            ConstraintKind::Custom => write!(f, "custom"),
            ConstraintKind::Min => write!(f, "min"),
            ConstraintKind::Max => write!(f, "max"),
            ConstraintKind::Count => write!(f, "count"),
        }
    }
}
