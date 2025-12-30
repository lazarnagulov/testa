use std::{collections::HashMap, path::PathBuf};

use testa_core::ast::Program;

use crate::{
    evaluator::{
        context::{Context, OutputFormat},
        error::EvalError,
    },
    object::Object,
};

pub mod context;
pub mod error;

mod directive;
mod expression;

pub struct EvaluationResult {
    pub records: Vec<Record>,
    pub output_format: OutputFormat,
    pub output_config: HashMap<String, Object>,
    pub output_path: Option<PathBuf>,
}

pub type Record = HashMap<String, Object>;

pub struct Evaluator {
    context: Context,
}

impl Evaluator {
    pub fn new(context: Context) -> Self {
        Self {
            context
        }
    }

    pub fn evaluate(mut self, program: Program) -> Result<EvaluationResult, EvalError> {
        self.evaluate_directive(&program)?;

        Ok(EvaluationResult {
            records: Vec::new(),
            output_format: self.context.output_format,
            output_config: self.context.output_options,
            output_path: self.context.output_path,
        })
    }
}
