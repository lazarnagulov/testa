// crates/testa-core/src/ast/attribute.rs
use crate::utils::Span;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Attribute {
    Flag(String, Span),
    KeyValue(String, String, Span),
}

impl Attribute {
    pub fn span(&self) -> Span {
        match self {
            Attribute::Flag(_, span) => *span,
            Attribute::KeyValue(_, _, span) => *span,
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            Attribute::Flag(name, _) => name,
            Attribute::KeyValue(name, _, _) => name,
        }
    }
    
    pub fn name_span(&self) -> Span {
        // For now, estimate: #[name] or #[name=value]
        // The name starts after "#[" (2 chars) and goes until ']' or '='
        // This is a temporary hack until we store actual name_span
        self.span()
    }
}