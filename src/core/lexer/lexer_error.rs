use core::fmt;

use crate::core::utils::span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexerError {
    InvalidBuiltIn(Span, String),
    InvalidToken(Span, char),
    InvalidDirective(Span, String),
    InvalidNumberLiteral(Span),
    MissingChar(Span, char),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidBuiltIn(span, built_in) => write!(
                f,
                "{}:{} ERROR: Invalid built-in '{}'.",
                span.line, span.line_offset, built_in
            ),
            LexerError::InvalidToken(span, token) => write!(
                f,
                "{}:{} ERROR: Invalid token '{}'.",
                span.line, span.line_offset, token
            ),
            LexerError::InvalidDirective(span, directive) => write!(
                f,
                "{}:{} ERROR: Invalid directive '{}'.",
                span.line, span.line_offset, directive
            ),
            LexerError::InvalidNumberLiteral(span) => write!(
                f,
                "{}:{} ERROR: Invalid number literal.",
                span.line, span.line_offset
            ),
            LexerError::MissingChar(span, char) => write!(
                f,
                "{}:{} ERROR: Missing char '{}'.",
                span.line, span.line_offset, char
            ),
        }
    }
}
