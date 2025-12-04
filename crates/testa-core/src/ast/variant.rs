use crate::{ast::Expression, utils::Span};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub weight: Option<Expression>,
    pub span: Span,
}

impl Variant {
    pub fn new(name: String, weight: Option<Expression>, span: Span) -> Self {
        Variant { name, weight, span }
    }
}
