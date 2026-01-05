use indexmap::{IndexMap, IndexSet};

use testa_core::{
    analyser::symbol_table::symbol::SymbolKind,
    ast::{Field, Program, Statement},
    utils::Span,
};

use crate::{
    evaluator::{
        Evaluator,
        context::{Context, State},
        error::EvalError,
        expression::evaluate_expression,
    },
    object::Object,
};

pub mod record_iterator;

pub type Record = IndexMap<String, Object>;

#[derive(Debug, Default)]
pub struct GenerateInfo {
    template_name: Option<String>,
    body: Vec<Field>,
    total_count: usize,
    span: Span,
}

pub struct RecordGenerator<'a> {
    pub evaluator: &'a mut Evaluator,
    generate_infos: Vec<GenerateInfo>,
    current_statement: usize,
    current_count: usize,
}

impl<'a> RecordGenerator<'a> {
    pub fn new(evaluator: &'a mut Evaluator) -> Self {
        Self {
            evaluator,
            generate_infos: Vec::new(),
            current_count: 0,
            current_statement: 0,
        }
    }

    pub fn stream(self) -> impl Iterator<Item = Result<Record, EvalError>> {
        self
    }

    pub fn collect_all(self) -> Result<Vec<Record>, EvalError> {
        self.collect()
    }

    pub fn collect_limit(self, limit: usize) -> Result<Vec<Record>, EvalError> {
        self.take(limit).collect()
    }

    pub fn get_field_names(&self) -> Result<Vec<String>, EvalError> {
        let mut names = IndexSet::new();

        for info in &self.generate_infos {
            match &info.template_name {
                Some(template_name) => {
                    self.collect_template_field_names(template_name, info.span, &mut names)?;
                }
                None => {
                    for field in &info.body {
                        names.insert(field.name.clone());
                    }
                }
            }
        }

        Ok(names.into_iter().collect())
    }

    fn collect_template_field_names(
        &self,
        template_name: &str,
        span: Span,
        out: &mut IndexSet<String>,
    ) -> Result<(), EvalError> {
        let symbol = self
            .evaluator
            .context
            .symbol_table
            .lookup(template_name)
            .ok_or_else(|| EvalError::NotDefined(template_name.to_string(), span))?;

        let (parent, fields) = match &symbol.kind {
            SymbolKind::Template { parent, fields, .. } => (parent, fields),
            kind => {
                return Err(EvalError::TypeMismatch {
                    expected: "template".to_string(),
                    got: kind.to_string(),
                    span: symbol.span,
                });
            }
        };

        if let Some(parent_name) = parent {
            self.collect_template_field_names(parent_name, span, out)?;
        }

        for field_name in fields {
            out.insert(field_name.clone());
        }

        Ok(())
    }

    pub fn generate_infos(&mut self, program: &Program) -> Result<(), EvalError> {
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

                self.generate_infos.push(GenerateInfo {
                    template_name: template_name.clone(),
                    body: body.clone(),
                    total_count,
                    span: *span,
                });
            }
        }

        Ok(())
    }

    fn generate_from_template(
        ctx: &Context,
        state: &mut State,
        name: &str,
        span: Span,
    ) -> Result<Record, EvalError> {
        let symbol = ctx
            .symbol_table
            .lookup(name)
            .ok_or_else(|| EvalError::NotDefined(name.to_string(), span))?;

        let (parent, field_names) = match &symbol.kind {
            SymbolKind::Template { parent, fields, .. } => (parent, fields),
            kind => {
                return Err(EvalError::TypeMismatch {
                    expected: "template".to_string(),
                    got: kind.to_string(),
                    span: symbol.span,
                });
            }
        };

        let mut record = IndexMap::new();

        if let Some(parent_name) = parent {
            let parent_record = Self::generate_from_template(ctx, state, parent_name, span)?;
            record.extend(parent_record);
        }

        let template_scope = ctx
            .find_template_scope(name)
            .ok_or_else(|| EvalError::MiscellaneousError("Invalid scope".into(), span))?;

        for field_name in field_names {
            let field_symbol = template_scope
                .symbols
                .get(field_name)
                .ok_or_else(|| EvalError::NotDefined(field_name.to_string(), span))?;

            let expression = match &field_symbol.kind {
                SymbolKind::Field { expression, .. } => expression,
                _ => return Err(EvalError::NotDefined(field_name.to_string(), span)),
            };

            let value = evaluate_expression(ctx, state, expression)?;
            record.insert(field_name.clone(), value);
        }

        Ok(record)
    }

    fn generate_anonymous(
        ctx: &Context,
        state: &mut State,
        body: &[Field],
    ) -> Result<Record, EvalError> {
        let mut record = IndexMap::new();

        for field in body {
            let value = evaluate_expression(ctx, state, &field.value)?;
            record.insert(field.name.clone(), value);
        }

        Ok(record)
    }
}
