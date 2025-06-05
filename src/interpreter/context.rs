use std::{
    collections::HashMap,
    path::PathBuf,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    core::{
        constraints::constrainted_type::ConstrainedType,
        model::{enumeration::Enum, template::Template},
    },
    generation::generator::Target,
};

use super::{eval_error::EvalError, object::Object};

pub trait Visitor<T>: std::fmt::Debug {
    fn visit(&self, context: &Context) -> Result<T, EvalError>;
}

// TODO: Consider changing String to &str
#[derive(Debug, Default)]
pub struct Context {
    pub output_path: Option<PathBuf>,
    pub target_format: Target,
    pub target_config: HashMap<String, Object>,

    templates: HashMap<String, Rc<Template>>,
    enums: HashMap<String, Enum>,
    types: HashMap<String, Rc<ConstrainedType>>,
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

    pub fn output_path(&self) -> PathBuf {
        match &self.output_path {
            Some(path) => path.clone(),
            None => PathBuf::from(format!(
                "testa_{}.{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_nanos(),
                self.target_format
            )),
        }
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
