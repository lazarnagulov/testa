use std::path::PathBuf;

use crate::{
    StringPool,
    module::{Module, ModuleMetadata, node::ModuleId},
    source_map::SourceMap,
};

impl Module {
    pub fn empty(name: &str) -> Self {
        Self {
            metadata: ModuleMetadata {
                id: ModuleId(0),
                version: 1,
                name: name.to_string(),
                source_file: PathBuf::from(format!("{}.testa", name)),
                source_hash: 0,
                compiled_at: 0,
            },
            string_pool: StringPool::new(),
            imports: Vec::new(),
            items: Vec::new(),
            directives: Vec::new(),
            source_map: SourceMap::new(Vec::new(), Vec::new(), Vec::new()),
        }
    }
}
