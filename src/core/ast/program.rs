use crate::core::{ast::statement::Statement, utils::span::Span};

#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>, pub Span);
