use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use testa_core::{
    analyser::symbol_table::{
        SymbolTable,
        symbol::{ScopeKind, SymbolKind, VariantInfo},
    },
    ast::{DataType, DataTypeKind, Expression, ExpressionKind},
    utils::Span,
};

use crate::module::{
    Module,
    error::ResolveError,
    node::{Enum, Field, GlobalItemId, Item, ModuleId, Struct, Template, Type, TypeAlias, Variant},
    patch,
};

pub struct ModuleResolver {
    search_paths: Vec<PathBuf>,
    cache: HashMap<String, Module>,
    next_id: u32,
}

impl ModuleResolver {
    pub fn new(search_paths: Vec<PathBuf>) -> Self {
        Self {
            cache: HashMap::new(),
            search_paths,
            // 0 is reserved as the "self" sentinel produced by lowering a
            // single file in isolation; real assigned ids start at 1.
            next_id: 1,
        }
    }

    pub fn with_defaults(relative_to: &Path) -> Self {
        let mut paths = Vec::new();

        if let Ok(exe) = std::env::current_exe()
            && let Some(dir) = exe.parent()
        {
            paths.push(dir.join("std"));
        }

        if let Some(parent) = relative_to.parent() {
            paths.push(parent.to_path_buf());
        }

        Self::new(paths)
    }

    pub fn resolve_all(
        &mut self,
        names: &[String],
        relative_to: &Path,
    ) -> Result<HashMap<String, Module>, ResolveError> {
        for name in names {
            self.load_one(name, relative_to)?;
        }

        let loaded: Vec<String> = self.cache.keys().cloned().collect();
        for name in &loaded {
            self.relocate_module(name);
        }

        Ok(self
            .cache
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    pub fn to_symbol_table(&self, module: &Module) -> SymbolTable {
        let mut table = SymbolTable::new();
        for item in &module.items {
            match item {
                Item::Template(t) => self.restore_template(module, t, &mut table),
                Item::Struct(s) => self.restore_struct(module, s, &mut table),
                Item::Enum(e) => self.restore_enum(module, e, &mut table),
                Item::TypeAlias(t) => self.restore_type_alias(module, t, &mut table),
            }
        }

        table
    }

    fn load_one(&mut self, name: &str, relative_to: &Path) -> Result<(), ResolveError> {
        if self.cache.contains_key(name) {
            return Ok(());
        }

        let path = self.find_tmod(name, relative_to).ok_or_else(|| {
            ResolveError::NotFound(format!("{} not found in {:?}", name, relative_to))
        })?;

        let mut module = Module::load(&path)?;
        module.metadata.id = ModuleId(self.next_id);
        self.next_id += 1;

        let transitive = module
            .imports
            .iter()
            .map(|id| module.string_pool.resolve(*id).to_string())
            .collect::<Vec<_>>();

        self.cache.insert(name.to_string(), module);

        for dep in transitive {
            self.load_one(&dep, &path)?;
        }

        Ok(())
    }

    fn relocate_module(&mut self, name: &str) {
        let Some((self_id, import_names)) = self.cache.get(name).map(|module| {
            let import_names = module
                .imports
                .iter()
                .map(|id| module.string_pool.resolve(*id).to_string())
                .collect::<Vec<_>>();
            (module.metadata.id, import_names)
        }) else {
            return;
        };

        let import_ids: Vec<ModuleId> = import_names
            .iter()
            .map(|n| self.cache.get(n).map(|m| m.metadata.id).unwrap_or_default())
            .collect();

        if let Some(module) = self.cache.get_mut(name) {
            patch::patch_module(module, self_id, &import_ids);
        }
    }

    pub fn relocate_root(&mut self, module: &mut Module) {
        module.metadata.id = ModuleId(self.next_id);
        self.next_id += 1;

        let import_names: Vec<String> = module
            .imports
            .iter()
            .map(|id| module.string_pool.resolve(*id).to_string())
            .collect();

        let import_ids: Vec<ModuleId> = import_names
            .iter()
            .map(|n| self.cache.get(n).map(|m| m.metadata.id).unwrap_or_default())
            .collect();

        patch::patch_module(module, module.metadata.id, &import_ids);
    }

    fn find_tmod(&self, name: &str, relative_to: &Path) -> Option<PathBuf> {
        let filename = format!("{}.tmod", name);

        if let Some(parent) = relative_to.parent() {
            let candidate = parent.join(&filename);

            if candidate.exists() {
                return Some(candidate);
            }
        }

        for path in &self.search_paths {
            let candidate = path.join(&filename);
            if candidate.exists() {
                return Some(candidate);
            }
        }

        None
    }

    fn module_by_id(&self, id: ModuleId) -> Option<&Module> {
        self.cache.values().find(|m| m.metadata.id == id)
    }

    fn restore_struct(&self, module: &Module, st: &Struct, table: &mut SymbolTable) {
        let name = module.string_pool.resolve(st.name).to_string();
        let span = module.source_map.item_spans.get(&st.id).unwrap_or_default();

        table
            .insert(
                name.clone(),
                SymbolKind::Struct {
                    name: name.clone(),
                    fields: vec![],
                },
                span,
            )
            .ok();

        table.enter_scope(ScopeKind::Template { name: name.clone() });
        self.restore_fields(module, &st.fields, &name, table);
        table.exit_scope();
    }

    fn restore_template(&self, module: &Module, template: &Template, table: &mut SymbolTable) {
        let name = module.string_pool.resolve(template.name).to_string();
        let parent = template
            .parent
            .as_ref()
            .map(|item_ref| self.resolve_item_name(module, *item_ref));
        let fields = template
            .fields
            .iter()
            .map(|f| module.string_pool.resolve(f.name).to_string())
            .collect();
        let span = module
            .source_map
            .item_spans
            .get(&template.id)
            .unwrap_or_default();

        table
            .insert(
                name.clone(),
                SymbolKind::Template {
                    fields,
                    parent,
                    attributes: vec![],
                },
                span,
            )
            .ok();

        table.enter_scope(ScopeKind::Template { name: name.clone() });
        self.restore_fields(module, &template.fields, &name, table);
        table.exit_scope();
    }

    fn restore_type_alias(&self, module: &Module, ty: &TypeAlias, table: &mut SymbolTable) {
        let name = module.string_pool.resolve(ty.name).to_string();
        let expr = self.type_to_expression(module, &ty.target_type);
        let span = module.source_map.item_spans.get(&ty.id).unwrap_or_default();

        table
            .insert(
                name.clone(),
                SymbolKind::TypeAlias {
                    name: name.clone(),
                    data_type: expr,
                    attributes: vec![],
                },
                span,
            )
            .ok();
    }

    fn restore_enum(&self, module: &Module, enumeration: &Enum, table: &mut SymbolTable) {
        let name = module.string_pool.resolve(enumeration.name).to_string();
        let variants = enumeration
            .variants
            .iter()
            .map(|v| VariantInfo {
                name: module.string_pool.resolve(v.name).to_string(),
                weight: None,
            })
            .collect();
        let span = module
            .source_map
            .item_spans
            .get(&enumeration.id)
            .unwrap_or_default();

        table
            .insert(
                name.clone(),
                SymbolKind::Enum {
                    variants,
                    attributes: vec![],
                },
                span,
            )
            .ok();

        table.enter_scope(ScopeKind::Enum { name: name.clone() });
        self.restore_variants(module, &enumeration.variants, &name, table);
        table.exit_scope();
    }

    fn restore_variants(
        &self,
        module: &Module,
        variants: &[Variant],
        name: &str,
        table: &mut SymbolTable,
    ) {
        for variant in variants {
            let variant_name = module.string_pool.resolve(variant.name).to_string();

            table
                .insert(
                    variant_name,
                    SymbolKind::Variant {
                        enum_name: name.to_string(),
                    },
                    Span::default(),
                )
                .ok();
        }
    }

    fn restore_fields(
        &self,
        module: &Module,
        fields: &[Field],
        name: &str,
        table: &mut SymbolTable,
    ) {
        for field in fields {
            let field_name = module.string_pool.resolve(field.name).to_string();
            let expr = self.type_to_expression(module, &field.ty);

            table
                .insert(
                    field_name.clone(),
                    SymbolKind::Field {
                        template_name: name.to_string(),
                        is_override: false,
                        expression: expr,
                    },
                    Span::default(),
                )
                .ok();
        }
    }

    fn type_to_data_type_kind(&self, module: &Module, ty: &Type) -> DataTypeKind {
        match ty {
            Type::Int => DataTypeKind::Int,
            Type::String => DataTypeKind::Str,
            Type::Bool => DataTypeKind::Boolean,
            Type::Float => DataTypeKind::Float,
            Type::List(inner) => DataTypeKind::List(Box::new(DataType {
                kind: self.type_to_data_type_kind(module, inner),
                constraints: None,
                span: Span::default(),
            })),
            Type::Optional(inner) => self.type_to_data_type_kind(module, inner),
            Type::UserDefined(item_ref) => {
                DataTypeKind::Custom(self.resolve_item_name(module, *item_ref))
            }
        }
    }

    fn type_to_expression(&self, module: &Module, ty: &Type) -> Expression {
        Expression {
            kind: ExpressionKind::Type(DataType {
                kind: self.type_to_data_type_kind(module, ty),
                constraints: None,
                span: Span::default(),
            }),
            span: Span::default(),
        }
    }

    fn resolve_item_name(&self, module: &Module, item: GlobalItemId) -> String {
        if item.module == module.metadata.id {
            return module
                .get_item(item.item)
                .map(|i| module.string_pool.resolve(i.name()).to_string())
                .unwrap_or_default();
        }

        self.module_by_id(item.module)
            .and_then(|m| {
                m.get_item(item.item)
                    .map(|i| m.string_pool.resolve(i.name()).to_string())
            })
            .unwrap_or_default()
    }
}
