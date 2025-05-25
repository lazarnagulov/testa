use std::collections::HashMap;

use crate::{enumeration::enumeration::Enum, template::template::Template};

use super::eval_error::EvalError;

pub trait Visitor<T> {
    fn visit(&self, context: &Context) -> Result<T, EvalError>;
}

// TODO: Consider changing String to &str
#[derive(Debug, Default)]
pub struct Context {
    pub templates: HashMap<String, Template>,
    pub enums: HashMap<String, Enum>,
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
}
