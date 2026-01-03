use std::collections::HashMap;

use testa_core::{
    ast::{Field, Program, Statement},
    utils::Span,
};

use crate::{
    evaluator::{Evaluator, error::EvalError},
    object::Object,
};

pub mod record_iterator;

#[derive(Debug, Default)]
pub struct GenerateInfo {
    template_name: Option<String>,
    body: Vec<Field>,
    total_count: usize,
    span: Span,
}

#[derive(Debug)]
pub struct RecordGenerator<'a> {
    pub evaluator: &'a Evaluator,
    generate_infos: Vec<GenerateInfo>,
    current_statement: usize,
    current_count: usize,
}

impl<'a> RecordGenerator<'a> {
    pub fn new(evaluator: &'a Evaluator) -> Self {
        Self {
            evaluator,
            generate_infos: Vec::new(),
            current_count: 0,
            current_statement: 0,
        }
    }

    pub fn extract_generate_infos(
        &mut self,
        program: &Program,
    ) -> Result<Vec<GenerateInfo>, EvalError> {
        let mut infos = Vec::new();
        for stmt in &program.0 {
            if let Statement::Generate {
                template_name,
                body,
                count,
                span,
                ..
            } = stmt
            {
                let count_value = self.evaluator.evaluate_expression(count)?;

                let total_count = match count_value {
                    Object::Int(n) if n > 0 => n as usize,
                    Object::Int(n) => {
                        return Err(EvalError::InvalidCount {
                            value: n,
                            span: count.span,
                        });
                    }
                    other => {
                        return Err(EvalError::TypeMismatch {
                            expected: "positive integer".to_string(),
                            got: format!("{}", other),
                            span: count.span,
                        });
                    }
                };

                infos.push(GenerateInfo {
                    template_name: template_name.clone(),
                    body: body.clone(),
                    total_count,
                    span: *span,
                });
            }
        }

        Ok(infos)
    }

    fn generate_from_template(
        &self,
        _name: &str,
        _span: Span,
    ) -> Result<HashMap<String, Object>, EvalError> {
        todo!()
    }

    fn generate_anonymous(&self, _body: &[Field]) -> Result<HashMap<String, Object>, EvalError> {
        todo!()
    }
}
