use std::{
    fs::File,
    io::{Read as _, Write},
    path::Path,
};

use testa_core::{
    analyser::symbol_table::SymbolTable,
    ast::{DataTypeKind, Expression},
    utils::Span,
};

use crate::{
    Item, Module,
    module::{ItemRef, Type, error::ResolveError},
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
        use testa_core::{
            analyser::symbol_table::symbol::{ScopeKind, SymbolKind, VariantInfo},
            ast::{DataType, DataTypeKind, Expression, ExpressionKind},
            utils::Span,
        };

        let mut table = SymbolTable::new();

        for item in &self.items {
            match item {
                Item::Template(t) => {
                    let name = self.string_pool.resolve(t.name).to_string();
                    let parent = t.parent.as_ref().and_then(|item_ref| {
                        let parent_id = match item_ref {
                            ItemRef::Local(id) => *id,
                            ItemRef::Imported { item, .. } => *item,
                        };
                        self.get_item(parent_id)
                            .map(|item| self.string_pool.resolve(item.name()).to_string())
                    });

                    let fields = t
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
                    for field in &t.fields {
                        let field_name = self.string_pool.resolve(field.name).to_string();
                        let expr = self.type_to_expression(&field.ty);

                        table
                            .insert(
                                field_name.clone(),
                                SymbolKind::Field {
                                    template_name: name.clone(),
                                    is_override: false,
                                    expression: expr,
                                },
                                Span::default(),
                            )
                            .ok();
                    }
                    table.exit_scope();
                }

                Item::Enum(e) => {
                    let name = self.string_pool.resolve(e.name).to_string();

                    let variants = e
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
                    for variant in &e.variants {
                        let variant_name = self.string_pool.resolve(variant.name).to_string();
                        table
                            .insert(
                                variant_name,
                                SymbolKind::Variant {
                                    enum_name: name.clone(),
                                },
                                Span::default(),
                            )
                            .ok();
                    }
                    table.exit_scope();
                }

                Item::TypeAlias(t) => {
                    let name = self.string_pool.resolve(t.name).to_string();

                    let data_type_kind = match &t.target_type {
                        Type::Int => DataTypeKind::Int,
                        Type::String => DataTypeKind::Str,
                        Type::Bool => DataTypeKind::Boolean,
                        Type::Float => DataTypeKind::Float,
                        Type::List(_) => DataTypeKind::List(Box::new(DataType {
                            kind: DataTypeKind::Int,
                            constraints: None,
                            span: Span::default(),
                        })),
                        Type::Optional(_) => DataTypeKind::Int,
                        Type::UserDefined(item_ref) => {
                            let id = match item_ref {
                                ItemRef::Local(id) => *id,
                                ItemRef::Imported { item, .. } => *item,
                            };
                            let type_name = self
                                .get_item(id)
                                .map(|i| self.string_pool.resolve(i.name()).to_string())
                                .unwrap_or_default();
                            DataTypeKind::Custom(type_name)
                        }
                    };

                    let expr = Expression {
                        kind: ExpressionKind::Type(DataType {
                            kind: data_type_kind,
                            constraints: None,
                            span: Span::default(),
                        }),
                        span: Span::default(),
                    };

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
            }
        }

        table
    }

    fn type_to_expression(&self, ty: &Type) -> Expression {
        use testa_core::ast::{DataType, DataTypeKind, Expression, ExpressionKind};

        let kind = match ty {
            Type::Int => ExpressionKind::Type(DataType {
                kind: DataTypeKind::Int,
                constraints: None,
                span: Span::default(),
            }),
            Type::String => ExpressionKind::Type(DataType {
                kind: DataTypeKind::Str,
                constraints: None,
                span: Span::default(),
            }),
            Type::Bool => ExpressionKind::Type(DataType {
                kind: DataTypeKind::Boolean,
                constraints: None,
                span: Span::default(),
            }),
            Type::Float => ExpressionKind::Type(DataType {
                kind: DataTypeKind::Float,
                constraints: None,
                span: Span::default(),
            }),
            Type::List(inner) => ExpressionKind::Type(DataType {
                kind: DataTypeKind::List(Box::new(DataType {
                    kind: self.data_type_kind_from_type(inner),
                    constraints: None,
                    span: Span::default(),
                })),
                constraints: None,
                span: Span::default(),
            }),
            Type::Optional(inner) => ExpressionKind::Type(DataType {
                kind: self.data_type_kind_from_type(inner),
                constraints: None,
                span: Span::default(),
            }),
            Type::UserDefined(item_ref) => {
                let id = match item_ref {
                    ItemRef::Local(id) => *id,
                    ItemRef::Imported { item, .. } => *item,
                };
                let type_name = self
                    .get_item(id)
                    .map(|i| self.string_pool.resolve(i.name()).to_string())
                    .unwrap_or_default();
                ExpressionKind::Identifier(type_name)
            }
        };

        Expression {
            kind,
            span: Span::default(),
        }
    }

    fn data_type_kind_from_type(&self, ty: &Type) -> DataTypeKind {
        match ty {
            Type::Int => DataTypeKind::Int,
            Type::String => DataTypeKind::Str,
            Type::Bool => DataTypeKind::Boolean,
            Type::Float => DataTypeKind::Float,
            Type::UserDefined(item_ref) => {
                let id = match item_ref {
                    ItemRef::Local(id) => *id,
                    ItemRef::Imported { item, .. } => *item,
                };
                let type_name = self
                    .get_item(id)
                    .map(|i| self.string_pool.resolve(i.name()).to_string())
                    .unwrap_or_default();
                DataTypeKind::Custom(type_name)
            }
            _ => DataTypeKind::Int,
        }
    }
}
