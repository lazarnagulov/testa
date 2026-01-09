use tower_lsp::lsp_types::{TextDocumentSyncCapability, TextDocumentSyncKind};
use tower_lsp::{Client, jsonrpc::Result};
use tower_lsp::{
    LanguageServer,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
        InitializedParams, MessageType, SemanticTokensParams, SemanticTokensResult,
        ServerCapabilities,
    },
};

use crate::lsp::workspace::Workspace;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub workspace: Workspace,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace: Workspace::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let diagnostics = self
            .workspace
            .open(
                params.text_document.uri.clone(),
                params.text_document.text.clone(),
                params.text_document.version,
            )
            .await;
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let diagnostics = self
            .workspace
            .change(
                params.text_document.uri.clone(),
                params.content_changes[0].text.clone(),
                params.text_document.version,
            )
            .await;
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let _uri = params.text_document.uri.to_string();

        Ok(None)
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Testa LSP server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
