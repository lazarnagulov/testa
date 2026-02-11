use std::{fs, path::PathBuf};

use testa_core::{
    analyser::{SemanticAnalyser, result::AnalysisResult, symbol_table::SymbolTable},
    ast::Program,
    diagnostics::{Diagnostic, DiagnosticCode},
    lexer::Lexer,
    parser::Parser,
    utils::Span,
};

pub fn parse_file(path: &PathBuf) -> Result<Program, Vec<Diagnostic>> {
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

pub fn compile_file(path: &PathBuf) -> Result<(Program, SymbolTable), Vec<Diagnostic>> {
    let program = parse_file(path)?;
    let AnalysisResult {
        diagnostics,
        symbol_table,
        ..
    } = SemanticAnalyser::new(&program).analyse();

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    Ok((program, symbol_table))
}
