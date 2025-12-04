use crate::utils::Span;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Attribute {
    Flag(String, Span),
    KeyValue(String, String, Span),
}
