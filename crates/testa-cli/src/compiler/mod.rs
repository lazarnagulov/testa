use std::{error::Error, fs, path::PathBuf};

use testa_core::{
    analyser::{SemanticAnalyser, result::AnalysisResult, symbol_table::SymbolTable},
    ast::Program,
    diagnostics::Severity,
    lexer::Lexer,
    parser::Parser,
};

pub fn parse_file(path: &PathBuf) -> Result<Program, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer, &content);
    Ok(parser.parse()?)
}

pub fn compile_file(
    path: &PathBuf,
    show_warnings: bool,
) -> Result<(Program, SymbolTable), Box<dyn Error>> {
    let program = parse_file(path)?;
    let AnalysisResult {
        diagnostics,
        symbol_table,
    } = SemanticAnalyser::new(&program).analyse();

    let mut has_errors = false;

    for diag in &diagnostics {
        if diag.severity == Severity::Warning && !show_warnings {
            continue;
        }
        eprintln!("{}", diag.format_cli());
        has_errors = true;
    }

    if has_errors {
        Err(format!("Failed to compile {}", path.display()).into())
    } else {
        Ok((program, symbol_table))
    }
}
