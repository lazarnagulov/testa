use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Hover, HoverParams};

use crate::lsp::backend::Backend;

pub(crate) async fn handle_hover(backend: &Backend, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let _document = backend.get_document(&uri).await?;

    let lsp_position = params.text_document_position_params.position;
    let _line = lsp_position.line + 1;
    let _column = lsp_position.character + 1;

    todo!()
}
