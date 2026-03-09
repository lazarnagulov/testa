use serde::{Deserialize, Serialize};
use testa_core::utils::Span;

use crate::{FieldId, ItemId, StringId};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    pub item_spans: Vec<(ItemId, Span)>,
    pub field_spans: Vec<(FieldId, Span)>,
    pub identifier_spans: Vec<(StringId, Span)>,
}

impl SourceMap {

    pub fn get_item_span(&self, id: ItemId) -> Option<Span> {
        self.item_spans
            .iter()
            .find(|(item_id, _)| *item_id == id)
            .map(|(_, span)| *span)
    }
    
    pub fn get_field_span(&self, id: FieldId) -> Option<Span> {
        self.field_spans
            .iter()
            .find(|(field_id, _)| *field_id == id)
            .map(|(_, span)| *span)
    }
    
    pub fn get_identifier_span(&self, id: StringId) -> Option<Span> {
        self.identifier_spans
            .iter()
            .find(|(string_id, _)| *string_id == id)
            .map(|(_, span)| *span)
    }
}
