use std::{collections::HashMap, fs, path::Path};

use testa_core::{
    analyser::{SemanticAnalyser, result::AnalysisResult},
    ast::{Program, Statement},
    diagnostics::{Diagnostic, DiagnosticCode},
    lexer::Lexer,
    parser::Parser,
    utils::Span,
};
use testa_hir::{Module, module::resolver::ModuleResolver};

#[derive(Debug)]
pub struct CompiledUnit {
    pub program: Program,
    pub analysis: AnalysisResult,
    pub imported: HashMap<String, Module>,
    pub source: String,
}

pub fn parse_file(path: &Path) -> Result<Program, Vec<Diagnostic>> {
    let content = fs::read_to_string(path).map_err(|err| {
        vec![
            Diagnostic::error(Span::default(), err.to_string())
                .with_code(DiagnosticCode::IOError)
                .with_hint("Ensure the file exists and is readable at the given path."),
        ]
    })?;
    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer, &content);
    match parser.parse() {
        Ok(program) => Ok(program),
        Err(errors) => Err(errors.iter().map(|e| e.to_diagnostic()).collect()),
    }
}

pub fn compile_file(path: &Path) -> Result<CompiledUnit, Vec<Diagnostic>> {
    let source = fs::read_to_string(path).map_err(|err| {
        vec![
            Diagnostic::error(Span::default(), err.to_string())
                .with_code(DiagnosticCode::IOError)
                .with_hint("Ensure the file exists and is readable at the given path."),
        ]
    })?;

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer, &source);
    let program = parser.parse().map_err(|errors| {
        errors.iter().map(|e| e.to_diagnostic()).collect::<Vec<_>>()
    })?;

    let import_names: Vec<String> = program.0.iter().filter_map(|stmt| {
        if let Statement::ImportDirective { argument, .. } = stmt {
            Some(argument.clone())
        } else {
            None
        }
    }).collect();

    let mut resolver = ModuleResolver::with_defaults(path);
    let imported = resolver.resolve_all(&import_names, path).map_err(|err| {
        vec![
            Diagnostic::error(Span::default(), format!("Failed to resolve imports: {}", err))
                .with_code(DiagnosticCode::IOError),
        ]
    })?;

    let analysis= SemanticAnalyser::new(&program).analyse();
    if !analysis.diagnostics.is_empty() {
        return Err(analysis.diagnostics);
    }

    Ok(CompiledUnit { source, program, analysis, imported })
}