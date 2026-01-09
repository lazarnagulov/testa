use std::{collections::HashMap, sync::Arc};

use testa_core::{
    analyser::{SemanticAnalyser, symbol_table::SymbolTable},
    diagnostics::Diagnostic,
    lexer::Lexer,
    parser::Parser,
};
use tokio::sync::RwLock;
use tower_lsp::{
    Client,
    lsp_types::{MessageType, Url},
};

use crate::document::Document;

struct AnalysisOutcome {
    ast: Option<testa_core::ast::Program>,
    diagnostics: Vec<Diagnostic>,
    symbol_table: Option<SymbolTable>,
}

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub documents: Arc<RwLock<HashMap<Url, Document>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) async fn insert(&self, key: Url, document: Document) {
        let mut docs = self.documents.write().await;
        docs.insert(key, document);
    }

    fn analyse_text(&self, text: &str) -> AnalysisOutcome {
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer, text);

        match parser.parse() {
            Ok(ast) => {
                let mut analyser = SemanticAnalyser::new(&ast);
                let analysis = analyser.analyse();

                let mut diagnostics = Vec::new();
                diagnostics.extend(analysis.diagnostics);

                AnalysisOutcome {
                    ast: Some(ast),
                    diagnostics,
                    symbol_table: Some(analysis.symbol_table),
                }
            }

            Err(errors) => {
                let diagnostics = errors.into_iter().map(|e| e.to_diagnostic()).collect();

                AnalysisOutcome {
                    ast: None,
                    diagnostics,
                    symbol_table: None,
                }
            }
        }
    }

    pub(super) async fn update_document(&self, uri: Url, text: String, version: i32) {
        self.client
            .log_message(MessageType::INFO, format!("Parsing document: {}", uri))
            .await;

        let result = self.analyse_text(&text);

        let document = Document::new(&text, version)
            .with_ast(result.ast)
            .with_diagnostics(&result.diagnostics)
            .with_symbol_table(result.symbol_table);

        self.publish_diagnostics(&uri, result.diagnostics).await;
        self.insert(uri, document).await;
    }

    async fn publish_diagnostics(&self, uri: &Url, diagnostics: Vec<Diagnostic>) {
        let lsp = diagnostics
            .iter()
            .map(Diagnostic::to_lsp_diagnostics)
            .collect();
        self.client
            .publish_diagnostics(uri.clone(), lsp, None)
            .await;
    }
}
