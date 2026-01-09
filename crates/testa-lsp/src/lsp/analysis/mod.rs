use testa_core::{
    analyser::{SemanticAnalyser, symbol_table::SymbolTable},
    diagnostics::Diagnostic,
    lexer::Lexer,
    parser::Parser,
};

#[derive(Debug)]
pub struct AnalysisOutcome {
    pub ast: Option<testa_core::ast::Program>,
    pub diagnostics: Vec<Diagnostic>,
    pub symbol_table: Option<SymbolTable>,
}

pub struct AnalysisEngine;

impl AnalysisEngine {
    pub fn analyse(text: &str) -> AnalysisOutcome {
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer, text);

        match parser.parse() {
            Ok(ast) => {
                let mut analyser = SemanticAnalyser::new(&ast);
                let analysis = analyser.analyse();

                AnalysisOutcome {
                    ast: Some(ast),
                    diagnostics: analysis.diagnostics,
                    symbol_table: Some(analysis.symbol_table),
                }
            }
            Err(errors) => AnalysisOutcome {
                ast: None,
                diagnostics: errors.into_iter().map(|e| e.to_diagnostic()).collect(),
                symbol_table: None,
            },
        }
    }
}
