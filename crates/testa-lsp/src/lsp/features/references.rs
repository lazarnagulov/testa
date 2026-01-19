use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Location, ReferenceParams, Url};

use crate::lsp::backend::Backend;
use crate::lsp::workspace::document::Analysis;

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

fn references_at(analysis: &Analysis, uri: &Url, line: u32, column: u32) -> Option<Vec<Location>> {
    let reference = analysis.find_reference_at(line, column)?;
    let all_refs = analysis.get_references(&reference.name)?;

    Some(
        all_refs
            .iter()
            .map(|r| Location {
                uri: uri.clone(),
                range: r.span.to_lsp_range(),
            })
            .collect(),
    )
}
