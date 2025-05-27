use std::{collections::HashMap, rc::Rc};

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
    pub templates: HashMap<String, Rc<Template>>,
    pub enums: HashMap<String, Enum>,
    pub types: HashMap<String, Rc<ConstrainedType>>,
}

impl Context {
    pub fn insert_template(&mut self, key: &str, template: Template) -> Option<Rc<Template>> {
        self.templates.insert(key.to_owned(), Rc::from(template))
    }

    pub fn get_template(&self, key: &str) -> Option<&Rc<Template>> {
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
    ) -> Option<Rc<ConstrainedType>> {
        self.types.insert(key.to_owned(), Rc::from(data_type))
    }

    pub fn get_type(&self, key: &str) -> Option<&Rc<ConstrainedType>> {
        self.types.get(key)
    }

    pub fn get_type_mut(&mut self, key: &str) -> Option<&mut Rc<ConstrainedType>> {
        self.types.get_mut(key)
    }
}
