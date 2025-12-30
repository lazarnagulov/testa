use core::fmt;
use std::error::Error;

use crate::{
    diagnostics::{Diagnostic, DiagnosticCode},
    lexer::error::LexerError,
    utils::Span,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParserError {
    Expected {
        span: Span,
        expected: String,
        got: String,
    },
    UndefinedConstraint {
        span: Span,
    },
    InvalidDirective {
        span: Span,
    },
    InvalidAttribute {
        span: Span,
        token: String,
    },
    UnexpectedEof {
        span: Span,
    },
    LexerError(LexerError),
    InvalidStringPattern {
        span: Span,
        pattern: String,
    },
    Syntax {
        span: Span,
        message: String,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected { expected, got, .. } => {
                write!(f, "Expected {} but got {}", expected, got)
            }
            Self::UndefinedConstraint { .. } => write!(f, "Undefined constraint"),
            Self::InvalidDirective { .. } => write!(f, "Invalid directive"),
            Self::InvalidAttribute { token, .. } => write!(f, "Invalid attribute '{}'", token),
            Self::UnexpectedEof { .. } => write!(f, "Unexpected end of file"),
            Self::LexerError(err) => write!(f, "{}", err),
            Self::InvalidStringPattern { pattern, .. } => {
                write!(f, "Invalid string pattern '{}'", pattern)
            }
            Self::Syntax { message, .. } => write!(f, "{}", message),
        }
    }
}

impl ParserError {
    pub fn to_diagnostic(&self) -> Diagnostic {
        match self {
            Self::Expected {
                span,
                expected,
                got,
            } => Diagnostic::error(*span, format!("Expected {} but got {}", expected, got))
                .with_code(DiagnosticCode::ExpectedToken),

            Self::UndefinedConstraint { span } => {
                Diagnostic::error(*span, "Undefined constraint".to_string())
                    .with_code(DiagnosticCode::UndefinedConstraint)
            }

            Self::InvalidDirective { span } => {
                Diagnostic::error(*span, "Invalid directive".to_string())
                    .with_code(DiagnosticCode::InvalidDirective)
            }

            Self::InvalidAttribute { span, token } => {
                Diagnostic::error(*span, format!("Invalid attribute '{}'", token))
                    .with_code(DiagnosticCode::InvalidAttribute)
            }

            Self::UnexpectedEof { span } => {
                Diagnostic::error(*span, "Unexpected end of file".to_string())
                    .with_code(DiagnosticCode::UnexpectedEof)
            }

            Self::LexerError(err) => err.to_diagnostic(),

            Self::InvalidStringPattern { span, pattern } => {
                Diagnostic::error(*span, format!("Invalid string pattern '{}'", pattern))
                    .with_code(DiagnosticCode::InvalidStringPattern)
            }

            Self::Syntax { span, message } => {
                Diagnostic::error(*span, message).with_code(DiagnosticCode::SyntaxError)
            }
        }
    }
}

impl From<LexerError> for ParserError {
    fn from(err: LexerError) -> Self {
        ParserError::LexerError(err)
    }
}

impl Error for ParserError {}
