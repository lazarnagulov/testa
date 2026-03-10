use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use testa_core::{
    analyser::result::AnalysisResult,
    ast::{Attribute, Field, Program, Statement},
};

use crate::{Item, ItemId, Module, ModuleMetadata, StringPool, source_map::SourceMapBuilder};

#[derive(Default)]
pub struct AstLowering {
    string_pool: StringPool,
    next_item_id: u32,
    source_map_builder: SourceMapBuilder,

    source_file: PathBuf,
    items: Vec<Item>,
}

impl AstLowering {
    pub fn new(source_file: PathBuf) -> Self {
        Self {
            string_pool: StringPool::new(),
            next_item_id: 0,
            items: Vec::new(),
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
                name: self
                    .source_file
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("module")
                    .to_string(),
                source_file: self.source_file,
                source_hash,
                compiled_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            string_pool: self.string_pool,
            items: self.items,
            source_map: self.source_map_builder.build(),
        }
    }

    fn lower_statement(&mut self, stmt: &Statement, _analysis: &AnalysisResult) {
        match stmt {
            Statement::Template {
                parent_name,
                attributes,
                name,
                body,
                ..
            } => {
                let template = self.lower_template(name, parent_name, body, attributes);
                self.items.push(Item::Template(template));
            }
            Statement::Struct { .. } => todo!(),
            Statement::TypeDecl { .. } => todo!(),
            Statement::ConstraintDecl { .. } => todo!(),
            Statement::Enum { .. } => todo!(),
            _ => {}
        }
    }

    fn next_item_id(&mut self) -> ItemId {
        let id = ItemId(self.next_item_id);
        self.next_item_id += 1;
        id
    }

    fn lower_template(
        &mut self,
        name: &str,
        parent_name: &Option<String>,
        body: &[Field],
        attributes: &[Attribute],
    ) -> crate::module::Template {
        let id = self.next_item_id();
        let name_id = self.string_pool.intern(name);
        let parent = parent_name
            .as_ref()
            .and_then(|p| self.find_item_id_by_name(p));

        let fields = body
            .iter()
            .enumerate()
            .map(|(idx, field)| self.lower_field(field, idx))
            .collect();

        let attrs = attributes
            .iter()
            .map(|attr| self.lower_attribute(attr))
            .collect();

        crate::module::Template {
            id,
            name: name_id,
            parent,
            fields,
            attributes: attrs,
        }
    }

    fn lower_field(&self, _field: &Field, _idx: usize) -> crate::module::Field {
        todo!()
    }

    fn lower_attribute(&self, _attr: &Attribute) -> crate::module::Attribute {
        todo!()
    }

    fn find_item_id_by_name(&self, name: &str) -> Option<ItemId> {
        for item in &self.items {
            let item_name = match item {
                Item::Template(t) => self.string_pool.resolve(t.name),
                //     Item::Enum(e) => self.string_pool.resolve(e.name),
                //     Item::TypeAlias(t) => self.string_pool.resolve(t.name),
            };

            if item_name == name {
                return Some(match item {
                    Item::Template(t) => t.id,
                    //     Item::Enum(e) => e.id,
                    //     Item::TypeAlias(t) => t.id,
                });
            }
        }
        None
    }
}
