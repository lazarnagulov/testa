use crate::{ast::Expression, utils::Span};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PatternElement {
    Literal(String, Span),
    RepeatChar {
        ch: PatternChar,
        count: usize,
        count_expression: Option<Expression>,
        span: Span,
    },
    RepeatGroup {
        chars: Vec<PatternChar>,
        count: Expression,
        span: Span,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PatternChar {
    Lowercase,
    Uppercase,
    Digit,
}
