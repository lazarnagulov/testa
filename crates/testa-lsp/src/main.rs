mod semantic_token;
mod semantic_token_builder;
mod document;
mod language_server;

use std::collections::HashMap;
use std::sync::Arc;

use testa_core::parser::Parser;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LspService, Server};

use crate::document::Document;


#[derive(Debug)]
struct Backend {
    client: Client,
    documents: Arc<RwLock<HashMap<String, Document>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn update_document(&self, uri: Url, text: String, _version: i32) {
        self.client
            .log_message(MessageType::INFO, format!("Parsing document: {}", uri))
            .await;
        
        // self.documents.insert(uri.clone(), document);
        // self.publish_diagnostics(&uri, diagnostics).await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
