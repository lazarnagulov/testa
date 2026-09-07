use crate::analyser::error::SemanticError;
use crate::analyser::symbol_table::SymbolTable;
use crate::analyser::symbol_table::symbol::SymbolKind;
use crate::ast::visitor::*;
use crate::ast::*;
use crate::utils::Span;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Reference {
    pub name: String,
    pub span: Span,
    pub kind: ReferenceKind,
    pub is_resolved: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReferenceKind {
    Type,
    TemplateParent,
    TemplateGenerate,
    TemplateDecl,
    StructDecl,
    Enum,
    Field,
}

#[derive(Debug, Clone)]
pub struct ReferenceTracker<'a> {
    references: HashMap<String, Vec<Reference>>,
    symbol_table: &'a SymbolTable,
    imported: &'a [&'a SymbolTable],
    errors: Vec<SemanticError>,
}

impl<'a> ReferenceTracker<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            references: HashMap::new(),
            errors: Vec::new(),
            imported: &[],
        }
    }

    pub fn with_imports(symbol_table: &'a SymbolTable, imported: &'a [&'a SymbolTable]) -> Self {
        Self {
            symbol_table,
            imported,
            references: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn track_references(
        mut self,
        program: &Program,
    ) -> (HashMap<String, Vec<Reference>>, Vec<SemanticError>) {
        self.visit_program(program);
        (self.references, self.errors)
    }

    fn lookup(&self, name: &str) -> bool {
        self.symbol_table.lookup(name).is_some()
            || self.imported.iter().any(|st| st.lookup(name).is_some())
    }

    fn resolve_field(&self, template_name: &str, field_name: &str) -> bool {
        let mut current = template_name.to_string();
        let mut visited = std::collections::HashSet::new();

        loop {
            if !visited.insert(current.clone()) {
                return false;
            }

            let symbol = self.symbol_table.get_template(&current).or_else(|| {
                self.imported
                    .iter()
                    .find_map(|st| st.get_template(&current))
            });

            let Some(symbol) = symbol else { return false };

            match &symbol.kind {
                SymbolKind::Template { fields, parent, .. } => {
                    if fields.iter().any(|f| f == field_name) {
                        return true;
                    }
                    match parent {
                        Some(p) => current = p.clone(),
                        None => return false,
                    }
                }
                _ => return false,
            }
        }
    }

    fn add_reference(&mut self, name: String, span: Span, kind: ReferenceKind) {
        let is_resolved = self.lookup(&name);

        if !is_resolved {
            self.errors.push(SemanticError::UnknownIdentifier {
                span,
                name: name.clone(),
            });
        }

        self.references
            .entry(name.clone())
            .or_default()
            .push(Reference {
                name,
                span,
                kind,
                is_resolved,
            });
    }

    pub fn take_errors(&mut self) -> Vec<SemanticError> {
        std::mem::take(&mut self.errors)
    }
}

impl<'a> Visitor for ReferenceTracker<'a> {
    fn visit_template(
        &mut self,
        _parent: &Option<String>,
        attributes: &[Attribute],
        _name: &str,
        body: &[Field],
        _span: Span,
    ) {
        walk_template(self, attributes, body);
    }

    fn visit_generate(
        &mut self,
        _template_name: &Option<String>,
        body: &[Field],
        _count: &Expression,
        _span: Span,
    ) {
        for field in body {
            walk_field(self, field);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Template {
                name,
                name_span: Some(span),
                parent_name,
                parent_span,
                ..
            } => {
                self.add_reference(name.to_string(), *span, ReferenceKind::TemplateDecl);
                if let (Some(parent), Some(parent_span)) = (parent_name, parent_span) {
                    self.add_reference(
                        parent.to_string(),
                        *parent_span,
                        ReferenceKind::TemplateParent,
                    );
                }
            }
            Statement::Struct {
                name, name_span, ..
            } => {
                self.add_reference(name.to_string(), *name_span, ReferenceKind::StructDecl);
            }
            Statement::Generate {
                template_name: Some(name),
                template_name_span: Some(span),
                ..
            } => {
                self.add_reference(name.clone(), *span, ReferenceKind::TemplateGenerate);
            }
            Statement::Enum {
                name,
                name_span: Some(span),
                ..
            } => {
                self.add_reference(name.clone(), *span, ReferenceKind::Enum);
            }
            Statement::TypeDecl {
                name,
                name_span: Some(name_span),
                ..
            } => {
                self.add_reference(name.clone(), *name_span, ReferenceKind::Type);
            }

            _ => {}
        }

        walk_statement(self, stmt);
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Identifier(name) => {
                self.add_reference(name.clone(), expression.span, ReferenceKind::Type)
            }
            ExpressionKind::Type(DataType {
                kind: DataTypeKind::Custom(custom_type),
                span,
                ..
            }) => {
                self.add_reference(custom_type.clone(), *span, ReferenceKind::Type);
            }
            ExpressionKind::Type(DataType {
                kind: DataTypeKind::List(list_type),
                ..
            }) => {
                if let DataTypeKind::Custom(custom_type) = &list_type.kind {
                    self.add_reference(custom_type.clone(), list_type.span, ReferenceKind::Type);
                }
            }
            ExpressionKind::Reference { template, field } => {
                self.add_reference(
                    template.clone(),
                    expression.span,
                    ReferenceKind::TemplateGenerate,
                );

                let is_resolved = self.resolve_field(template, field);
                if !is_resolved {
                    self.errors.push(SemanticError::UnknownIdentifier {
                        span: expression.span,
                        name: field.clone(),
                    });
                }
                self.references
                    .entry(field.clone())
                    .or_default()
                    .push(Reference {
                        name: field.clone(),
                        span: expression.span,
                        kind: ReferenceKind::Field,
                        is_resolved,
                    });
            }
            _ => {}
        }

        walk_expression(self, expression);
    }
}
