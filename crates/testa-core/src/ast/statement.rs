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
    Struct {
        name: String,
        name_span: Span,
        body: Vec<Field>,
        span: Span,
    },
    OutputDirective {
        argument: String,
        options: Vec<Field>,
        span: Span,
    },
    ImportDirective {
        argument: String,
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
        template_name_span: Option<Span>,
        body: Vec<Field>,
        count: Expression,
        span: Span,
    },
}

impl Statement {
    pub fn name(&self) -> Option<&str> {
        match self {
            Statement::Template { name, .. }
            | Statement::Struct { name, .. }
            | Statement::Enum { name, .. }
            | Statement::TypeDecl { name, .. } => Some(name),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExpressionStatemnt {
    pub expression: Expression,
    pub span: Span,
}
