use crate::{ast::Expression, utils::Span};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub weight: Option<Expression>,
    pub span: Span,
    pub name_span: Option<Span>,
}

impl Variant {
    pub fn new(name: String, weight: Option<Expression>, span: Span) -> Self {
        Variant {
            name,
            weight,
            span,
            name_span: None,
        }
    }

    pub fn with_name_span(
        name: &str,
        weight: Option<Expression>,
        span: Span,
        name_span: Span,
    ) -> Self {
        Variant {
            name: name.to_string(),
            weight,
            span,
            name_span: Some(name_span),
        }
    }
}
