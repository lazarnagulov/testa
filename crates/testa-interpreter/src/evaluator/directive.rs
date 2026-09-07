use std::path::PathBuf;

use testa_hir::module::node::Directive;

use crate::evaluator::{Evaluator, error::EvalError, expression::evaluate_expression};

impl Evaluator {
    pub fn evaluate_directives(&mut self) -> Result<(), EvalError> {
        for directive in &self.context.module.directives.clone() {
            match directive {
                Directive::Output { format, options } => {
                    let format_str = self.context.resolve_local_string(*format);
                    self.context.output_format = format_str.parse().map_err(|_| {
                        EvalError::InvalidTarget(format_str.to_string(), Default::default())
                    })?;
                    for (key, val_expr) in options {
                        let key = self.context.resolve_local_string(*key).to_string();
                        let val = evaluate_expression(&self.context, &mut self.state, val_expr)?;
                        self.context.output_options.insert(key, val);
                    }
                }
                Directive::OutputPath(path_id) => {
                    let path = self.context.resolve_local_string(*path_id);
                    self.context.output_path = Some(PathBuf::from(path));
                }
                _ => {}
            }
        }
        Ok(())
    }
}
