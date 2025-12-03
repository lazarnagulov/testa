pub mod error;

mod constraint;
mod data_type;
mod expression;
mod helpers;
mod pattern;
mod statement;
mod token_stream;

use std::path::Path;

use crate::core::ast::{Attribute, Program};
use crate::core::lexer::Lexer;
use crate::core::parser::error::ParserError;
use crate::core::parser::token_stream::TokenStream;

// TODO: Add lookups for prefix and infix expressions { TokenKind: fn () }
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
