use core::fmt;

use crate::{
    ast::{DataType, InfixOperator, PatternElement, PrefixOperator},
    utils::Span,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Expression { kind, span }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ExpressionKind {
    IntLiteral(isize),
    FloatLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    StringPattern(Vec<PatternElement>),
    Identifier(String),
    List(Vec<Element>),
    Type(DataType),
    Prefix {
        operator: PrefixOperator,
        expression: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    FuncCall {
        arguments: Vec<Expression>,
    },
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionKind::IntLiteral(n) => write!(f, "{}", n),
            ExpressionKind::FloatLiteral(value) => write!(f, "{}", value),
            ExpressionKind::StringLiteral(s) => write!(f, "\"{}\"", s),
            ExpressionKind::BooleanLiteral(b) => write!(f, "{}", b),
            ExpressionKind::Identifier(id) => write!(f, "{}", id),
            ExpressionKind::Type(data_type) => {
                let base = data_type.kind.to_string();
                if let Some(constraints) = &data_type.constraints {
                    if !constraints.is_empty() {
                        return write!(
                            f,
                            "{} [{} constraints]",
                            base,
                            constraints.len()
                        );
                    }
                }
                write!(f, "{}", base)
            }
            ExpressionKind::List(elements) => {
                if elements.is_empty() {
                    write!(f, "[]")
                } else {
                    write!(f, "[{} items]", elements.len())
                }
            }
            ExpressionKind::StringPattern(_) => write!(f, "string_pattern(...)"),
            ExpressionKind::Prefix { operator, .. } => write!(f, "{} ...", operator),
            ExpressionKind::Infix { operator, .. } => write!(f, "... {} ...", operator),
            ExpressionKind::FuncCall { .. } => write!(f, "function_call(...)"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Element {
    pub value: Expression,
    pub weight: Option<Expression>,
    pub span: Span,
}

impl Element {
    pub fn new(value: Expression, weight: Option<Expression>, span: Span) -> Self {
        Element {
            value,
            weight,
            span,
        }
    }
}
