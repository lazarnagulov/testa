use std::collections::HashMap;

use testa_core::ast::{Field, Program, Statement};

use crate::{
    evaluator::{Evaluator, context::OutputFormat, error::EvalError},
    object::Object,
};

impl Evaluator {
    pub(super) fn evaluate_directive(&mut self, program: &Program) -> Result<(), EvalError> {
        for statement in &program.0 {
            match statement {
                Statement::OutputDirective {
                    argument,
                    options,
                    span,
                } => {
                    match argument.as_str() {
                        "csv" => self.context.output_format = OutputFormat::Csv,
                        _ => return Err(EvalError::InvalidTarget(argument.clone(), *span)),
                    }
                    self.context.output_options = self.evaluate_output_options(options)?;
                }
                Statement::OutputPathDirective { argument, .. } => {
                    self.context.output_path = Some(argument.clone());
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn evaluate_output_options(
        &mut self,
        fields: &[Field],
    ) -> Result<HashMap<String, Object>, EvalError> {
        let mut result = HashMap::new();
        for field in fields {
            let name = field.name.clone();
            result.insert(name, self.evaluate_expression(&field.value)?);
        }

        Ok(result)
    }
}
