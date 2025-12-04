#![allow(unused)]
use std::iter::Peekable;

use crate::{
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::error::ParserError,
    utils::Span,
};

pub struct TokenStream<'src> {
    lexer: Peekable<Lexer<'src>>,
    buffer: Vec<Token>,
    last_span: Span,
}

impl<'src> TokenStream<'src> {
    pub fn new(lexer: Peekable<Lexer<'src>>) -> Self {
        Self {
            lexer,
            buffer: Vec::new(),
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
        let token = if !self.buffer.is_empty() {
            self.buffer.remove(0)
        } else {
            self.lexer
                .next()
                .ok_or(ParserError::UnexpectedEof {
                    span: self.last_span,
                })?
                .map_err(ParserError::from)?
        };

        self.last_span = token.span;
        Ok(token)
    }

    pub fn peek_token(&mut self) -> Result<&Token, ParserError> {
        self.lexer
            .peek()
            .ok_or(ParserError::UnexpectedEof {
                span: self.last_span,
            })?
            .as_ref()
            .map_err(|e| ParserError::from(e.clone()))
    }

    pub fn peek_kind(&mut self) -> &TokenKind {
        self.lexer
            .peek()
            .and_then(|r| r.as_ref().ok())
            .map_or(&TokenKind::Eof, |t| &t.kind)
    }

    pub fn peek_kind_n(&mut self, n: usize) -> TokenKind {
        self.lexer
            .clone()
            .nth(n - 1)
            .and_then(|r| r.ok())
            .map_or(TokenKind::Eof, |token| token.kind)
    }

    pub fn peek_nth(&mut self, n: usize) -> Result<&Token, ParserError> {
        while self.buffer.len() < n {
            let token = self.lexer.next().ok_or(ParserError::UnexpectedEof {
                span: self.last_span,
            })??;
            self.buffer.push(token);
        }

        self.buffer.get(n - 1).ok_or(ParserError::UnexpectedEof {
            span: self.last_span,
        })
    }
}
