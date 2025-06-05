use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    core::ast::nodes::Field,
    evaluator::{
        self,
        context::{Context, Visitor},
        eval_error::EvalError,
    },
    generation::generator::Record,
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

impl Visitor<Record> for Template {
    fn visit(&self, context: &Context) -> Result<Record, EvalError> {
        let mut field_map: HashMap<&str, &Field> = HashMap::new();
        let mut override_set: HashSet<&str> = HashSet::new();
        override_set.extend(
            self.fields
                .iter()
                .filter(|field| field.overridable)
                .map(|field| field.name.as_str()),
        );

        let result = self
            .all_fields()
            .iter()
            .try_fold(Vec::new(), |mut acc, field| {
                if let Some(existing_field) = field_map.get(field.name.as_str()) {
                    override_set.remove(field.name.as_str());
                    if !existing_field.overridable {
                        eprintln!(
                            "WARNING: Field '{}' is overridden but not marked as 'override'!",
                            existing_field.name
                        );
                    }
                    return Ok(acc);
                }

                field_map.insert(&field.name, field);

                match evaluator::evaluate_expression(&field.value, context) {
                    Ok(result) => {
                        acc.push(result);
                        Ok(acc)
                    }
                    Err(e) => Err(e),
                }
            })?;
        override_set.iter().for_each(|field_name| eprintln!("WARNING: The field '{}' is marked as 'override', but it is not actually overridden.", field_name));
        Ok(Record::new(result))
    }
}
