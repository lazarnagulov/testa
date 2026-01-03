use std::collections::HashMap;

use testa_core::{
    analyser::symbol_table::symbol::{Scope, ScopeKind, SymbolKind},
    ast::{Expression, Field, Statement},
    utils::Span,
};

use crate::{
    evaluator::{Evaluator, Record, error::EvalError},
    object::Object,
};

impl Evaluator {
    pub(super) fn evaluate_generate(
        &self,
        template_name: Option<&Statement>,
        body: &[Field],
        count: &Expression,
    ) -> Result<Vec<Record>, EvalError> {
        let count_value = self.evaluate_expression(count)?;
        let num = match count_value {
            Object::Int(n) if n > 0 => n as usize,
            Object::Int(n) => {
                return Err(EvalError::MiscellaneousError(
                    format!("Count must be positive, got {}", n),
                    count.span,
                ));
            }
            _ => {
                return Err(EvalError::TypeMismatch {
                    expected: "int".to_string(),
                    got: format!("{}", count_value),
                    span: count.span,
                });
            }
        };

        let mut records = Vec::new();

        Ok(records)
    }

    fn generate_from_template(
        &self,
        template_name: &str,
        template_name_span: Span,
    ) -> Result<Record, EvalError> {
        let symbol = self
            .context
            .symbol_table
            .lookup(template_name)
            .ok_or_else(|| EvalError::NotDefined(template_name.to_string(), template_name_span))?;

        let (parent, field_names) = match &symbol.kind {
            SymbolKind::Template { parent, fields, .. } => (parent, fields),
            _ => {
                return Err(EvalError::TypeMismatch {
                    expected: "template".to_string(),
                    got: "other".to_string(),
                    span: template_name_span,
                });
            }
        };

        let mut record = HashMap::new();
        if let Some(parent_name) = parent {
            let parent_record = self.generate_from_template(parent_name, Span::default())?;
            record.extend(parent_record);
        }

        let template_scope = self.find_template_scope(template_name)?;

        for field_name in field_names {
            let field_symbol = template_scope.symbols.get(field_name).ok_or_else(|| {
                EvalError::NotDefined(
                    format!("Field '{}' in template '{}'", field_name, template_name,),
                    Span::default(),
                )
            })?;

            let expression = match &field_symbol.kind {
                SymbolKind::Field { expression, .. } => expression,
                _ => {
                    return Err(EvalError::NotDefined(
                        field_name.to_string(),
                        Span::default(),
                    ));
                }
            };

            let value = self.evaluate_expression(expression)?;
            record.insert(field_name.clone(), value);
        }

        Ok(record)
    }

    fn find_template_scope(&self, template_name: &str) -> Result<&Scope, EvalError> {
        self.context.symbol_table.scopes()
        .iter()
        .find(|scope| {
            matches!(&scope.kind, ScopeKind::Template { name } if name == template_name)
        })
        .ok_or_else(|| EvalError::NotDefined(
            format!("Template scope '{}' not found", template_name),
            Span::default()
        ))
    }

    fn generate_anonymous(&self, body: &[Field]) -> Result<Record, EvalError> {
        let mut record = HashMap::new();

        for field in body {
            let value = self.evaluate_expression(&field.value)?;
            record.insert(field.name.clone(), value);
        }

        Ok(record)
    }
}
