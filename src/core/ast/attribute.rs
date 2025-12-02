use crate::core::utils::span::Span;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Attribute {
    Flag(String, Span),
    KeyValue(String, String, Span),
}
