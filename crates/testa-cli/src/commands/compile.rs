use std::{fs, path::PathBuf};

use testa_core::{analyser::SemanticAnalyser, diagnostics::{Diagnostic, DiagnosticCode}, lexer::Lexer, parser::Parser, utils::Span};
use testa_hir::AstLowering;


pub fn compile_module(input: PathBuf, output: Option<PathBuf>) -> Result<(), Vec<Diagnostic>> {
    let source_file = input.clone();
    
    let content = fs::read_to_string(&input).map_err(|err| {
        vec![
            Diagnostic::error(Span::default(), format!("Failed to read file: {}", err))
                .with_code(DiagnosticCode::IOError)
                .with_hint("Ensure the file exists and is readable at the given path."),
        ]
    })?;
    
    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer, &content);
    let ast = parser.parse().map_err(|errors| {
        errors.iter().map(|e| e.to_diagnostic()).collect::<Vec<Diagnostic>>()
    })?;
    
    let mut analyzer = SemanticAnalyser::new(&ast);
    let analysis = analyzer.analyse();
    
    if analysis.has_errors() {
        return Err(analysis.diagnostics);
    }
    
    let lowering = AstLowering::new(source_file.clone());
    let hir = lowering.lower(&ast, &analysis, &content);
    
    let output_path = output.unwrap_or_else(|| {
        source_file.with_extension("tmod")
    });
    
    hir.save(&output_path).map_err(|err| {
        vec![
            Diagnostic::error(Span::default(), format!("Failed to save module: {}", err))
                .with_code(DiagnosticCode::IOError)
                .with_hint(format!("Ensure you have write permissions for '{}'", output_path.display())),
        ]
    })
}