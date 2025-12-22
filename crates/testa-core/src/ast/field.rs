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
    pub name_span: Option<Span>, 
}

impl Field {
    pub fn new(
        name: &str,
        value: Expression,
        overridable: bool,
        attributes: Vec<Attribute>,
        span: Span,
    ) -> Self {
        Field {
            name: name.to_string(),
            value,
            overridable,
            attributes,
            span,
            name_span: None, 
        }
    }
    
    pub fn with_name_span(
        name: &str,
        value: Expression,
        overridable: bool,
        attributes: Vec<Attribute>,
        span: Span,
        name_span: Span,
    ) -> Self {
        Field {
            name: name.to_string(),
            value,
            overridable,
            attributes,
            span,
            name_span: Some(name_span),
        }
    }
}