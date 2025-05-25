use std::collections::HashMap;

use crate::{enumeration::enumeration::Enum, parser::ast::Field};

use super::eval_error::EvalError;

pub trait Visitor<T> {
    fn visit(&self, context: &Context) -> Result<T, EvalError>;
}

#[derive(Debug, Default, Clone)]
pub struct Template {
    pub fields: Vec<Field>,
}

impl Template {
    pub fn new(fields: Vec<Field>) -> Self {
        Template { fields }
    }

    pub fn insert_field(&mut self, field: Field) {
        self.fields.push(field)
    }

    pub fn field_names(&self) -> impl Iterator<Item = &str> {
        self.fields.iter().map(|field| field.name.as_str())
    }
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
