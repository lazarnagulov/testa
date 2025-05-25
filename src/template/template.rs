use crate::{evaluator::{context::{Context, Visitor}, eval_error::EvalError, evaluator::evaluate_expression}, parser::ast::Field};

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

impl Visitor<Vec<String>> for Template {
    fn visit(&self, context: &Context) -> Result<Vec<String>, EvalError> {
        self
            .fields
            .iter()
            .map(|field| {
                evaluate_expression(&field.value, context).map(|result| format!("{}", result))
            })
            .collect::<Result<Vec<String>, EvalError>>()
    }
}