use std::collections::HashMap;

use crate::{
    constraints::constrainted_type::ConstrainedType, enumeration::enumeration::Enum,
    template::template::Template,
};

use super::eval_error::EvalError;

pub trait Visitor<T>: std::fmt::Debug {
    fn visit(&self, context: &Context) -> Result<T, EvalError>;
}

// TODO: Consider changing String to &str
#[derive(Debug, Default)]
pub struct Context {
    pub templates: HashMap<String, Template>,
    pub enums: HashMap<String, Enum>,
    pub types: HashMap<String, ConstrainedType>,
}

impl Context {
    pub fn insert_template(&mut self, key: &str, template: Template) -> Option<Template> {
        self.templates.insert(key.to_owned(), template)
    }

    pub fn get_template(&self, key: &str) -> Option<&Template> {
        self.templates.get(key)
    }

    pub fn insert_enum(&mut self, key: &str, enumeration: Enum) -> Option<Enum> {
        self.enums.insert(key.to_owned(), enumeration)
    }

    pub fn get_enum(&self, key: &str) -> Option<&Enum> {
        self.enums.get(key)
    }

    pub fn insert_type(
        &mut self,
        key: &str,
        data_type: ConstrainedType,
    ) -> Option<ConstrainedType> {
        self.types.insert(key.to_owned(), data_type)
    }

    pub fn get_type(&self, key: &str) -> Option<&ConstrainedType> {
        self.types.get(key)
    }

    pub fn get_type_mut(&mut self, key: &str) -> Option<&mut ConstrainedType> {
        self.types.get_mut(key)
    }
}
