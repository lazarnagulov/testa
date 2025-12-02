use crate::core::utils::span::Span;

#[derive(Debug, Clone)]
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
    
    InvalidBuiltIn,
    InvalidDirective,
    InvalidNumberLiteral,
    UnexpectedEof,

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
}

impl Diagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            severity: Severity::Error,
            code: None,
            hint: None
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
}

pub type DiagnosticResult<T> = Result<T, Vec<Diagnostic>>;
