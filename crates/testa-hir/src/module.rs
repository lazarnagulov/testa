use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{SourceMap, StringPool};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringId(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub u32);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct FieldId(pub u32);


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub metadata: ModuleMetadata,
    pub string_pool: StringPool,
    // pub items: Vec<Item>,
    pub source_map: SourceMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub version: u32,
    pub name: String,
    pub source_file: PathBuf,
    pub source_hash: u64,
    pub compiled_at: u64,
}