use std::{collections::HashMap, path::PathBuf};

use crate::{
    evaluator::context::{Context, OutputFormat},
    object::Object,
};

pub mod context;
pub mod error;

mod tests;

mod directive;
mod expression;

pub struct EvaluationResult {
    pub output_format: OutputFormat,
    pub output_config: HashMap<String, Object>,
    pub output_path: Option<PathBuf>,
}

pub type Record = HashMap<String, Object>;

#[derive(Debug)]
pub struct Evaluator {
    pub context: Context,
}

impl Evaluator {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub fn output_config(&self) -> (&OutputFormat, &HashMap<String, Object>, &Option<PathBuf>) {
        (
            &self.context.output_format,
            &self.context.output_options,
            &self.context.output_path,
        )
    }
}
