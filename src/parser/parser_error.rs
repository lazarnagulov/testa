#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParserError {
    Expected { expected: String, got: String },
    InvalidDirective,
    UnexpectedEOF,
    Syntax(String)
}

impl ParserError {
     pub fn expected(expected: &str, got: &str) -> Self {
        Self::Expected {
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }

    pub fn syntax_err(s: &str) -> Self {
        Self::Syntax(format!("Syntax error: {s}"))
    }
}