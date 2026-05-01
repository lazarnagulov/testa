use std::{
    fs::File,
    io::{Read as _, Write},
    path::Path,
};

use testa_core::{
    analyser::symbol_table::{
        SymbolTable,
        symbol::{ScopeKind, SymbolKind, VariantInfo},
    },
    ast::{DataType, DataTypeKind, Expression, ExpressionKind},
    utils::Span,
};

use crate::{
    Item, Module,
    module::{Enum, Field, ItemRef, Template, Type, TypeAlias, Variant, error::ResolveError},
};

impl Module {
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = rmp_serde::to_vec(self)?;
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, ResolveError> {
        let mut file = File::open(path)
            .map_err(|err| ResolveError::Io("Failed to open {path}".to_string(), err))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|err| ResolveError::Io("Failed to read file at {path}".to_string(), err))?;
        let module = rmp_serde::from_slice(&buffer).map_err(|err| {
            ResolveError::Deserialize("Failed to deserialize".to_string(), Box::new(err))
        })?;
        Ok(module)
    }

    pub fn to_symbol_table(&self) -> SymbolTable {
        let mut table = SymbolTable::new();

        for item in &self.items {
            match item {
                Item::Template(t) => self.restore_template(t, &mut table),
                Item::Enum(e) => self.restore_enum(e, &mut table),
                Item::TypeAlias(t) => self.restore_type_alias(t, &mut table),
                Item::Struct(_) => {}
            }
        }

        table
    }

    fn restore_template(&self, template: &Template, table: &mut SymbolTable) {
        let name = self.string_pool.resolve(template.name).to_string();
        let parent = template
            .parent
            .as_ref()
            .map(|item_ref| self.resolve_item_ref_name(item_ref));
        let fields = template
            .fields
            .iter()
            .map(|f| self.string_pool.resolve(f.name).to_string())
            .collect();

        table
            .insert(
                name.clone(),
                SymbolKind::Template {
                    fields,
                    parent,
                    attributes: vec![],
                },
                Span::default(),
            )
            .ok();

        table.enter_scope(ScopeKind::Template { name: name.clone() });
        self.restore_fields(&template.fields, &name, table);
        table.exit_scope();
    }

    fn restore_type_alias(&self, ty: &TypeAlias, table: &mut SymbolTable) {
        let name = self.string_pool.resolve(ty.name).to_string();
        let expr = self.type_to_expression(&ty.target_type);

        table
            .insert(
                name.clone(),
                SymbolKind::TypeAlias {
                    name: name.clone(),
                    data_type: expr,
                    attributes: vec![],
                },
                Span::default(),
            )
            .ok();
    }

    fn restore_enum(&self, enumeration: &Enum, table: &mut SymbolTable) {
        let name = self.string_pool.resolve(enumeration.name).to_string();
        let variants = enumeration
            .variants
            .iter()
            .map(|v| VariantInfo {
                name: self.string_pool.resolve(v.name).to_string(),
                weight: None,
            })
            .collect();

        table
            .insert(
                name.clone(),
                SymbolKind::Enum {
                    variants,
                    attributes: vec![],
                },
                Span::default(),
            )
            .ok();

        table.enter_scope(ScopeKind::Enum { name: name.clone() });
        self.restore_variants(&enumeration.variants, &name, table);
        table.exit_scope();
    }

    fn restore_variants(&self, variants: &[Variant], name: &str, table: &mut SymbolTable) {
        for variant in variants {
            let variant_name = self.string_pool.resolve(variant.name).to_string();
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

    fn restore_fields(&self, fields: &[Field], name: &str, table: &mut SymbolTable) {
        for field in fields {
            let field_name = self.string_pool.resolve(field.name).to_string();
            let expr = self.type_to_expression(&field.ty);

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

    fn type_to_data_type_kind(&self, ty: &Type) -> DataTypeKind {
        match ty {
            Type::Int => DataTypeKind::Int,
            Type::String => DataTypeKind::Str,
            Type::Bool => DataTypeKind::Boolean,
            Type::Float => DataTypeKind::Float,
            Type::List(inner) => DataTypeKind::List(Box::new(DataType {
                kind: self.type_to_data_type_kind(inner),
                constraints: None,
                span: Span::default(),
            })),
            Type::Optional(inner) => self.type_to_data_type_kind(inner),
            Type::UserDefined(item_ref) => {
                DataTypeKind::Custom(self.resolve_item_ref_name(item_ref))
            }
        }
    }

    fn type_to_expression(&self, ty: &Type) -> Expression {
        Expression {
            kind: ExpressionKind::Type(DataType {
                kind: self.type_to_data_type_kind(ty),
                constraints: None,
                span: Span::default(),
            }),
            span: Span::default(),
        }
    }

    fn resolve_item_ref_name(&self, item_ref: &ItemRef) -> String {
        let id = match item_ref {
            ItemRef::Local(id) => *id,
            // NOTE: for imported items we only have the item ID, not the module.
            // This is sufficient for reference checking but not for cross-module resolution.
            ItemRef::Imported { item, .. } => *item,
        };
        self.get_item(id)
            .map(|i| self.string_pool.resolve(i.name()).to_string())
            .unwrap_or_default()
    }
}
