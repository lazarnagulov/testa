use testa_core::utils::Span;

use crate::{FieldId, ItemId, SourceMap, StringId};

#[derive(Default)]
pub struct SourceMapBuilder {
    item_spans: Vec<(ItemId, Span)>,
    field_spans: Vec<(FieldId, Span)>,
    identifier_spans: Vec<(StringId, Span)>,
}

impl SourceMapBuilder {
    pub fn new() -> Self {
        Self {
            item_spans: Vec::new(),
            field_spans: Vec::new(),
            identifier_spans: Vec::new(),
        }
    }
    
    pub fn add_item(&mut self, id: ItemId, span: Span) {
        self.item_spans.push((id, span));
    }
    
    pub fn add_field(&mut self, id: FieldId, span: Span) {
        self.field_spans.push((id, span));
    }
    
    pub fn add_identifier(&mut self, id: StringId, span: Span) {
        self.identifier_spans.push((id, span));
    }
    
    pub fn build(self) -> SourceMap {
        SourceMap {
            item_spans: self.item_spans,
            field_spans: self.field_spans,
            identifier_spans: self.identifier_spans,
        }
    }
}
