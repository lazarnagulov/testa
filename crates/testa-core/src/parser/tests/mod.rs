use crate::{lexer::{error::LexerError, token::{Token, TokenKind}}, parser::Parser, utils::{Location, Span}};

mod enumeration;
mod data_type;
mod template;
mod directive;
mod error;

pub fn loc(offset: usize, line: u32, col: u32) -> Location {
    Location::new(offset, line, col)
}

pub fn span(
    start_offset: usize,
    start_line: u32,
    start_col: u32,
    end_offset: usize,
    end_line: u32,
    end_col: u32,
) -> Span {
    Span::new(
        loc(start_offset, start_line, start_col),
        loc(end_offset, end_line, end_col),
    )
}

pub fn token(kind: TokenKind) -> Token {
    Token {
        kind,
        span: Span::default(),
    }
}

pub fn token_with_span(kind: TokenKind, span: Span) -> Token {
    Token { kind, span }
}

pub fn identifier(span: Span) -> Token {
    token_with_span(TokenKind::Identifier, span)
}

pub fn int_literal(span: Span) -> Token {
    token_with_span(TokenKind::IntLiteral, span)
}

pub fn string_literal(span: Span) -> Token {
    token_with_span(TokenKind::StringLiteral, span)
}

pub fn parser_from_tokens(
    tokens: Vec<Result<Token, LexerError>>,
    source: &'static str,
) -> Parser<'static, std::vec::IntoIter<Result<Token, LexerError>>> {
    Parser::new(tokens.into_iter(), source)
}
