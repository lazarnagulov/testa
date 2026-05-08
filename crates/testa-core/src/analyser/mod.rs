pub mod error;
pub mod reference_checker;
pub mod reference_tracker;
pub mod symbol_table;
pub mod type_checker;

use std::collections::{HashMap, HashSet};

use crate::{
    analyser::{
        error::SemanticError,
        reference_checker::ReferenceChecker,
        reference_tracker::ReferenceTracker,
        result::AnalysisResult,
        symbol_table::{SymbolTable, symbol_table_builder::SymbolTableBuilder},
        type_checker::TypeChecker,
    },
    ast::{Program, Statement},
    utils::Span,
};

pub mod result;

#[derive(Debug)]
pub struct SemanticAnalyser<'a> {
    errors: Vec<SemanticError>,
    program: &'a Program,
}

impl<'a> SemanticAnalyser<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            errors: Vec::new(),
        }
    }

    pub fn analyse(&mut self) -> AnalysisResult {
        self.analyse_with_imports(&[])
    }

    pub fn analyse_with_imports(&mut self, imported: &[&SymbolTable]) -> AnalysisResult {
        let symbol_table = match SymbolTableBuilder::new().build(self.program) {
            Ok(table) => table,
            Err(errors) => {
                self.errors.extend(errors);
                return AnalysisResult {
                    symbol_table: SymbolTable::default(),
                    references: HashMap::new(),
                    type_map: HashMap::new(),
                    diagnostics: self
                        .errors
                        .iter()
                        .map(SemanticError::to_diagnostic)
                        .collect(),
                };
            }
        };

        self.check_all_inheritance_cycles(&symbol_table);

        if let Err(errs) =
            ReferenceChecker::with_imports(&symbol_table, imported).check(self.program)
        {
            self.errors.extend(errs);
        }

        let (references, ref_errors) =
            ReferenceTracker::with_imports(&symbol_table, imported).track_references(self.program);

        self.errors.extend(ref_errors);

        let (type_map, type_errs) = TypeChecker::new(&symbol_table).check(self.program);

        self.errors.extend(type_errs);

        AnalysisResult {
            symbol_table,
            references,
            type_map,
            diagnostics: self
                .errors
                .iter()
                .map(SemanticError::to_diagnostic)
                .collect(),
        }
    }

    fn check_all_inheritance_cycles(&mut self, symbol_table: &SymbolTable) {
        let mut templates: Vec<(String, Span)> = Vec::new();

        for stmt in &self.program.0 {
            if let Statement::Template { name, span, .. } = stmt {
                templates.push((name.clone(), *span));
            }
        }

        let mut checked = HashSet::new();

        for (template_name, span) in templates {
            if checked.contains(&template_name) {
                continue;
            }

            match symbol_table.check_inheritance_cycle(&template_name) {
                Ok(chain) => {
                    for t in chain {
                        checked.insert(t);
                    }
                }
                Err(cycle) => {
                    let cycle_str = cycle.join(" -> ");

                    self.errors.push(SemanticError::InheritanceCycle {
                        template_chain: cycle_str.to_string(),
                        span,
                    });

                    for t in cycle {
                        checked.insert(t);
                    }
                }
            }
        }
    }
}
