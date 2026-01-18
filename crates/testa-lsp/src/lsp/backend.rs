use tower_lsp::lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, Location, OneOf, ReferenceParams,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp::{Client, jsonrpc::Result};
use tower_lsp::{
    LanguageServer,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
        InitializedParams, MessageType, ServerCapabilities,
    },
};

use crate::lsp::features::{goto_definition, references};
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
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Testa LSP server initialized!")
            .await;
    }

    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
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

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        goto_definition::handle_goto_definition(self, params).await
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        references::handle_references(self, params).await
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
