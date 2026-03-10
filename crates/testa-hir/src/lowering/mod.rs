mod helpers;

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use testa_core::{
    analyser::{result::AnalysisResult, type_checker},
    ast::{
        Attribute, DataTypeKind, Expression, ExpressionKind, Field, Program, Statement, Variant,
    },
};

use crate::{
    FieldId, Item, Module, ModuleMetadata, StringPool,
    module::{
        Attribute as HirAttribute, Enum, Expr, Template, Type, TypeAlias, Variant as HirVariant,
        attribute_kind,
    },
    source_map::SourceMapBuilder,
};

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

    pub fn lower(
        mut self,
        ast: &Program,
        analysis: &AnalysisResult,
        source_text: &str,
        imported: &HashMap<String, &Module>,
    ) -> Module {
        for stmt in &ast.0 {
            self.lower_statement(stmt, analysis, imported);
        }

        let mut hasher = DefaultHasher::new();
        source_text.hash(&mut hasher);
        let source_hash = hasher.finish();
        let imports = imported
            .keys()
            .map(|name| self.string_pool.intern(name))
            .collect();

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
            imports,
            string_pool: self.string_pool,
            items: self.items,
            source_map: self.source_map_builder.build(),
        }
    }

    fn lower_statement(
        &mut self,
        stmt: &Statement,
        analysis: &AnalysisResult,
        imported: &HashMap<String, &Module>,
    ) {
        match stmt {
            Statement::Template {
                parent_name,
                attributes,
                name,
                name_span,
                body,
                span,
                ..
            } => {
                let template = self.lower_template(name, parent_name, body, attributes, imported);
                self.register_item(Item::Template(template), *span, *name_span);
            }

            Statement::Enum {
                name,
                name_span,
                variants,
                attributes,
                span,
            } => {
                let enum_item = self.lower_enum(name, variants, attributes);
                self.register_item(Item::Enum(enum_item), *span, *name_span);
            }

            Statement::TypeDecl {
                name,
                name_span,
                data_type,
                attributes,
                span,
                ..
            } => {
                let type_alias =
                    self.lower_type_alias(name, data_type, attributes, analysis, imported);
                self.register_item(Item::TypeAlias(type_alias), *span, *name_span);
            }

            _ => {}
        }
    }

    fn lower_template(
        &mut self,
        name: &str,
        parent_name: &Option<String>,
        body: &[Field],
        attributes: &[Attribute],
        imported: &HashMap<String, &Module>,
    ) -> Template {
        let id = self.next_item_id();
        let name_id = self.string_pool.intern(name);

        let parent = parent_name
            .as_ref()
            .and_then(|p| self.find_item_ref_by_name(p, imported));

        let fields = body
            .iter()
            .enumerate()
            .map(|(idx, field)| self.lower_field(field, idx, imported))
            .collect();

        let attrs = attributes
            .iter()
            .map(|attr| self.lower_attribute(attr))
            .collect();

        Template {
            id,
            name: name_id,
            parent,
            fields,
            attributes: attrs,
        }
    }

    fn lower_enum(&mut self, name: &str, variants: &[Variant], attributes: &[Attribute]) -> Enum {
        let id = self.next_item_id();
        let name_id = self.string_pool.intern(name);

        let hir_variants = variants.iter().map(|v| self.lower_variant(v)).collect();

        let attrs = attributes
            .iter()
            .map(|attr| self.lower_attribute(attr))
            .collect();

        Enum {
            id,
            name: name_id,
            variants: hir_variants,
            attributes: attrs,
        }
    }

    fn lower_variant(&mut self, variant: &Variant) -> HirVariant {
        let name_id = self.string_pool.intern(&variant.name);
        let weight = variant.weight.as_ref().map(|w| self.lower_expr(w));

        HirVariant {
            name: name_id,
            weight,
        }
    }

    fn lower_type_alias(
        &mut self,
        name: &str,
        data_type: &Expression,
        attributes: &[Attribute],
        analysis: &AnalysisResult,
        imported: &HashMap<String, &Module>,
    ) -> TypeAlias {
        let id = self.next_item_id();
        let name_id = self.string_pool.intern(name);

        let target_type = self.lower_type_from_expr(data_type, analysis, imported);

        let constraints = self.extract_constraints(data_type);

        let attrs = attributes
            .iter()
            .map(|attr| self.lower_attribute(attr))
            .collect();

        TypeAlias {
            id,
            name: name_id,
            target_type,
            constraints,
            attributes: attrs,
        }
    }

    fn lower_field(
        &mut self,
        field: &Field,
        idx: usize,
        imported: &HashMap<String, &Module>,
    ) -> crate::module::Field {
        let field_id = FieldId(idx as u32);
        let name_id = self.string_pool.intern(&field.name);

        self.source_map_builder.add_field(field_id, field.span);
        if let Some(name_span) = field.name_span {
            self.source_map_builder.add_identifier(name_id, name_span);
        }

        crate::module::Field {
            id: field_id,
            name: name_id,
            ty: self.lower_type(&field.value, imported),
            default_value: None, // Could be extracted from field.value if it's a literal
            attributes: field
                .attributes
                .iter()
                .map(|attr| self.lower_attribute(attr))
                .collect(),
        }
    }

    fn lower_attribute(&mut self, attr: &Attribute) -> HirAttribute {
        use testa_core::ast::Attribute as AstAttr;

        match attr {
            AstAttr::Flag(name, _) => {
                let kind = attribute_kind(name.as_str());
                HirAttribute {
                    kind,
                    args: Vec::new(),
                }
            }

            AstAttr::KeyValue(name, value, _) => {
                let kind = attribute_kind(name.as_str());
                let arg = Expr::String(self.string_pool.intern(value));

                HirAttribute {
                    kind,
                    args: vec![arg],
                }
            }
        }
    }

    fn lower_type(&mut self, expr: &Expression, imported: &HashMap<String, &Module>) -> Type {
        match &expr.kind {
            ExpressionKind::Identifier(name) => match name.as_str() {
                "int" => Type::Int,
                "string" => Type::String,
                "bool" => Type::Bool,
                "float" => Type::Float,
                _ => self
                    .find_item_ref_by_name(name, imported)
                    .map(Type::UserDefined)
                    .unwrap_or(Type::Int),
            },
            ExpressionKind::Type(data_type) => match &data_type.kind {
                DataTypeKind::Int => Type::Int,
                DataTypeKind::Str => Type::String,
                DataTypeKind::Boolean => Type::Bool,
                DataTypeKind::Float => Type::Float,
                DataTypeKind::List(inner) => {
                    let inner_expr = Expression {
                        kind: ExpressionKind::Type(*inner.clone()),
                        span: expr.span,
                    };
                    Type::List(Box::new(self.lower_type(&inner_expr, imported)))
                }
                DataTypeKind::Custom(name) => self
                    .find_item_ref_by_name(name, imported)
                    .map(Type::UserDefined)
                    .unwrap_or(Type::Int),
            },
            _ => Type::Int,
        }
    }

    fn lower_type_from_expr(
        &mut self,
        expr: &Expression,
        analysis: &AnalysisResult,
        imported: &HashMap<String, &Module>,
    ) -> Type {
        if let ExpressionKind::Identifier(name) = &expr.kind {
            if let Some(checker_type) = analysis.type_map.get(name) {
                return match checker_type {
                    type_checker::types::Type::Custom(name) => self
                        .find_item_ref_by_name(name, imported)
                        .map(Type::UserDefined)
                        .unwrap_or(Type::Int),
                    other => Type::from(other),
                };
            }
        }
        self.lower_type(expr, imported)
    }

    fn lower_expr(&mut self, expr: &Expression) -> Expr {
        match &expr.kind {
            ExpressionKind::IntLiteral(n) => Expr::Int(*n as i64),

            ExpressionKind::FloatLiteral(f) => f
                .parse::<f64>()
                .map(Expr::Float)
                .unwrap_or(Expr::Float(0.0)),
            ExpressionKind::StringLiteral(s) => Expr::String(self.string_pool.intern(s)),
            ExpressionKind::BooleanLiteral(b) => Expr::Bool(*b),
            ExpressionKind::List(elements) => {
                let exprs = elements
                    .iter()
                    .map(|elem| self.lower_expr(&elem.value))
                    .collect();
                Expr::List(exprs)
            }

            ExpressionKind::Infix {
                left,
                right,
                operator,
                ..
            } => {
                use testa_core::ast::InfixOperator;

                match operator {
                    InfixOperator::ExclusiveRange | InfixOperator::InclusiveRange => Expr::Range {
                        start: Box::new(self.lower_expr(left)),
                        end: Box::new(self.lower_expr(right)),
                        inclusive: matches!(operator, InfixOperator::InclusiveRange),
                    },
                    _ => self.lower_expr(left),
                }
            }

            _ => Expr::Int(0),
        }
    }
}
