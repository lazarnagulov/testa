use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use crate::lsp::backend::Backend;

pub(crate) async fn handle_goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;

    let document = backend
        .workspace
        .get(&uri)
        .await
        .ok_or_else(Error::invalid_request)?;

    let lsp_position = params.text_document_position_params.position;
    let line = lsp_position.line + 1;
    let column = lsp_position.character + 1;

    let analysis = document
        .analysis
        .as_ref()
        .ok_or_else(Error::invalid_request)?;

    let Some(reference) = analysis.find_reference_at(line, column) else {
        return Ok(None);
    };
    let Some(symbol_table) = analysis.symbol_table.as_ref() else {
        return Ok(None);
    };

    let Some(definition_span) = symbol_table.get_definition_span(&reference.name) else {
        return Ok(None);
    };

    Ok(Some(GotoDefinitionResponse::Scalar(Location {
        uri,
        range: definition_span.to_lsp_range(),
    })))
}
