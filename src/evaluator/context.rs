use std::collections::HashMap;

use crate::parser::ast::Field;

#[derive(Debug, Default, Clone)]
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

    pub fn get_field_names(&self) -> Vec<&str> {
        self.fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<Vec<&str>>()
    }

}

#[derive(Debug, Default)]
pub struct Enum {
    pub variants: Vec<String>
}

impl Enum {

    pub fn new(variants: Vec<String>) -> Self {
        Enum { variants }
    }

    pub fn insert_field(&mut self, variant: &str) {
        self.variants.push(variant.to_owned())
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