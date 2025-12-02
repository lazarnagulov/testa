use crate::core::{ast::{DataType, InfixOperator, PatternElement, PrefixOperator}, utils::span::Span};

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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Element {
    pub value: Expression,
    pub weight: Option<Expression>,
    pub start: usize,
    pub size: usize,
}

impl Element {
    pub fn new(value: Expression, weight: Option<Expression>, start: usize, size: usize) -> Self {
        Element {
            value,
            weight,
            start,
            size,
        }
    }
}


