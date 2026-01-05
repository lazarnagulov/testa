use crate::{
    analyser::{
        error::SemanticError,
        symbol_table::{
            SymbolTable,
            symbol::{ScopeKind, SymbolKind, VariantInfo},
        },
    },
    ast::{
        Attribute, Expression, Field, Program, Variant,
        visitor::{Visitor, walk_enum, walk_template},
    },
    utils::Span,
};
#[derive(Debug, Default)]
pub struct SymbolTableBuilder {
    table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn build(mut self, program: &Program) -> Result<SymbolTable, Vec<SemanticError>> {
        self.visit_program(program);

        if self.errors.is_empty() {
            Ok(self.table)
        } else {
            Err(self.errors)
        }
    }

    pub fn finish(self) -> (SymbolTable, Vec<SemanticError>) {
        (self.table, self.errors)
    }

    pub fn insert_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }
}

impl Visitor for SymbolTableBuilder {
    fn visit_template(
        &mut self,
        parent: &Option<String>,
        attributes: &[Attribute],
        name: &str,
        body: &[Field],
        span: Span,
    ) {
        if let Err(symbol_error) = self.table.insert(
            name.to_string(),
            SymbolKind::Template {
                parent: parent.clone(),
                fields: body.iter().map(|f| f.name.clone()).collect(),
                attributes: attributes.to_vec(),
            },
            span,
        ) {
            self.insert_error(symbol_error);
        }

        self.table.enter_scope(ScopeKind::Template {
            name: name.to_string(),
        });
        walk_template(self, attributes, body);
        self.table.exit_scope();
    }

    fn visit_field(&mut self, field: &Field) {
        let current_scope = match self.table.get_current_scope() {
            Some(scope) => scope,
            None => {
                self.insert_error(SemanticError::InvalidContext {
                    message: format!("Field '{}' declared outside of valid scope", field.name),
                    span: field.span,
                });
                return;
            }
        };

        let parent_name = match &current_scope.kind {
            ScopeKind::Template { name }
            | ScopeKind::Generate { name }
            | ScopeKind::Directive { name } => name.clone(),
            scope => {
                self.insert_error(SemanticError::InvalidContext {
                    message: format!(
                        "Field '{}' declared in invalid scope: {}",
                        field.name, scope
                    ),
                    span: field.span,
                });
                return;
            }
        };

        if let Err(symbol_error) = self.table.insert(
            field.name.clone(),
            SymbolKind::Field {
                is_override: field.overridable,
                template_name: parent_name,
                expression: field.value.clone(),
            },
            field.span,
        ) {
            self.insert_error(symbol_error);
        }
    }

    fn visit_output_directive(&mut self, _argument: &str, options: &[Field], _span: Span) {
        if !options.is_empty() {
            self.table.enter_scope(ScopeKind::Directive {
                name: "output".to_string(),
            });
            for field in options {
                self.visit_field(field);
            }
            self.table.exit_scope();
        }
    }

    fn visit_type_decl(
        &mut self,
        name: &str,
        data_type: &Expression,
        attributes: &[Attribute],
        span: Span,
    ) {
        // TODO: it is not a good idea to clone ast node, think about doing it better
        if let Err(symbol_error) = self.table.insert(
            name.to_string(),
            SymbolKind::TypeAlias {
                name: name.to_string(),
                data_type: data_type.clone(),
                attributes: attributes.to_vec(),
            },
            span,
        ) {
            self.insert_error(symbol_error);
        }
    }

    fn visit_generate(
        &mut self,
        template_name: &Option<String>,
        body: &[Field],
        _count: &Expression,
        _span: Span,
    ) {
        if template_name.is_some() {
            return;
        }

        self.table.enter_scope(ScopeKind::Generate {
            name: "Generate".to_string(),
        });

        for field in body {
            self.visit_field(field);
        }

        self.table.exit_scope();
    }

    fn visit_enum(
        &mut self,
        name: &str,
        variants: &[Variant],
        attributes: &[Attribute],
        span: Span,
    ) {
        let variant_infos = variants
            .iter()
            .map(|v| VariantInfo {
                name: v.name.clone(),
                weight: v.weight.clone(),
            })
            .collect::<Vec<VariantInfo>>();

        if let Err(symbol_error) = self.table.insert(
            name.to_string(),
            SymbolKind::Enum {
                variants: variant_infos,
                attributes: attributes.to_vec(),
            },
            span,
        ) {
            self.insert_error(symbol_error);
        }

        self.table.enter_scope(ScopeKind::Enum {
            name: name.to_string(),
        });
        walk_enum(self, attributes, variants);
        self.table.exit_scope();
    }

    fn visit_variant(&mut self, variant: &Variant) {
        let current_scope = match self.table.get_current_scope() {
            Some(scope) => scope,
            None => {
                self.insert_error(SemanticError::InvalidContext {
                    message: format!("Variant '{}' declared outside of valid scope", variant.name),
                    span: variant.span,
                });
                return;
            }
        };

        let enum_name = match &current_scope.kind {
            ScopeKind::Enum { name } => name.clone(),
            _ => {
                self.insert_error(SemanticError::InvalidContext {
                    message: format!("Variant '{}' declared in invalid scope", variant.name),
                    span: variant.span,
                });
                return;
            }
        };

        if let Err(symbol_error) = self.table.insert(
            variant.name.clone(),
            SymbolKind::Variant { enum_name },
            variant.span,
        ) {
            self.insert_error(symbol_error);
        }
    }
}
