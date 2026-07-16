use std::collections::HashMap;

use crate::{ItemId, StringId, module::ItemRef};

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
    item_ref: ItemRef,
}

#[derive(Default)]
pub struct LoweringContext {
    item_lookup: HashMap<StringId, RegisteredItem>,
}

impl LoweringContext {
    pub fn new() -> Self {
        Self {
            item_lookup: HashMap::new(),
        }
    }

    pub fn register_local(&mut self, kind: ItemKind, name: StringId, id: ItemId) {
        self.register(kind, name, ItemRef::Local(id));
    }

    pub fn register_import(
        &mut self,
        kind: ItemKind,
        module: StringId,
        name: StringId,
        id: ItemId,
    ) {
        self.register(kind, name, ItemRef::Imported { module, item: id });
    }

    pub fn resolve(&self, kind: ItemKind, name: StringId) -> Option<&ItemRef> {
        let item = self.item_lookup.get(&name)?;
        (item.kind == kind).then_some(&item.item_ref)
    }

    fn register(&mut self, kind: ItemKind, name: StringId, item_ref: ItemRef) {
        self.item_lookup
            .insert(name, RegisteredItem { kind, item_ref });
    }
}
