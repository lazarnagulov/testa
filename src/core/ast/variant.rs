use crate::core::ast::Expression;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub weight: Option<Expression>,
}

impl Variant {
    pub fn new(name: String, weight: Option<Expression>) -> Self {
        Variant { name, weight }
    }
}
