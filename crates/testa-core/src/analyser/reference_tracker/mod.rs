use crate::analyser::error::SemanticError;
use crate::analyser::symbol_table::SymbolTable;
use crate::ast::visitor::*;
use crate::ast::*;
use crate::utils::Span;
use std::collections::HashMap;

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
    Enum,
    Field,
}

#[derive(Debug, Clone)]
pub struct ReferenceTracker<'a> {
    references: HashMap<String, Vec<Reference>>,
    symbol_table: &'a SymbolTable,
    errors: Vec<SemanticError>,
}

impl<'a> ReferenceTracker<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            references: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn track_references(
        mut self,
        program: &Program,
    ) -> Result<HashMap<String, Vec<Reference>>, Vec<SemanticError>> {
        self.visit_program(program);

        if self.errors.is_empty() {
            Ok(self.references)
        } else {
            Err(self.errors)
        }
    }

    fn add_reference(&mut self, name: String, span: Span, kind: ReferenceKind) {
        let is_resolved = self.symbol_table.lookup(&name).is_some();

        if !is_resolved {
            self.errors.push(SemanticError::UnknownIdentifier {
                span,
                name: name.clone(),
            });
        }

        let reference = Reference {
            name: name.clone(),
            span,
            kind,
            is_resolved,
        };

        self.references.entry(name).or_default().push(reference);
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
                    self.add_reference(parent.to_string(), *parent_span, ReferenceKind::TemplateParent);
                }
            }

            Statement::Generate {
                template_name: Some(name),
                template_name_span: Some(span),
                ..
            } => {
                self.add_reference(name.clone(), *span, ReferenceKind::TemplateGenerate);
            }

            Statement::TypeDecl { name, name_span: Some(name_span),  ..} => {
                self.add_reference(name.clone(), *name_span, ReferenceKind::Type);
            }

            _ => {}
        }

        walk_statement(self, stmt);
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Identifier(name) => self.add_reference(name.clone(), expression.span, ReferenceKind::Type),
            ExpressionKind::Type(DataType {
                kind: DataTypeKind::Custom(custom_type),
                span,
                ..
            }) => {
                self.add_reference(custom_type.clone(), *span, ReferenceKind::Type);
            }
            _ => {}
        }
        

        walk_expression(self, expression);
    }
}
