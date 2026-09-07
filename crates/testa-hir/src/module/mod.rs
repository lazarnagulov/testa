pub mod error;
pub mod node;
pub mod resolver;
pub mod serialize;

mod patch;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    StringPool,
    module::node::{Directive, Enum, Item, LocalItemId, ModuleId, StringId, Template, TypeAlias},
    source_map::SourceMap,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub metadata: ModuleMetadata,
    pub string_pool: StringPool,
    pub imports: Vec<StringId>,
    pub directives: Vec<Directive>,
    pub items: Vec<Item>,
    pub source_map: SourceMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    #[serde(skip)]
    pub id: ModuleId,
    pub version: u32,
    pub name: String,
    pub source_file: PathBuf,
    pub source_hash: u64,
    pub compiled_at: u64,
}

impl Module {
    pub fn get_item(&self, id: LocalItemId) -> Option<&Item> {
        self.items.iter().find(|item| match item {
            Item::Template(t) => t.id == id,
            Item::Enum(e) => e.id == id,
            Item::TypeAlias(t) => t.id == id,
            Item::Struct(s) => s.id == id,
        })
    }

    pub fn get_item_by_name(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|item| {
            let item_name = match item {
                Item::Template(t) => self.string_pool.resolve(t.name),
                Item::Enum(e) => self.string_pool.resolve(e.name),
                Item::TypeAlias(t) => self.string_pool.resolve(t.name),
                Item::Struct(s) => self.string_pool.resolve(s.name),
            };
            item_name == name
        })
    }

    pub fn templates(&self) -> impl Iterator<Item = &Template> {
        self.items.iter().filter_map(|item| {
            if let Item::Template(t) = item {
                Some(t)
            } else {
                None
            }
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &Enum> {
        self.items.iter().filter_map(|item| {
            if let Item::Enum(e) = item {
                Some(e)
            } else {
                None
            }
        })
    }

    pub fn type_aliases(&self) -> impl Iterator<Item = &TypeAlias> {
        self.items.iter().filter_map(|item| {
            if let Item::TypeAlias(t) = item {
                Some(t)
            } else {
                None
            }
        })
    }

    pub fn is_stale(&self) -> Result<bool, std::io::Error> {
        use std::collections::hash_map::DefaultHasher;
        use std::fs;
        use std::hash::{Hash, Hasher};

        let current_source = fs::read_to_string(&self.metadata.source_file)?;

        let mut hasher = DefaultHasher::new();
        current_source.hash(&mut hasher);
        let current_hash = hasher.finish();

        Ok(current_hash != self.metadata.source_hash)
    }
}
