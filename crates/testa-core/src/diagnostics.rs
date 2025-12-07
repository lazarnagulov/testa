use crate::utils::Span;

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
    DuplicateDeclaration,
    InvalidParent,
    EmptyEnum,
    DuplicateVariant,
    InvalidContext,
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

        format!(
            "{}{}:{}:{}: [{}] {}{}{}",
            severity_color,
            self.span.start.line,
            self.span.start.column,
            self.span.start.offset,
            severity_label,
            self.message,
            reset_color,
            self.hint
                .as_ref()
                .map(|h| format!("\n  hint: {}", h))
                .unwrap_or_default()
        )
    }
}

pub type DiagnosticResult<T> = Result<T, Vec<Diagnostic>>;
