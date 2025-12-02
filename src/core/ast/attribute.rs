
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Attribute {
    Flag(String),
    KeyValue(String, String),
}
