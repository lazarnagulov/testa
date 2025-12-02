use crate::core::ast::Expression;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PatternElement {
    Literal(String),
    RepeatChar {
        ch: PatternChar,
        count: usize,
        count_expression: Option<Expression>,
    },
    RepeatGroup {
        chars: Vec<PatternChar>,
        count: Expression,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PatternChar {
    Lowercase,
    Uppercase,
    Digit,
}
