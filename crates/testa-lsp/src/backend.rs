use std::{collections::HashMap, sync::Arc};

use testa_core::{
    analyser::SemanticAnalyser,
    diagnostics::{Diagnostic, Severity},
    lexer::Lexer,
    parser::Parser,
};
use tokio::sync::RwLock;
use tower_lsp::{
    Client,
    lsp_types::{self, DiagnosticSeverity, MessageType, NumberOrString, Url},
};

use crate::document::Document;

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

    pub(super) async fn update_document(&self, uri: Url, text: String, version: i32) {
        self.client
            .log_message(MessageType::INFO, format!("Parsing document: {}", uri))
            .await;

        let lexer = Lexer::new(&text);
        let mut parser = Parser::new(lexer, &text);
        let parse_result = parser.parse();
        let (ast, mut diagnostics) = match parse_result {
            Ok(ast) => {
                self.client
                    .log_message(MessageType::INFO, format!("Parsed {} successfully", uri))
                    .await;
                (Some(ast), Vec::new())
            }
            Err(parse_errors) => {
                self.client
                    .log_message(
                        MessageType::WARNING,
                        format!("Parse errors in {}: {} errors", uri, parse_errors.len()),
                    )
                    .await;

                let diags = parse_errors
                    .into_iter()
                    .map(|e| e.to_diagnostic())
                    .collect::<Vec<_>>();

                (None, diags)
            }
        };

        let symbol_table = if let Some(ref ast) = ast {
            let mut analyzer = SemanticAnalyser::new(ast);
            let analysis = analyzer.analyse();

            diagnostics.extend(analysis.diagnostics);
            Some(analysis.symbol_table)
        } else {
            None
        };

        let document = Document::new(&text, version)
            .with_ast(ast)
            .with_diagnostics(&diagnostics)
            .with_symbol_table(symbol_table);

        if document.diagnostics == diagnostics {
            self.publish_diagnostics(uri.as_ref(), diagnostics).await;
        }
        self.insert(uri.clone(), document).await;
    }

    async fn publish_diagnostics(&self, uri: &str, diagnostics: Vec<Diagnostic>) {
        let lsp_diagnostics = diagnostics
            .into_iter()
            .map(|d| lsp_types::Diagnostic {
                range: d.span.to_lsp_range(),
                severity: Some(match d.severity {
                    Severity::Error => DiagnosticSeverity::ERROR,
                    Severity::Warning => DiagnosticSeverity::WARNING,
                    Severity::Info => DiagnosticSeverity::INFORMATION,
                    Severity::Hint => DiagnosticSeverity::HINT,
                }),
                message: d.message,
                code: d.code.map(|code| NumberOrString::String(code.to_string())),
                ..Default::default()
            })
            .collect();

        if let Ok(uri) = uri.parse() {
            self.client
                .publish_diagnostics(uri, lsp_diagnostics, None)
                .await;
        }
    }
}
