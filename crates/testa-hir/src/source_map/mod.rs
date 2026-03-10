pub mod source_map_builder;
pub mod span_map;

pub use source_map_builder::SourceMapBuilder;
pub use span_map::SpanMap;

use serde::{Deserialize, Serialize};
use testa_core::utils::Span;

use crate::{FieldId, ItemId, StringId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    pub item_spans: SpanMap<ItemId, Span>,
    pub field_spans: SpanMap<FieldId, Span>,
    pub identifier_spans: SpanMap<StringId, Span>,
}

impl SourceMap {
    pub fn new(
        item_spans: Vec<(ItemId, Span)>,
        field_spans: Vec<(FieldId, Span)>,
        identifier_spans: Vec<(StringId, Span)>,
    ) -> Self {
        Self {
            item_spans: SpanMap::new(item_spans),
            field_spans: SpanMap::new(field_spans),
            identifier_spans: SpanMap::new(identifier_spans),
        }
    }

    pub fn get_item_span(&mut self, id: ItemId) -> Option<Span> {
        self.item_spans.get(&id)
    }

    pub fn get_field_span(&mut self, id: FieldId) -> Option<Span> {
        self.field_spans.get(&id)
    }

    pub fn get_identifier_span(&mut self, id: StringId) -> Option<Span> {
        self.identifier_spans.get(&id)
    }
}
