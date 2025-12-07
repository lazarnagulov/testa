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
use crate::lexer::error::LexerError;
use crate::lexer::token::Token;
use crate::parser::error::ParserError;
use crate::parser::token_stream::TokenStream;

pub struct Parser<'src, I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    token_stream: TokenStream<I>,
    source: &'src str,

    attributes: Vec<Attribute>,
    path: &'src Path,
}

impl<'src, I> Parser<'src, I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    pub fn new(lexer: I, program: &'src str, path: &'src Path) -> Self
    where
        I: Iterator<Item = Result<Token, LexerError>>,
    {
        let lexer = lexer.peekable();
        Self {
            token_stream: TokenStream::new(lexer),
            source: program,
            attributes: Vec::new(),
            path,
        }
    }

    pub fn from_token_stream(token_stream: TokenStream<I>, source: &'src str) -> Self {
        Self {
            token_stream,
            source,
            attributes: Vec::new(),
            path: Path::new("")
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
