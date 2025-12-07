use crate::{
    ast::{
        Attribute, Field, Program,
        visitor::{Visitor, walk_template},
    },
    symbol_table::{
        SymbolTable,
        error::SymbolError,
        symbol::{ScopeKind, SymbolKind},
    },
    utils::Span,
};

#[derive(Debug, Default)]
pub struct SymbolTableBuilder {
    table: SymbolTable,
    errors: Vec<SymbolError>,
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn build(mut self, program: &Program) -> Result<SymbolTable, Vec<SymbolError>> {
        self.visit_program(program);

        if self.errors.is_empty() {
            Ok(self.table)
        } else {
            Err(self.errors)
        }
    }

    pub fn finish(self) -> (SymbolTable, Vec<SymbolError>) {
        (self.table, self.errors)
    }

    pub fn insert_error(&mut self, symbol_error: SymbolError) {
        self.errors.push(symbol_error);
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
                self.insert_error(SymbolError::InvalidContext {
                    message: format!("Field '{}' declared outside of valid scope", field.name),
                    span: field.span,
                });
                return;
            }
        };

        let parent_name = match &current_scope.kind {
            ScopeKind::Template { name } => name.clone(),
            _ => {
                self.insert_error(SymbolError::InvalidContext {
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
}
