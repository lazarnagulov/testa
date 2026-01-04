use std::{collections::HashMap, path::PathBuf};

use testa_core::ast::Expression;

use crate::{
    evaluator::{
        context::{Context, OutputFormat, State},
        error::EvalError,
        expression::evaluate_expression,
    },
    object::Object,
};

pub mod context;
pub mod error;
pub mod expression;

mod directive;
mod data_type;
mod enumeration;
mod tests;

pub struct EvaluationResult {
    pub output_format: OutputFormat,
    pub output_config: HashMap<String, Object>,
    pub output_path: Option<PathBuf>,
}

pub struct Evaluator {
    pub context: Context,
    pub state: State,
}

impl Evaluator {
    pub fn new(context: Context, seed: Option<u64>) -> Self {
        Self {
            context,
            state: State::new(seed),
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> Result<Object, EvalError> {
        evaluate_expression(&self.context, &mut self.state, expression)
    }

    pub fn output_config(&self) -> (&OutputFormat, &HashMap<String, Object>, &Option<PathBuf>) {
        (
            &self.context.output_format,
            &self.context.output_options,
            &self.context.output_path,
        )
    }
}
