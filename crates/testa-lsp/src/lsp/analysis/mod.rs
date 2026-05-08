use std::{collections::HashMap, path::Path};

use testa_core::{
    analyser::{SemanticAnalyser, symbol_table::SymbolTable},
    ast::{Program, Statement},
    lexer::Lexer,
    parser::Parser,
};
use testa_hir::{Module, module::resolver::ModuleResolver};

use crate::lsp::workspace::document::Analysis;

struct ResolvedUnit {
    pub modules: HashMap<String, Module>,
    pub tables: Vec<SymbolTable>,
}

impl ResolvedUnit {
    pub fn new(modules: HashMap<String, Module>, tables: Vec<SymbolTable>) -> Self {
        Self { modules, tables }
    }
}

pub struct AnalysisEngine;

impl AnalysisEngine {
    pub fn analyse(text: &str, file_path: Option<&Path>) -> Analysis {
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer, text);

        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(errors) => {
                return Analysis {
                    ast: None,
                    references: HashMap::new(),
                    diagnostics: errors.into_iter().map(|e| e.to_diagnostic()).collect(),
                    symbol_table: None,
                    imported_modules: HashMap::new(),
                };
            }
        };

        let unit = AnalysisEngine::resolve_imports(&ast, file_path);
        let mut analiser = SemanticAnalyser::new(&ast);
        let analysis = analiser.analyse_with_imports(&unit.tables);

        Analysis {
            ast: Some(ast),
            diagnostics: analysis.diagnostics,
            symbol_table: Some(analysis.symbol_table),
            references: analysis.references,
            imported_modules: unit.modules,
        }
    }

    fn resolve_imports(ast: &Program, file_path: Option<&Path>) -> ResolvedUnit {
        if let Some(path) = file_path {
            let import_names: Vec<String> = ast
                .0
                .iter()
                .filter_map(|stmt| {
                    if let Statement::ImportDirective { argument, .. } = stmt {
                        Some(argument.clone())
                    } else {
                        None
                    }
                })
                .collect();
            let mut resolver = ModuleResolver::with_defaults(path);
            match resolver.resolve_all(&import_names, path) {
                Ok(modules) => {
                    let tables = modules
                        .values()
                        .map(|m| m.to_symbol_table())
                        .collect::<Vec<_>>();
                    ResolvedUnit::new(modules, tables)
                }
                Err(_) => ResolvedUnit::new(HashMap::new(), Vec::new()),
            }
        } else {
            ResolvedUnit::new(HashMap::new(), Vec::new())
        }
    }
}
