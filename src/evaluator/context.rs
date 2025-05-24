use std::collections::HashMap;

use crate::parser::ast::Field;

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

#[derive(Debug, Default)]
pub struct EvaluatedVariant {
    pub name: String,
    pub weight: isize,
}

impl EvaluatedVariant {
    pub fn new(name: &str, weight: isize) -> Self {
        EvaluatedVariant {
            name: name.to_owned(),
            weight,
        }
    }
}

#[derive(Debug, Default)]
pub struct Enum {
    pub variants: Vec<EvaluatedVariant>,
    pub cummulative_weights: Vec<(String, isize)>,
    pub total_weight: isize,
}

impl Enum {
    pub fn new(
        variants: Vec<EvaluatedVariant>,
        cummulative_weights: Vec<(String, isize)>,
        total_weight: isize,
    ) -> Self {
        Enum {
            variants,
            cummulative_weights,
            total_weight,
        }
    }

    pub fn insert_variant(&mut self, variant: EvaluatedVariant) {
        self.variants.push(variant)
    }

    pub fn variant(&self, index: usize) -> Option<&EvaluatedVariant> {
        self.variants.get(index)
    }

    pub fn weights(&self) -> impl Iterator<Item = isize> {
        self.variants.iter().map(|variant| variant.weight)
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
