use testa_core::{
    analyser::SemanticAnalyser,
    lexer::Lexer,
    parser::Parser,
};

use crate::lsp::workspace::document::Analysis;

pub struct AnalysisEngine;

impl AnalysisEngine {
    pub fn analyse(text: &str) -> Analysis {
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer, text);

        match parser.parse() {
            Ok(ast) => {
                let mut analyser = SemanticAnalyser::new(&ast);
                let analysis = analyser.analyse();

                Analysis {
                    ast: Some(ast),
                    diagnostics: analysis.diagnostics,
                    symbol_table: Some(analysis.symbol_table),
                }
            }
            Err(errors) => Analysis {
                ast: None,
                diagnostics: errors.into_iter().map(|e| e.to_diagnostic()).collect(),
                symbol_table: None,
            },
        }
    }
}
