use std::{collections::HashMap, path::Path};

use testa_core::{
    analyser::{SemanticAnalyser, symbol_table::SymbolTable},
    ast::{Program, Statement},
    lexer::Lexer,
    parser::Parser,
};
use testa_hir::module::{Module, resolver::ModuleResolver};

use crate::lsp::workspace::document::Analysis;

struct ResolvedUnit {
    pub modules: HashMap<String, Module>,
    pub tables: HashMap<String, SymbolTable>,
}

impl ResolvedUnit {
    pub fn new(modules: HashMap<String, Module>, tables: HashMap<String, SymbolTable>) -> Self {
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
                    imported_tables: HashMap::new(),
                };
            }
        };

        let unit = AnalysisEngine::resolve_imports(&ast, file_path);
        let mut analyser = SemanticAnalyser::new(&ast);

        let imported = unit.tables.values().collect::<Vec<&SymbolTable>>();
        let analysis = analyser.analyse_with_imports(&imported);

        Analysis {
            ast: Some(ast),
            diagnostics: analysis.diagnostics,
            symbol_table: Some(analysis.symbol_table),
            references: analysis.references,
            imported_modules: unit.modules,
            imported_tables: unit.tables,
        }
    }

    fn resolve_imports(ast: &Program, file_path: Option<&Path>) -> ResolvedUnit {
        let Some(path) = file_path else {
            return ResolvedUnit::new(HashMap::new(), HashMap::new());
        };

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
                    .iter()
                    .map(|(name, module)| (name.clone(), resolver.to_symbol_table(module)))
                    .collect();
                ResolvedUnit::new(modules, tables)
            }
            Err(_) => ResolvedUnit::new(HashMap::new(), HashMap::new()),
        }
    }
}
