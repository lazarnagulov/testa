pub mod context;
mod helpers;

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use testa_core::{
    analyser::{result::AnalysisResult, symbol_table::symbol::SymbolKind, type_checker},
    ast::{
        Attribute, DataTypeKind, Expression, ExpressionKind, Field, Program, Statement, Variant,
    },
};

use crate::{
    FieldId, Item, Module, ModuleMetadata, StringPool,
    lowering::context::{ItemKind, LoweringContext},
    module::{
        Attribute as HirAttribute, Directive, Enum, Expr, InfixOp, ItemRef, PatternChar,
        PatternPart, PrefixOp, Struct, Template, Type, TypeAlias, Variant as HirVariant,
        attribute_kind,
    },
    source_map::SourceMapBuilder,
};

macro_rules! lower_infix {
    ($self:expr, $left:expr, $right:expr, $analysis:expr, $imported:expr, $op:expr) => {
        Expr::Infix {
            left: Box::new($self.lower_expr($left, $analysis, $imported)),
            right: Box::new($self.lower_expr($right, $analysis, $imported)),
            op: $op,
        }
    };
}

pub struct AstLowering {
    context: LoweringContext,

    string_pool: StringPool,
    next_item_id: u32,

    source_map_builder: SourceMapBuilder,
    directives: Vec<Directive>,
    source_file: PathBuf,
    items: Vec<Item>,
}

impl AstLowering {
    pub fn new(source_file: PathBuf) -> Self {
        Self {
            string_pool: StringPool::new(),
            next_item_id: 0,
            directives: Vec::new(),
            items: Vec::new(),
            context: LoweringContext::new(),
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
        self.register_items(ast, imported);
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
            directives: self.directives,
            string_pool: self.string_pool,
            items: self.items,
            source_map: self.source_map_builder.build(),
        }
    }

    fn register_items(&mut self, ast: &Program, imported: &HashMap<String, &Module>) {
        self.register_local_items(ast);
        self.register_imported_items(imported);
    }

    fn register_local_items(&mut self, ast: &Program) {
        for stmt in &ast.0 {
            let (kind, name) = match stmt {
                Statement::Template { name, .. } => (ItemKind::Template, name),
                Statement::Struct { name, .. } => (ItemKind::Struct, name),
                Statement::Enum { name, .. } => (ItemKind::Enum, name),
                Statement::TypeDecl { name, .. } => (ItemKind::TypeAlias, name),
                _ => continue,
            };

            let id = self.next_item_id();
            self.context
                .register_local(kind, self.string_pool.intern(name), id);
        }
    }

    fn register_imported_items(&mut self, imported: &HashMap<String, &Module>) {
        for (module_name, module) in imported {
            let module_id = self.string_pool.intern(module_name);

            for item in &module.items {
                self.context
                    .register_import(item.kind(), module_id, item.name(), item.id());
            }
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
                let template =
                    self.lower_template(analysis, name, parent_name, body, attributes, imported);
                self.register_item(Item::Template(template), *span, *name_span);
            }
            Statement::Enum {
                name,
                name_span,
                variants,
                attributes,
                span,
            } => {
                let enum_item = self.lower_enum(analysis, name, variants, attributes);
                self.register_item(Item::Enum(enum_item), *span, *name_span);
            }
            Statement::Struct {
                name,
                name_span,
                body,
                span,
            } => {
                let struct_item = self.lower_struct(analysis, name, body, imported);
                self.register_item(Item::Struct(struct_item), *span, Some(*name_span));
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
            Statement::OutputDirective {
                argument, options, ..
            } => {
                let format_id = self.string_pool.intern(argument);
                let opts = options
                    .iter()
                    .map(|field| {
                        let name = self.string_pool.intern(&field.name);
                        let value = self.lower_expr(&field.value, analysis, imported);
                        (name, value)
                    })
                    .collect();
                self.directives.push(Directive::Output {
                    format: format_id,
                    options: opts,
                });
            }

            Statement::OutputPathDirective { argument, .. } => {
                let path_id = self
                    .string_pool
                    .intern(argument.to_str().unwrap_or_default());
                self.directives.push(Directive::OutputPath(path_id));
            }

            Statement::Generate {
                template_name,
                body,
                count,
                ..
            } => {
                let count_expr = self.lower_expr(count, analysis, imported);

                let template_ref = if let Some(name) = template_name {
                    self.resolve_item(ItemKind::Template, name)
                } else if !body.is_empty() {
                    let id = self.next_item_id();
                    let name_id = self.string_pool.intern("_");
                    let fields = body
                        .iter()
                        .enumerate()
                        .map(|(idx, field)| self.lower_field(analysis, field, idx, imported))
                        .collect();
                    let template = Template {
                        id,
                        name: name_id,
                        parent: None,
                        fields,
                        attributes: vec![],
                    };
                    self.items.push(Item::Template(template));
                    Some(ItemRef::Local(id))
                } else {
                    None
                };

                if let Some(template_ref) = template_ref {
                    self.directives.push(Directive::Generate {
                        template: template_ref,
                        count: count_expr,
                    });
                }
            }
            Statement::ImportDirective { .. } => {} // consumed at compile time, skip
            _ => {}
        }
    }

    fn lower_template(
        &mut self,
        analysis: &AnalysisResult,
        name: &str,
        parent_name: &Option<String>,
        body: &[Field],
        attributes: &[Attribute],
        imported: &HashMap<String, &Module>,
    ) -> Template {
        let (id, name_id) = self.resolve_local_item(ItemKind::Template, name);

        let parent = parent_name
            .as_ref()
            .and_then(|p| self.resolve_item(ItemKind::Template, p));

        let fields = body
            .iter()
            .enumerate()
            .map(|(idx, field)| self.lower_field(analysis, field, idx, imported))
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

    fn lower_struct(
        &mut self,
        analysis: &AnalysisResult,
        name: &str,
        body: &[Field],
        imported: &HashMap<String, &Module>,
    ) -> Struct {
        let (id, name_id) = self.resolve_local_item(ItemKind::Struct, name);

        let fields = body
            .iter()
            .enumerate()
            .map(|(idx, field)| self.lower_field(analysis, field, idx, imported))
            .collect();

        Struct {
            id,
            name: name_id,
            fields,
        }
    }

    fn lower_enum(
        &mut self,
        analysis: &AnalysisResult,
        name: &str,
        variants: &[Variant],
        attributes: &[Attribute],
    ) -> Enum {
        let (id, name_id) = self.resolve_local_item(ItemKind::Enum, name);

        let hir_variants = variants
            .iter()
            .map(|v| self.lower_variant(analysis, v))
            .collect();

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

    fn lower_variant(&mut self, analysis: &AnalysisResult, variant: &Variant) -> HirVariant {
        let name_id = self.string_pool.intern(&variant.name);
        let weight = variant
            .weight
            .as_ref()
            .map(|w| self.lower_expr(w, analysis, &HashMap::new()));

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
        let (id, name_id) = self.resolve_local_item(ItemKind::TypeAlias, name);

        let expr = match &data_type.kind {
            ExpressionKind::StringPattern(_) => {
                Some(self.lower_expr(data_type, analysis, imported))
            }
            _ => None,
        };

        let target_type = self.lower_type_from_expr(data_type, analysis);

        let constraints = self.extract_constraints(analysis, data_type);

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
            expr,
        }
    }

    fn lower_field(
        &mut self,
        analysis: &AnalysisResult,
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
            ty: self.lower_type(&field.value),
            value: self.lower_expr(&field.value, analysis, imported),
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

    fn lower_type(&mut self, expr: &Expression) -> Type {
        match &expr.kind {
            ExpressionKind::Identifier(name) => match name.as_str() {
                "int" => Type::Int,
                "string" => Type::String,
                "bool" => Type::Bool,
                "float" => Type::Float,
                _ => self
                    .resolve_item(ItemKind::TypeAlias, name)
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
                    Type::List(Box::new(self.lower_type(&inner_expr)))
                }
                DataTypeKind::Custom(name) => self
                    .resolve_item(ItemKind::TypeAlias, name)
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
    ) -> Type {
        if let ExpressionKind::Identifier(name) = &expr.kind
            && let Some(checker_type) = analysis.type_map.get(name)
        {
            return match checker_type {
                type_checker::types::Type::Custom(name) => self
                    .resolve_item(ItemKind::TypeAlias, name)
                    .map(Type::UserDefined)
                    .unwrap_or(Type::Int),
                other => Type::from(other),
            };
        }
        self.lower_type(expr)
    }

    fn lower_expr(
        &mut self,
        expr: &Expression,
        analysis: &AnalysisResult,
        imported: &HashMap<String, &Module>,
    ) -> Expr {
        use testa_core::ast::{InfixOperator, PrefixOperator};

        match &expr.kind {
            ExpressionKind::IntLiteral(n) => Expr::Int(*n as i64),
            ExpressionKind::FloatLiteral(f) => f
                .parse::<f64>()
                .map(Expr::Float)
                .unwrap_or(Expr::Float(0.0)),
            ExpressionKind::StringLiteral(s) => Expr::String(self.string_pool.intern(s)),
            ExpressionKind::BooleanLiteral(b) => Expr::Bool(*b),
            ExpressionKind::Type(data_type) => {
                if data_type.constraints.is_some() {
                    let ty = self.lower_type(expr);
                    let constraints = self.extract_constraints(analysis, expr);
                    Expr::ConstrainedType { ty, constraints }
                } else {
                    Expr::Type(self.lower_type(expr))
                }
            }
            ExpressionKind::StringPattern(elements) => {
                use testa_core::ast::{PatternChar as AstPatternChar, PatternElement};

                let parts = elements
                    .iter()
                    .map(|elem| match elem {
                        PatternElement::Literal(s, _) => {
                            PatternPart::Literal(self.string_pool.intern(s))
                        }
                        PatternElement::RepeatChar {
                            ch,
                            count,
                            count_expression,
                            ..
                        } => PatternPart::RepeatChar {
                            ch: match ch {
                                AstPatternChar::Lowercase => PatternChar::Lowercase,
                                AstPatternChar::Uppercase => PatternChar::Uppercase,
                                AstPatternChar::Digit => PatternChar::Digit,
                            },
                            count: *count,
                            count_expr: count_expression
                                .as_ref()
                                .map(|e| Box::new(self.lower_expr(e, analysis, imported))),
                        },
                        PatternElement::RepeatGroup { chars, count, .. } => {
                            PatternPart::RepeatGroup {
                                chars: chars
                                    .iter()
                                    .map(|ch| match ch {
                                        AstPatternChar::Lowercase => PatternChar::Lowercase,
                                        AstPatternChar::Uppercase => PatternChar::Uppercase,
                                        AstPatternChar::Digit => PatternChar::Digit,
                                    })
                                    .collect(),
                                count: Box::new(self.lower_expr(count, analysis, imported)),
                            }
                        }
                    })
                    .collect();
                Expr::StringPattern(parts)
            }
            ExpressionKind::Identifier(name) => match name.as_str() {
                "int" => Expr::Type(Type::Int),
                "string" => Expr::Type(Type::String),
                "bool" => Expr::Type(Type::Bool),
                "float" => Expr::Type(Type::Float),
                _ => self
                    .resolve_item(ItemKind::TypeAlias, name)
                    .map(Expr::Identifier)
                    .unwrap_or(Expr::Int(0)),
            },
            ExpressionKind::List(elements) => {
                let exprs = elements
                    .iter()
                    .map(|elem| self.lower_expr(&elem.value, analysis, imported))
                    .collect();
                Expr::List(exprs)
            }
            ExpressionKind::Reference { template, field } => {
                if let Some(template_ref) = self.resolve_item(ItemKind::Template, template) {
                    let field_id = analysis
                        .symbol_table
                        .get_template(template)
                        .and_then(|sym| {
                            if let SymbolKind::Template { fields, .. } = &sym.kind {
                                fields
                                    .iter()
                                    .position(|f| f == field)
                                    .map(|i| FieldId(i as u32))
                            } else {
                                None
                            }
                        })
                        .or_else(|| {
                            imported.values().find_map(|module| {
                                let item = module.get_item_by_name(template)?;
                                let t = match item {
                                    Item::Template(t) => t,
                                    _ => return None,
                                };
                                t.fields
                                    .iter()
                                    .position(|f| module.string_pool.resolve(f.name) == field)
                                    .map(|i| FieldId(i as u32))
                            })
                        })
                        .unwrap_or(FieldId(0));
                    Expr::Reference {
                        template: template_ref,
                        field: field_id,
                    }
                } else {
                    Expr::Int(0)
                }
            }
            ExpressionKind::Infix {
                left,
                operator,
                right,
            } => match operator {
                InfixOperator::ExclusiveRange => Expr::Range {
                    start: Box::new(self.lower_expr(left, analysis, imported)),
                    end: Box::new(self.lower_expr(right, analysis, imported)),
                    inclusive: false,
                },
                InfixOperator::InclusiveRange => Expr::Range {
                    start: Box::new(self.lower_expr(left, analysis, imported)),
                    end: Box::new(self.lower_expr(right, analysis, imported)),
                    inclusive: true,
                },
                InfixOperator::Plus => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::Add)
                }
                InfixOperator::Minus => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::Sub)
                }
                InfixOperator::Multiply => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::Mul)
                }
                InfixOperator::Divide => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::Div)
                }
                InfixOperator::Mod => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::Mod)
                }
                InfixOperator::BitAnd => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::BitAnd)
                }
                InfixOperator::BitOr => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::BitOr)
                }
                InfixOperator::BitXor => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::BitXor)
                }
                InfixOperator::BitLShift => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::BitLShift)
                }
                InfixOperator::BitRShift => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::BitRShift)
                }
                InfixOperator::Equal => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::Equal)
                }
                InfixOperator::And => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::And)
                }
                InfixOperator::Or => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::Or)
                }
                InfixOperator::NotEqual => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::NotEqual)
                }
                InfixOperator::LessThan => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::LessThen)
                }
                InfixOperator::GreaterThan => {
                    lower_infix!(self, left, right, analysis, imported, InfixOp::GreaterThan)
                }
                InfixOperator::LessThanOrEqual => {
                    lower_infix!(
                        self,
                        left,
                        right,
                        analysis,
                        imported,
                        InfixOp::LessThanOrEqual
                    )
                }
                InfixOperator::GreaterThanOrEqual => {
                    lower_infix!(
                        self,
                        left,
                        right,
                        analysis,
                        imported,
                        InfixOp::GreaterThanOrEqual
                    )
                }
            },
            ExpressionKind::Prefix {
                operator,
                expression,
            } => {
                let op = match operator {
                    PrefixOperator::Negative => PrefixOp::Neg,
                    PrefixOperator::LogicalNegate => PrefixOp::Not,
                    PrefixOperator::BitNegate => PrefixOp::BitNeg,
                };
                Expr::Prefix {
                    op,
                    expr: Box::new(self.lower_expr(expression, analysis, imported)),
                }
            }

            ExpressionKind::FuncCall { .. } => Expr::Int(0), // not implemented yet
        }
    }
}
