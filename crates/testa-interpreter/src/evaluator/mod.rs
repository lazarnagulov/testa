use std::{collections::HashMap, path::PathBuf};

use testa_hir::module::node::Expr;

use crate::{
    evaluator::{
        context::{Context, OutputFormat, State},
        error::EvalError,
    },
    object::Object,
};

pub mod context;
pub mod error;
pub mod expression;

mod constrained_type;
mod directive;
mod enumeration;
mod identifier;
mod pattern;
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

    pub fn evaluate_expression(&mut self, expr: &Expr) -> Result<Object, EvalError> {
        expression::evaluate_expression(&self.context, &mut self.state, expr)
    }

    pub fn output_config(&self) -> (&OutputFormat, &HashMap<String, Object>, &Option<PathBuf>) {
        (
            &self.context.output_format,
            &self.context.output_options,
            &self.context.output_path,
        )
    }
}
