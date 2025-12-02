use crate::core::ast::{Attribute, Expression};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub value: Expression,
    pub overridable: bool,
    pub attributes: Vec<Attribute>,
}

impl Field {
    pub fn new(
        name: String,
        value: Expression,
        overridable: bool,
        attributes: Vec<Attribute>,
    ) -> Self {
        Field {
            name,
            value,
            overridable,
            attributes,
        }
    }
}
