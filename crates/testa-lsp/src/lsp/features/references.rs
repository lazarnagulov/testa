use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::{Location, ReferenceParams};

use crate::lsp::backend::Backend;

pub async fn handle_references(
    backend: &Backend,
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;

    let lsp_position = params.text_document_position.position;
    let line = lsp_position.line + 1;
    let column = lsp_position.character + 1;

    let document = backend
        .workspace
        .get(&uri)
        .await
        .ok_or_else(Error::invalid_request)?;

    let analysis = document
        .analysis
        .as_ref()
        .ok_or_else(Error::invalid_request)?;

    let Some(reference) = analysis.find_reference_at(line, column) else {
        return Ok(None);
    };

    let Some(all_refs) = analysis.get_references(&reference.name) else {
        return Ok(None);
    };

    let locations = all_refs
        .iter()
        .map(|r| Location {
            uri: uri.clone(),
            range: r.span.to_lsp_range(),
        })
        .collect::<Vec<Location>>();

    Ok(Some(locations))
}
