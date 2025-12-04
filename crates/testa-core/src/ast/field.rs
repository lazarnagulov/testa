use crate::{
    ast::{Attribute, Expression},
    utils::Span,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub value: Expression,
    pub overridable: bool,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

impl Field {
    pub fn new(
        name: String,
        value: Expression,
        overridable: bool,
        attributes: Vec<Attribute>,
        span: Span,
    ) -> Self {
        Field {
            name,
            value,
            overridable,
            attributes,
            span,
        }
    }
}
