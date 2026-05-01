use crate::{
    analyser::{
        error::SemanticError,
        symbol_table::{SymbolTable, symbol::SymbolKind},
    },
    ast::{
        Attribute, DataType, DataTypeKind, Expression, ExpressionKind, Field, Program,
        visitor::{Visitor, walk_expression, walk_field, walk_template},
    },
    utils::Span,
};

#[cfg(test)]
mod tests;

// TODO: Should not allow template references in template and structs.
pub struct ReferenceChecker<'a> {
    symbol_table: &'a SymbolTable,
    imported: &'a [&'a SymbolTable],
    errors: Vec<SemanticError>,
}

impl<'a> ReferenceChecker<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            imported: &[],
            errors: Vec::new(),
        }
    }

    pub fn with_imports(symbol_table: &'a SymbolTable, imported: &'a [&'a SymbolTable]) -> Self {
        Self {
            symbol_table,
            imported,
            errors: Vec::new(),
        }
    }

    pub fn check(mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        self.visit_program(program);

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }

    pub fn finish(self) -> (&'a SymbolTable, Vec<SemanticError>) {
        (self.symbol_table, self.errors)
    }

    fn lookup(&self, name: &str) -> bool {
        if self.symbol_table.lookup(name).is_some() {
            return true;
        }
        self.imported.iter().any(|st| st.lookup(name).is_some())
    }

    fn lookup_symbol(&self, name: &str) -> Option<&SymbolKind> {
        if let Some(symbol) = self.symbol_table.lookup(name) {
            return Some(&symbol.kind);
        }
        for st in self.imported {
            if let Some(symbol) = st.lookup(name) {
                return Some(&symbol.kind);
            }
        }
        None
    }
}

impl<'a> Visitor for ReferenceChecker<'a> {
    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Identifier(name) if !self.lookup(name) => {
                self.errors.push(SemanticError::UnknownIdentifier {
                    name: name.clone(),
                    span: expression.span,
                });
            }
            ExpressionKind::Type(DataType {
                kind: DataTypeKind::List(list_type),
                ..
            }) => {
                if let DataTypeKind::Custom(custom_type) = &list_type.kind {
                    if !self.lookup(custom_type) {
                        self.errors.push(SemanticError::UnknownIdentifier {
                            name: custom_type.clone(),
                            span: list_type.span,
                        });
                    }
                }
            }
            _ => walk_expression(self, expression),
        }
    }

    fn visit_template(
        &mut self,
        parent: &Option<String>,
        attributes: &[Attribute],
        _name: &str,
        body: &[Field],
        span: Span,
    ) {
        if let Some(parent) = parent {
            match self.lookup_symbol(parent) {
                Some(kind) => match &kind {
                    SymbolKind::Template { .. } => {}
                    kind => self.errors.push(SemanticError::TypeMismatch {
                        expected: "template".to_string(),
                        found: kind.to_string(),
                        span,
                    }),
                },
                None => {
                    self.errors.push(SemanticError::UnknownParentTemplate {
                        name: parent.clone(),
                        span,
                    });
                }
            }
        }

        walk_template(self, attributes, body);
    }

    fn visit_generate(
        &mut self,
        template_name: &Option<String>,
        body: &[Field],
        count: &Expression,
        span: Span,
    ) {
        if let Some(name) = template_name {
            match self.lookup_symbol(name) {
                Some(kind) => match &kind {
                    SymbolKind::Template { .. } => {}
                    kind => self.errors.push(SemanticError::TypeMismatch {
                        expected: "template".to_string(),
                        found: kind.to_string(),
                        span,
                    }),
                },
                None => {
                    self.errors.push(SemanticError::UnknownTemplate {
                        name: name.clone(),
                        span,
                    });
                }
            }
        }
        for field in body {
            walk_field(self, field);
        }
        self.visit_expression(count);
    }
}
