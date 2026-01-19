use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use crate::lsp::backend::Backend;
use crate::lsp::features::util::definition_at;

pub(crate) async fn handle_goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;
    let document = backend.get_document(&uri).await?;

    let lsp_position = params.text_document_position_params.position;
    let line = lsp_position.line + 1;
    let column = lsp_position.character + 1;

    let analysis = document.get_analysis()?;

    let definition_span = match definition_at(analysis, line, column) {
        Some(span) => span,
        None => return Ok(None),
    };

    Ok(Some(GotoDefinitionResponse::Scalar(Location {
        uri,
        range: definition_span.to_lsp_range(),
    })))
}
