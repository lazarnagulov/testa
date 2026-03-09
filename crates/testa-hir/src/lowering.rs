use std::{hash::{DefaultHasher, Hash, Hasher}, path::PathBuf};

use testa_core::{analyser::result::AnalysisResult, ast::{Program, Statement}};

use crate::{ItemId, Module, SourceMapBuilder, StringPool, module::ModuleMetadata};

#[derive(Default)]
pub struct AstLowering {
    string_pool: StringPool,
    next_item_id: u32,
    source_map_builder: SourceMapBuilder,
    
    source_file: PathBuf,
    // items: Vec<Item>,
}

impl AstLowering {
    pub fn new(source_file: PathBuf) -> Self {
        Self {
            string_pool: StringPool::new(),
            next_item_id: 0,
            // items: Vec::new(),
            source_map_builder: SourceMapBuilder::new(),
            source_file,
        }
    }

    pub fn lower(mut self, ast: &Program, analysis: &AnalysisResult, source_text: &str) -> Module {
        for stmt in &ast.0 {
            self.lower_statement(stmt, analysis);
        }

        let mut hasher = DefaultHasher::new();
        source_text.hash(&mut hasher);
        let source_hash = hasher.finish();
        
        Module {
            metadata: ModuleMetadata {
                version: 1,
                name: self.source_file
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("module")
                    .to_string(),
                source_file: self.source_file.clone(),
                source_hash,
                compiled_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            string_pool: self.string_pool,
            // items: self.items,
            source_map: self.source_map_builder.build(),
        }
    }

    fn lower_statement(&mut self, _stmt: &Statement, _analysis: &AnalysisResult) {
        self.next_item_id();
        todo!()
    }

    fn next_item_id(&mut self) -> ItemId {
        let id = ItemId(self.next_item_id);
        self.next_item_id += 1;
        id
    }

}
