use std::path::PathBuf;

use crate::{
    ast::{Attribute, ConstraintExpression, Field, expression::Expression, variant::Variant},
    utils::Span,
};

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Expression(ExpressionStatemnt),
    Template {
        parent: Option<String>,
        attributes: Vec<Attribute>,
        name: String,
        body: Vec<Field>,
        span: Span,
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
