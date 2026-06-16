use indexmap::{IndexMap, IndexSet};
use testa_hir::FieldId;
use std::io::Write;
use std::{collections::HashMap, fmt, fs::File, path::PathBuf};

use testa_hir::module::{Directive, Expr, ItemRef};

use crate::{
    evaluator::{
        Evaluator,
        context::{Context, State},
        error::EvalError,
        expression::evaluate_expression,
    },
    generator::error::GeneratorError,
    object::Object,
};

pub mod error;
pub mod record_iterator;

pub type Record = IndexMap<String, Object>;
pub type FileConfig = HashMap<String, Object>;

pub trait FileGenerator: fmt::Debug {
    fn generate(&self, record: &Record) -> Result<String, GeneratorError>;
    fn extension(&self) -> &'static str;
    fn generate_header(&self, fields: &[String]) -> Option<String>;
    fn generate_footer(&self) -> Option<String>;
    fn needs_separator(&self) -> bool {
        false
    }
    fn separator(&self) -> &str {
        ""
    }
}

#[derive(Debug)]
pub struct GenerateInfo {
    template_ref: ItemRef,
    total_count: usize,
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
            self.collect_field_names(&info.template_ref, &mut names)?;
        }

        Ok(names.into_iter().collect())
    }

    fn collect_field_names(
        &self,
        item_ref: &ItemRef,
        out: &mut IndexSet<String>,
    ) -> Result<(), EvalError> {
        let ctx = &self.evaluator.context;
        let template = ctx
            .resolve_template(item_ref)
            .ok_or_else(|| EvalError::NotDefined(format!("{}", item_ref), Default::default()))?;

        if let Some(parent_ref) = &template.parent {
            self.collect_field_names(parent_ref, out)?;
        }

        let module = ctx.module_for(item_ref);
        for field in &template.fields {
            out.insert(module.string_pool.resolve(field.name).to_string());
        }

        Ok(())
    }

    pub fn generate_infos(&mut self) -> Result<(), EvalError> {
        for directive in self.evaluator.context.module.directives.clone() {
            if let Directive::Generate { template, count } = directive {
                let count_value = evaluate_expression(
                    &self.evaluator.context,
                    &mut self.evaluator.state,
                    &count,
                )?;

                let total_count = match count_value {
                    Object::Int(n) if n > 0 => n as usize,
                    Object::Int(n) => {
                        return Err(EvalError::InvalidCount {
                            value: n,
                            span: Default::default(),
                        });
                    }
                    other => {
                        return Err(EvalError::TypeMismatch {
                            expected: "positive integer".to_string(),
                            got: format!("{}", other),
                            span: Default::default(),
                        });
                    }
                };

                self.generate_infos.push(GenerateInfo {
                    template_ref: template,
                    total_count,
                });
            }
        }

        Ok(())
    }

    pub fn write_records(
        self,
        file_generator: Box<dyn FileGenerator>,
        path: PathBuf,
    ) -> Result<(), GeneratorError> {
        let mut file = File::create(&path)
            .map_err(|err| GeneratorError::SerializationError(err.to_string()))?;

        let field_names = self
            .get_field_names()
            .map_err(|err| GeneratorError::EvaluationError(err.to_string()))?;

        if let Some(header) = file_generator.generate_header(&field_names) {
            writeln!(file, "{}", header)
                .map_err(|err| GeneratorError::SerializationError(err.to_string()))?;
        }

        let total = self.len();
        let mut count = 0;
        let mut first = true;

        for result in self {
            let record =
                result.map_err(|err| GeneratorError::SerializationError(err.to_string()))?;

            count += 1;
            if count % 10000 == 0 {
                println!("Generated {}/{} records...", count, total);
            }

            if !first && file_generator.needs_separator() {
                write!(file, "{}", file_generator.separator())
                    .map_err(|err| GeneratorError::SerializationError(err.to_string()))?;
            }
            first = false;

            let line = file_generator.generate(&record)?;
            write!(file, "{}", line)
                .map_err(|err| GeneratorError::SerializationError(err.to_string()))?;
        }

        if let Some(footer) = file_generator.generate_footer() {
            writeln!(file, "{}", footer)
                .map_err(|err| GeneratorError::SerializationError(err.to_string()))?;
        }

        println!("Generated {} records to {:?}", count, path);
        Ok(())
    }

    pub(crate) fn generate_record(
        ctx: &Context,
        state: &mut State,
        item_ref: &ItemRef,
    ) -> Result<Record, EvalError> {
        let template = ctx
            .resolve_template(item_ref)
            .ok_or_else(|| EvalError::NotDefined(format!("{}", item_ref), Default::default()))?;

        let mut record = IndexMap::new();

        if let Some(parent_ref) = template.parent.clone() {
            let parent_record = Self::generate_record(ctx, state, &parent_ref)?;
            record.extend(parent_record);
        }

        let module = ctx.module_for(item_ref);
        let item_id = item_ref.item_id();

        let fields: Vec<(String, FieldId, Expr)> = template
            .fields
            .iter()
            .map(|field| {
                let name = module.string_pool.resolve(field.name).to_string();
                (name, field.id, field.value.clone())
            })
            .collect();

        for (name, field_id, value_expr) in fields {
            let value = evaluate_expression(ctx, state, &value_expr)?;
            state.pool.push(item_id, field_id, value.clone());
            record.insert(name, value);
        }

        Ok(record)
    }
}
