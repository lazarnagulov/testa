use crate::ast::statement::Statement;

#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>);
