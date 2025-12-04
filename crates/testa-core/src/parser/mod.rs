pub mod error;

mod constraint;
mod data_type;
mod expression;
mod helpers;
mod pattern;
mod statement;
mod token_stream;

#[cfg(test)]
mod tests;

use std::path::Path;

use crate::ast::{Attribute, Program};
use crate::lexer::Lexer;
use crate::parser::error::ParserError;
use crate::parser::token_stream::TokenStream;

pub struct Parser<'src> {
    token_stream: TokenStream<'src>,
    source: &'src str,

    attributes: Vec<Attribute>,
    path: &'src Path,
}

impl<'src> Parser<'src> {
    pub fn new(program: &'src str, path: &'src Path) -> Self {
        let lexer = Lexer::new(program).peekable();
        Self {
            token_stream: TokenStream::new(lexer),
            source: program,
            attributes: Vec::new(),
            path,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut statements = vec![];
        while self.token_stream.has_next() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        Ok(Program(statements))
    }
}
