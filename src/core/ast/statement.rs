use std::path::PathBuf;

use crate::core::ast::{Attribute, ConstraintExpression, Field, expression::Expression, variant::Variant};


#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Expression(ExpressionStatemnt),
    Template {
        parent: Option<String>,
        attributes: Vec<Attribute>,
        name: String,
        body: Vec<Field>,
    },
    OutputDirective {
        argument: String,
        options: Vec<Field>,
    },
    OutputPathDirective {
        argument: PathBuf,
    },
    TypeDecl {
        name: String,
        data_type: Expression,
        attributes: Vec<Attribute>,
    },
    ConstraintDecl {
        name: String,
        constraint: ConstraintExpression,
    },
    Enum {
        name: String,
        variants: Vec<Variant>,
        attributes: Vec<Attribute>,
    },
    Resource {
        name: String,
        body: Vec<Field>,
    },
    Generate {
        template_name: Option<String>,
        body: Vec<Field>,
        count: Expression,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExpressionStatemnt {
    pub expression: Expression,
}