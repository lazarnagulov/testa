use std::path::PathBuf;

use crate::{
    ast::{Attribute, ConstraintExpression, Field, expression::Expression, variant::Variant},
    utils::Span,
};

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Expression(ExpressionStatemnt),
    Template {
        parent_name: Option<String>,
        parent_span: Option<Span>,
        attributes: Vec<Attribute>,
        name: String,
        body: Vec<Field>,
        span: Span,
        name_span: Option<Span>,
    },
    OutputDirective {
        argument: String,
        options: Vec<Field>,
        span: Span,
    },
    OutputPathDirective {
        argument: PathBuf,
        span: Span,
    },
    TypeDecl {
        name: String,
        data_type: Expression,
        attributes: Vec<Attribute>,
        span: Span,
        name_span: Option<Span>,
    },
    ConstraintDecl {
        name: String,
        constraint: ConstraintExpression,
        span: Span,
    },
    Enum {
        name: String,
        variants: Vec<Variant>,
        attributes: Vec<Attribute>,
        span: Span,
        name_span: Option<Span>, 
    },
    Resource {
        name: String,
        body: Vec<Field>,
        span: Span,
    },
    Generate {
        template_name: Option<String>,
        template_name_span: Span,
        body: Vec<Field>,
        count: Expression,
        span: Span,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExpressionStatemnt {
    pub expression: Expression,
    pub span: Span,
}
