use std::fmt;

use lsp_types::{DiagnosticSeverity, NumberOrString};

use crate::utils::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub span: Span,
    pub severity: Severity,
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    UnexpectedToken,
    UnexpectedCharacter,
    ExpectedToken,
    IOError,

    InvalidBuiltIn,
    InvalidDirective,
    InvalidNumberLiteral,
    UnexpectedEof,
    InvalidBinaryOperator,
    InvalidUnaryOperator,
    InvalidConstraintForType,
    InvalidConstraintValue,
    InvalidWeightType,
    UnsupportedPrefixOperator,
    UnsupportedInfixOperand,
    DivisionByZero,
    NotDefined,
    UncompatibleConstraint,
    InvalidTarget,
    FileError,
    MiscellaneousError,

    InvalidStringPattern,
    UndefinedType,
    SyntaxError,
    UndefinedConstraint,
    UndefinedTemplate,
    DuplicateDefinition,
    TypeMismatch,
    InvalidAttribute,

    UnusedType,
    UnusedTemplate,
    DuplicateDeclaration,
    InvalidParent,
    EmptyEnum,
    DuplicateVariant,
    InvalidContext,

    UnknownType,
    UnknownTemplate,
    UnknownEnum,
    UnknownEnumVariant,
    UnknownField,
    UnknownIdentifier,
    UnknownParentTemplate,
    InheritanceCycle,
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            DiagnosticCode::UnexpectedToken => "unexpected token",
            DiagnosticCode::UnexpectedCharacter => "unexpected character",
            DiagnosticCode::ExpectedToken => "expected token",
            DiagnosticCode::InvalidBuiltIn => "invalid built-in",
            DiagnosticCode::InvalidDirective => "invalid directive",
            DiagnosticCode::InvalidNumberLiteral => "invalid number literal",
            DiagnosticCode::UnexpectedEof => "unexpected end of file",
            DiagnosticCode::UnsupportedPrefixOperator => "unsupported prefix operator",
            DiagnosticCode::UnsupportedInfixOperand => "unsupported infix operand",
            DiagnosticCode::DivisionByZero => "division by zero",
            DiagnosticCode::NotDefined => "not defined",
            DiagnosticCode::UncompatibleConstraint => "incompatible constraint",
            DiagnosticCode::InvalidTarget => "invalid target",
            DiagnosticCode::FileError => "file error",
            DiagnosticCode::MiscellaneousError => "miscellaneous error",
            DiagnosticCode::InvalidStringPattern => "invalid string pattern",
            DiagnosticCode::UndefinedType => "undefined type",
            DiagnosticCode::SyntaxError => "syntax error",
            DiagnosticCode::UndefinedConstraint => "undefined constraint",
            DiagnosticCode::UndefinedTemplate => "undefined template",
            DiagnosticCode::DuplicateDefinition => "duplicate definition",
            DiagnosticCode::TypeMismatch => "type mismatch",
            DiagnosticCode::InvalidAttribute => "invalid attribute",
            DiagnosticCode::UnusedType => "unused type",
            DiagnosticCode::UnusedTemplate => "unused template",
            DiagnosticCode::DuplicateDeclaration => "duplicate declaration",
            DiagnosticCode::InvalidParent => "invalid parent",
            DiagnosticCode::EmptyEnum => "empty enum",
            DiagnosticCode::DuplicateVariant => "duplicate enum variant",
            DiagnosticCode::InvalidContext => "invalid context",
            DiagnosticCode::UnknownType => "unknown type",
            DiagnosticCode::UnknownTemplate => "unknown template",
            DiagnosticCode::UnknownEnum => "unknown enum",
            DiagnosticCode::UnknownEnumVariant => "unknown enum variant",
            DiagnosticCode::UnknownField => "unknown field",
            DiagnosticCode::UnknownIdentifier => "unknown identifier",
            DiagnosticCode::UnknownParentTemplate => "unknown parent template",
            DiagnosticCode::InheritanceCycle => "inheritance cycle detected",
            DiagnosticCode::IOError => "failed to read file",
            DiagnosticCode::InvalidBinaryOperator => "invalid binary operator",
            DiagnosticCode::InvalidUnaryOperator => "invalid unary operator",
            DiagnosticCode::InvalidConstraintForType => "invalid constraint for type",
            DiagnosticCode::InvalidConstraintValue => "invalid constraint value",
            DiagnosticCode::InvalidWeightType => "invalid weight type",
        };

        write!(f, "{message}")
    }
}

impl Diagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            severity: Severity::Error,
            code: None,
            hint: None,
        }
    }

    pub fn info(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            severity: Severity::Info,
            code: None,
            hint: None,
        }
    }

    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn format_cli(&self) -> String {
        let severity_label = match self.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARNING",
            Severity::Info => "INFO",
            Severity::Hint => "HINT",
        };

        let severity_color = match self.severity {
            Severity::Error => "\x1b[31m",
            Severity::Warning => "\x1b[33m",
            Severity::Info => "\x1b[34m",
            Severity::Hint => "\x1b[32m",
        };

        let reset_color = "\x1b[0m";

        let span_part = if self.span != Span::default() {
            format!(
                "{}:{}:{}: ",
                self.span.start.line, self.span.start.column, self.span.start.offset
            )
        } else {
            String::new()
        };

        let hint_part = self
            .hint
            .as_ref()
            .map(|h| format!("\n  hint: {}", h))
            .unwrap_or_default();

        format!(
            "{}{}[{}] {}{}{}",
            severity_color, span_part, severity_label, self.message, reset_color, hint_part
        )
    }

    pub fn to_lsp_diagnostics(&self) -> lsp_types::Diagnostic {
        lsp_types::Diagnostic {
            range: self.span.to_lsp_range(),
            severity: Some(match self.severity {
                Severity::Error => DiagnosticSeverity::ERROR,
                Severity::Warning => DiagnosticSeverity::WARNING,
                Severity::Info => DiagnosticSeverity::INFORMATION,
                Severity::Hint => DiagnosticSeverity::HINT,
            }),
            message: self.message.clone(),
            code: self.code.map(|c| NumberOrString::String(c.to_string())),
            ..Default::default()
        }
    }
}

pub type DiagnosticResult<T> = Result<T, Vec<Diagnostic>>;
