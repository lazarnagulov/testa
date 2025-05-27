use std::{collections::HashMap, rc::Rc};

use crate::{
    evaluator::{
        context::{Context, Visitor},
        eval_error::EvalError,
        evaluator::evaluate_expression,
    },
    parser::ast::Field,
};

#[derive(Debug, Default, Clone)]
pub struct Template {
    pub parent: Option<Rc<Template>>,
    pub fields: Vec<Field>,
}

impl Template {
    pub fn new(parent: Option<Rc<Template>>, fields: Vec<Field>) -> Self {
        Template { parent, fields }
    }

    pub fn insert_field(&mut self, field: Field) {
        self.fields.push(field)
    }

    pub fn all_fields(&self) -> Vec<&Field> {
        let mut all_fields = vec![];
        let mut current = Some(self);

        while let Some(current_type) = current {
            all_fields.extend(&current_type.fields);
            current = current_type.parent.as_deref();
        }

        all_fields
    }

    pub fn all_field_names(&self) -> Vec<&str> {
        let mut all_fields = vec![];
        let mut current = Some(self);

        while let Some(current_type) = current {
            all_fields.extend(current_type.field_names());
            current = current_type.parent.as_deref();
        }

        all_fields
    }

    pub fn field_names(&self) -> impl Iterator<Item = &str> {
        self.fields.iter().map(|field| field.name.as_str())
    }
}

impl Visitor<Vec<String>> for Template {
    fn visit(&self, context: &Context) -> Result<Vec<String>, EvalError> {
        let mut field_map: HashMap<&str, &Field> = HashMap::new();

        let result = self
            .all_fields()
            .iter()
            .filter_map(|field| {
                if let Some(field) = field_map.get(field.name.as_str()) {
                    if !field.overridable {
                        eprintln!(
                            "WARNING: Field {} is overridden but not marked as 'override'!",
                            field.name
                        );
                    }
                    return None;
                }
                field_map.insert(&field.name, &field);
                evaluate_expression(&field.value, context)
                    .ok()
                    .map(|result| format!("{}", result))
            })
            .collect::<Vec<String>>();
        Ok(result)
    }
}
