use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Url};

use crate::lsp::backend::Backend;
use crate::lsp::workspace::document::Analysis;

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
    Ok(find_definition(analysis, &uri, line, column).map(GotoDefinitionResponse::Scalar))
}

fn find_definition(
    analysis: &Analysis,
    current_uri: &Url,
    line: u32,
    column: u32,
) -> Option<Location> {
    let reference = analysis.find_reference_at(line, column)?;
    let name = &reference.name;

    if let Some(table) = &analysis.symbol_table
        && let Some(span) = table.get_definition_span(name)
    {
        return Some(Location {
            uri: current_uri.clone(),
            range: span.to_lsp_range(),
        });
    }

    for module in analysis.imported_modules.values() {
        let table = analysis.imported_tables.get(&module.metadata.name)?;
        if let Some(span) = table.get_definition_span(name)
            && let Ok(uri) = Url::from_file_path(&module.metadata.source_file)
        {
            return Some(Location {
                uri,
                range: span.to_lsp_range(),
            });
        }
    }

    None
}
