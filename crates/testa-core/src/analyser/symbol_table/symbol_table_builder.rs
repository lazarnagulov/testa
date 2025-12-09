use crate::{
    analyser::{error::SemanticError, symbol_table::{SymbolTable, symbol::{ScopeKind, SymbolKind}}}, ast::{
        Attribute, Expression, Field, Program, Variant,
        visitor::{Visitor, walk_enum, walk_template},
    }, utils::Span
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
            ScopeKind::Template { name } | ScopeKind::Generate { name } => name.clone(),
            _ => {
                self.insert_error(SemanticError::InvalidContext {
                    message: format!("Field '{}' declared in invalid scope", field.name),
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
            },
            field.span,
        ) {
            self.insert_error(symbol_error);
        }
    }

    fn visit_type_decl(&mut self, name: &str, attributes: &[Attribute], span: Span) {
        if let Err(symbol_error) = self.table.insert(
            name.to_string(),
            SymbolKind::TypeAlias {
                name: name.to_string(),
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
        if let Err(symbol_error) = self.table.insert(
            name.to_string(),
            SymbolKind::Enum {
                variants: variants.iter().map(|f| f.name.clone()).collect(),
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
