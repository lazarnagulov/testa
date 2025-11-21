use crate::core::{lexer::lexer_error::LexerError, utils::span::Span};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParserError {
    Expected {
        span: Span,
        expected: String,
        got: String,
    },
    UndefinedConstraint(Span),
    InvalidDirective(Span),
    InvalidAttribute(Span, String),
    UnexpectedEOF,
    LexerError(LexerError),
    InvalidStringPattern(Span, String),
    Syntax(Span, String),
}

impl ParserError {
    pub fn invalid_attribute(token: &str, span: Span) -> Self {
        Self::InvalidAttribute(span, token.to_owned())
    }

    pub fn expected(expected: &str, got: &str, span: Span) -> Self {
        Self::Expected {
            expected: expected.to_owned(),
            got: got.to_owned(),
            span,
        }
    }

    pub fn syntax_err(s: &str, span: Span) -> Self {
        Self::Syntax(span, format!("Syntax error: {s}"))
    }
}

impl From<LexerError> for ParserError {
    fn from(err: LexerError) -> Self {
        ParserError::LexerError(err) 
    }
}