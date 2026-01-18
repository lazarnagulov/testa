use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};

use crate::lsp::backend::Backend;

pub(crate) async fn handle_goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params
        .text_document_position_params
        .text_document
        .uri;

    let document = backend.workspace.get(&uri).await
        .ok_or_else(Error::invalid_request)?;

    let _position = params
        .text_document_position_params
        .position;

    let _analysis = document
        .analysis
        .as_ref()
        .ok_or_else(Error::invalid_request)?;

    todo!()
}
