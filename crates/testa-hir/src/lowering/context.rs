use std::collections::HashMap;

use crate::module::node::{GlobalItemId, StringId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Template,
    Struct,
    Enum,
    TypeAlias,
}

#[derive(Debug, Clone)]
struct RegisteredItem {
    kind: ItemKind,
    id: GlobalItemId,
}

#[derive(Default)]
pub struct LoweringContext {
    item_lookup: HashMap<StringId, RegisteredItem>,
}

impl LoweringContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, kind: ItemKind, name: StringId, id: GlobalItemId) {
        self.item_lookup.insert(name, RegisteredItem { kind, id });
    }

    pub fn resolve(&self, kind: ItemKind, name: StringId) -> Option<GlobalItemId> {
        let item = self.item_lookup.get(&name)?;
        (item.kind == kind).then_some(item.id)
    }
}
