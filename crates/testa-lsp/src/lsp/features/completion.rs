use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{CompletionParams, CompletionResponse};

use crate::lsp::backend::Backend;

pub(crate) async fn handle_completion(
    _backend: &Backend,
    _params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    todo!()
}
