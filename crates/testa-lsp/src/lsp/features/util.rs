use testa_core::utils::Span;
use tower_lsp::lsp_types::{Location, Url};

use crate::lsp::workspace::document::Analysis;

pub(super) fn references_at(
    analysis: &Analysis,
    uri: &Url,
    line: u32,
    column: u32,
) -> Option<Vec<Location>> {
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

pub(super) fn definition_at(analysis: &Analysis, line: u32, column: u32) -> Option<Span> {
    let reference = analysis.find_reference_at(line, column)?;
    let symbol_table = analysis.symbol_table.as_ref()?;
    symbol_table.get_definition_span(&reference.name)
}
