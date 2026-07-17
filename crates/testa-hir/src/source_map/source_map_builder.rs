use testa_core::utils::Span;

use crate::{
    module::node::{FieldId, LocalItemId, StringId},
    source_map::SourceMap,
};

#[derive(Default, Debug)]
pub struct SourceMapBuilder {
    item_spans: Vec<(LocalItemId, Span)>,
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

    pub fn add_item(&mut self, id: LocalItemId, span: Span) {
        self.item_spans.push((id, span));
    }

    pub fn add_field(&mut self, id: FieldId, span: Span) {
        self.field_spans.push((id, span));
    }

    pub fn add_identifier(&mut self, id: StringId, span: Span) {
        self.identifier_spans.push((id, span));
    }

    pub fn build(self) -> SourceMap {
        SourceMap::new(self.item_spans, self.field_spans, self.identifier_spans)
    }
}
