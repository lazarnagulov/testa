use crate::{
    analyser::{error::SemanticError, symbol_table::SymbolTable, type_checker::types::Type},
    ast::{
        Attribute, Expression, ExpressionKind, Field, Program, Variant,
        visitor::{Visitor, walk_expression, walk_field},
    },
    utils::Span,
};

pub mod types;

mod constraint;
mod inference;
mod infix;
mod prefix;

pub struct TypeChecker<'a> {
    symbol_table: &'a SymbolTable,
    errors: Vec<SemanticError>,
    // TODO: Add caching later
    // type_cache: HashMap<String, Type>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            errors: Vec::new(),
            // type_cache: HashMap::new(),
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
}

impl<'a> Visitor for TypeChecker<'a> {
    fn visit_expression(&mut self, expression: &Expression) {
        self.infer_type(expression);
        walk_expression(self, expression);
    }

    fn visit_field(&mut self, field: &Field) {
        self.infer_type(&field.value);
        walk_field(self, field);
    }

    fn visit_enum(
        &mut self,
        _name: &str,
        variants: &[Variant],
        _attributes: &[Attribute],
        _span: Span,
    ) {
        for variant in variants {
            self.visit_variant(variant);
        }
    }

    fn visit_type_decl(
        &mut self,
        _name: &str,
        data_type: &Expression,
        _attributes: &[Attribute],
        _span: Span,
    ) {
        if let ExpressionKind::Type(dt) = &data_type.kind {
            let base_type = Type::from(dt);
            self.check_constraints(dt, &base_type);
        }
    }

    fn visit_variant(&mut self, variant: &Variant) {
        if let Some(weight) = &variant.weight {
            let weight_type = self.infer_type(weight);
            if !matches!(weight_type, Type::Int) && !weight_type.is_unknown() {
                self.errors.push(SemanticError::TypeMismatch {
                    expected: "int".to_string(),
                    found: weight_type.display(),
                    span: variant.span,
                });
            }
        }
    }

    fn visit_generate(
        &mut self,
        _template_name: &Option<String>,
        body: &[Field],
        count: &Expression,
        _span: Span,
    ) {
        let count_type = self.infer_type(count);
        if !matches!(count_type, Type::Int) && !count_type.is_unknown() {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "int".to_string(),
                found: count_type.display(),
                span: count.span,
            });
        }

        for field in body {
            walk_field(self, field);
        }
    }

    fn visit_template(
        &mut self,
        _parent: &Option<String>,
        _attributes: &[Attribute],
        _name: &str,
        body: &[Field],
        _span: Span,
    ) {
        for field in body {
            walk_field(self, field);
        }
    }
}
