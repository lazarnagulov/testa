use std::{collections::VecDeque, iter::Peekable};

use crate::{
    lexer::{
        error::LexerError,
        token::{Token, TokenKind},
    },
    parser::error::ParserError,
    utils::Span,
};

pub struct TokenStream<I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    lexer: Peekable<I>,
    buffer: VecDeque<Token>,
    last_span: Span,
}

impl<I> TokenStream<I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    pub fn new(lexer: Peekable<I>) -> Self {
        Self {
            lexer,
            buffer: VecDeque::new(),
            last_span: Span::default(),
        }
    }

    pub fn from_iterator(iter: I) -> Self {
        Self {
            lexer: iter.peekable(),
            buffer: VecDeque::new(),
            last_span: Span::default(),
        }
    }

    pub fn expect_token(&mut self, kind: TokenKind) -> Result<Span, ParserError> {
        let token = self.next_token()?;
        if token.kind != kind {
            Err(ParserError::Syntax {
                message: format!("Expected {} but got {}", kind, token.kind),
                span: token.span,
            })
        } else {
            Ok(token.span)
        }
    }

    pub fn has_next(&mut self) -> bool {
        !self.buffer.is_empty() || self.lexer.peek().is_some()
    }

    pub fn consume_token(&mut self) -> Result<Span, ParserError> {
        let token = self.next_token()?;
        Ok(token.span)
    }

    pub fn last_span(&self) -> Span {
        self.last_span
    }

    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        let token = match self.buffer.pop_front() {
            Some(token) => Ok(token),
            None => self
                .lexer
                .next()
                .ok_or(ParserError::UnexpectedEof {
                    span: self.last_span,
                })?
                .map_err(ParserError::from),
        }?;

        self.last_span = token.span;
        Ok(token)
    }

    pub fn peek_token(&mut self) -> Result<&Token, ParserError> {
        if !self.buffer.is_empty() {
            return Ok(&self.buffer[0]);
        }

        self.lexer
            .peek()
            .ok_or(ParserError::UnexpectedEof {
                span: self.last_span,
            })?
            .as_ref()
            .map_err(|e| ParserError::from(e.clone()))
    }

    pub fn peek_kind(&mut self) -> &TokenKind {
        if !self.buffer.is_empty() {
            return &self.buffer[0].kind;
        }

        self.lexer
            .peek()
            .and_then(|r| r.as_ref().ok())
            .map_or(&TokenKind::Eof, |t| &t.kind)
    }

    pub fn peek_kind_n(&mut self, n: usize) -> TokenKind {
        while self.buffer.len() < n {
            match self.lexer.next() {
                Some(Ok(token)) => self.buffer.push_back(token),
                Some(Err(_)) => return TokenKind::Eof,
                None => return TokenKind::Eof,
            }
        }

        self.buffer
            .get(n - 1)
            .map_or(TokenKind::Eof, |token| token.kind.clone())
    }
}
