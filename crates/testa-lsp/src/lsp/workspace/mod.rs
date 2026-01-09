pub mod document;

use std::{collections::HashMap, sync::Arc};

use testa_core::diagnostics::Diagnostic;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{self, Url};

use crate::lsp::{analysis::AnalysisEngine, workspace::document::Document};

#[derive(Debug)]
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
        self.update_document(uri, text, version).await
    }

    pub async fn change(&self, uri: Url, text: String, version: i32) -> Vec<lsp_types::Diagnostic> {
        self.update_document(uri, text, version).await
    }

    pub async fn update_document(
        &self,
        uri: Url,
        text: String,
        version: i32,
    ) -> Vec<lsp_types::Diagnostic> {
        let result = AnalysisEngine::analyse(&text);

        let document = Document::new(&text, version)
            .with_ast(result.ast)
            .with_diagnostics(&result.diagnostics)
            .with_symbol_table(result.symbol_table);

        self.insert(uri, document).await;

        result
            .diagnostics
            .iter()
            .map(Diagnostic::to_lsp_diagnostics)
            .collect::<Vec<_>>()
    }
}
