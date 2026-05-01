pub mod document;

use std::{collections::HashMap, sync::Arc};

use testa_core::diagnostics::Diagnostic;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{self, Url};

use crate::lsp::{
    analysis::AnalysisEngine,
    workspace::document::{Analysis, Document},
};

#[derive(Debug, Default)]
pub struct Workspace {
    documents: Arc<RwLock<HashMap<Url, Document>>>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: Url, document: Document) {
        let mut docs = self.documents.write().await;
        docs.insert(key, document);
    }

    pub async fn open(&self, uri: Url, text: String, version: i32) -> Vec<lsp_types::Diagnostic> {
        self.update(uri, text, version).await
    }

    pub async fn change(&self, uri: Url, text: String, version: i32) -> Vec<lsp_types::Diagnostic> {
        self.update(uri, text, version).await
    }

    pub async fn get(&self, uri: &Url) -> Option<Document> {
        self.documents.read().await.get(uri).cloned()
    }

    pub async fn update(&self, uri: Url, text: String, version: i32) -> Vec<lsp_types::Diagnostic> {
        let result = AnalysisEngine::analyse(&text);

        let document = match (result.ast, result.symbol_table) {
            (Some(ast), Some(symbols)) => {
                Document::new(uri.clone(), text, version).with_analysis(Analysis {
                    ast: Some(ast),
                    references: result.references,
                    symbol_table: Some(symbols),
                    diagnostics: result.diagnostics.clone(),
                })
            }
            _ => Document::new(uri.clone(), text, version),
        };

        self.insert(uri.clone(), document).await;

        result
            .diagnostics
            .into_iter()
            .map(Diagnostic::to_lsp_diagnostics)
            .collect()
    }
}
