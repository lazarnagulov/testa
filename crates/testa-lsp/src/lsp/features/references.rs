use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Location, ReferenceParams};

use crate::lsp::backend::Backend;
use crate::lsp::features::util::references_at;

pub(crate) async fn handle_references(
    backend: &Backend,
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;
    let document = backend.get_document(&uri).await?;

    let lsp_position = params.text_document_position.position;
    let line = lsp_position.line + 1;
    let column = lsp_position.character + 1;

    let analysis = document.get_analysis()?;

    Ok(references_at(analysis, &uri, line, column))
}
