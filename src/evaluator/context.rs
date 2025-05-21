use std::collections::HashMap;

use crate::parser::ast::Field;

#[derive(Debug)]
pub struct Template {
    pub fields: Vec<Field>
}

impl Template {

    pub fn new(fields: Vec<Field>) -> Self {
        Template { fields }
    }

    pub fn insert_field(&mut self, field: Field) {
        self.fields.push(field)
    }

}

// TODO: Consider changing String to &str
#[derive(Debug, Default)]
pub struct Context {
    pub templates: HashMap<String, Template>
}

impl Context {
    
    pub fn insert_template(&mut self, key: &str, template: Template) -> Option<Template> {
        self.templates.insert(key.to_owned(), template)
    }

    pub fn get_template(&self, key: &str) -> Option<&Template> {
        self.templates.get(key)
    }

}