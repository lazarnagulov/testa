use core::fmt;

use crate::{
    diagnostics::{
        Diagnostic,
        DiagnosticCode::{self},
    },
    utils::Span,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexerError {
    InvalidBuiltIn { span: Span, value: String },
    InvalidToken { span: Span, token: char },
    InvalidDirective { span: Span, value: String },
    InvalidNumberLiteral { span: Span, value: String },
    MissingChar { span: Span, expected: char },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBuiltIn { value, .. } => write!(f, "Invalid built-in type '{}'", value),
            Self::InvalidToken { token, .. } => write!(f, "Unexpected character '{}'", token),
            Self::InvalidDirective { value, .. } => write!(f, "Invalid directive '@{}'", value),
            Self::InvalidNumberLiteral { value, .. } => {
                write!(f, "Invalid number literal '{}'", value)
            }
            Self::MissingChar { expected, .. } => {
                write!(f, "Expected '{}' but reached end of input", expected)
            }
        }
    }
}

impl LexerError {
    pub fn to_diagnostic(&self) -> Diagnostic {
        match self {
            Self::InvalidBuiltIn { span, value } => {
                Diagnostic::error(*span, format!("Invalid built-in type '{}'", value))
                    .with_code(DiagnosticCode::InvalidBuiltIn)
                    .with_hint("Valid built-in types are: int, string, bool, float")
            }

            Self::InvalidToken { span, token } => {
                Diagnostic::error(*span, format!("Unexpected character '{}'", token))
                    .with_code(DiagnosticCode::UnexpectedCharacter)
            }

            Self::InvalidDirective { span, value } => {
                Diagnostic::error(*span, format!("Invalid directive '@{}'", value))
                    .with_code(DiagnosticCode::InvalidDirective)
                    .with_hint("Valid directives are: @output, @output_path, @generate")
            }

            Self::InvalidNumberLiteral { span, value } => {
                Diagnostic::error(*span, format!("Invalid number literal '{}'", value))
                    .with_code(DiagnosticCode::InvalidNumberLiteral)
                    .with_hint("Numbers must be valid integers or floats (e.g., 42, 3.14)")
            }

            Self::MissingChar { span, expected } => Diagnostic::error(
                *span,
                format!("Expected '{}' but reached end of input", expected),
            )
            .with_code(DiagnosticCode::UnexpectedEof)
            .with_hint(match expected {
                '"' => "Add a closing quote to complete the string literal",
                '}' => "Add a closing brace to complete the block",
                ']' => "Add a closing bracket to complete the constraint",
                ')' => "Add a closing parenthesis",
                _ => "Add the missing character",
            }),
        }
    }
}
