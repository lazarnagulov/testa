pub mod error;

use crate::{
    ast::{
        Attribute, Expression, ExpressionKind, Field, Program,
        visitor::{Visitor, walk_expression, walk_field, walk_template},
    },
    reference_checker::error::SemanticError,
    symbol_table::SymbolTable,
    utils::Span,
};

pub struct ReferenceChecker {
    symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl ReferenceChecker {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Self {
            symbol_table,
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

    pub fn finish(self) -> (SymbolTable, Vec<SemanticError>) {
        (self.symbol_table, self.errors)
    }
}

impl Visitor for ReferenceChecker {
    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Identifier(name) => {
                if self.symbol_table.lookup(name).is_none() {
                    self.errors.push(SemanticError::UnknownIdentifier {
                        name: name.clone(),
                        span: expression.span,
                    });
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
            if self.symbol_table.lookup(parent).is_none() {
                self.errors.push(SemanticError::UnknownParentTemplate {
                    name: parent.clone(),
                    span,
                });
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
            if self.symbol_table.lookup(name).is_none() {
                self.errors.push(SemanticError::UnknownTemplate {
                    name: name.clone(),
                    span,
                });
            }
        }
        for field in body {
            walk_field(self, field);
        }
        self.visit_expression(count);
    }

}
