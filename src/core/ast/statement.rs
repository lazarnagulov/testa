use std::path::PathBuf;

use crate::core::{
    ast::{Attribute, ConstraintExpression, Field, expression::Expression, variant::Variant},
    utils::span::Span,
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
