use crate::{
    ast::{
        Attribute, ConstraintExpression, Expression, ExpressionKind, Field, PatternElement,
        Program, Statement, Variant,
    },
    utils::Span,
};

pub trait Visitor: Sized {
    fn visit_program(&mut self, program: &Program) {
        walk_program(self, program);
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        walk_statement(self, stmt);
    }

    fn visit_template(
        &mut self,
        _parent: &Option<String>,
        _attributes: &[Attribute],
        _name: &str,
        _body: &[Field],
        _span: Span,
    ) {
        // Default: do nothing
    }

    fn visit_enum(
        &mut self,
        _name: &str,
        _variants: &[Variant],
        _attributes: &[Attribute],
        _span: Span,
    ) {
        // Default: do nothing
    }

    fn visit_type_decl(
        &mut self,
        _name: &str,
        _data_type: &Expression,
        _attributes: &[Attribute],
        _span: Span,
    ) {
        // Default: do nothing
    }

    fn visit_resource(&mut self, _name: &str, _body: &[Field], _span: Span) {
        // Default: do nothing
    }

    fn visit_generate(
        &mut self,
        _template_name: &Option<String>,
        _body: &[Field],
        _count: &Expression,
        _span: Span,
    ) {
        // Default: do nothing
    }

    fn visit_constraint_decl(
        &mut self,
        _name: &str,
        _constraint: &ConstraintExpression,
        _span: Span,
    ) {
        // Default: do nothing
    }

    fn visit_output_directive(&mut self, _argument: &str, _options: &[Field], _span: Span) {
        // Default: do nothing
    }

    fn visit_output_path_directive(&mut self, _path: &std::path::Path, _span: Span) {
        // Default: do nothing
    }

    fn visit_expression(&mut self, expr: &Expression) {
        walk_expression(self, expr);
    }

    fn visit_field(&mut self, field: &Field) {
        walk_field(self, field);
    }

    fn visit_attribute(&mut self, _attribute: &Attribute) {
        // Default: do nothing
    }
}

pub fn walk_program<V: Visitor>(visitor: &mut V, program: &Program) {
    for stmt in &program.0 {
        visitor.visit_statement(stmt);
    }
}

pub fn walk_statement<V: Visitor>(visitor: &mut V, stmt: &Statement) {
    match stmt {
        Statement::Template {
            parent,
            attributes,
            name,
            body,
            span,
        } => {
            visitor.visit_template(parent, attributes, name, body, *span);
            for attr in attributes {
                visitor.visit_attribute(attr);
            }
            for field in body {
                visitor.visit_field(field);
            }
        }
        Statement::Enum {
            name,
            variants,
            attributes,
            span,
        } => {
            visitor.visit_enum(name, variants, attributes, *span);
            for attr in attributes {
                visitor.visit_attribute(attr);
            }
            for variant in variants {
                if let Some(weight) = &variant.weight {
                    visitor.visit_expression(weight);
                }
            }
        }
        Statement::TypeDecl {
            name,
            data_type,
            attributes,
            span,
        } => {
            visitor.visit_type_decl(name, data_type, attributes, *span);
            for attr in attributes {
                visitor.visit_attribute(attr);
            }
            visitor.visit_expression(data_type);
        }
        Statement::Resource { name, body, span } => {
            visitor.visit_resource(name, body, *span);
            for field in body {
                visitor.visit_field(field);
            }
        }
        Statement::Generate {
            template_name,
            body,
            count,
            span,
        } => {
            visitor.visit_generate(template_name, body, count, *span);
            for field in body {
                visitor.visit_field(field);
            }
            visitor.visit_expression(count);
        }
        Statement::ConstraintDecl {
            name,
            constraint,
            span,
        } => {
            visitor.visit_constraint_decl(name, constraint, *span);
            visitor.visit_expression(&constraint.expression);
        }
        Statement::OutputDirective {
            argument,
            options,
            span,
        } => {
            visitor.visit_output_directive(argument, options, *span);
            for field in options {
                visitor.visit_field(field);
            }
        }
        Statement::OutputPathDirective { argument, span } => {
            visitor.visit_output_path_directive(argument, *span);
        }
        Statement::Expression(expr_stmt) => {
            visitor.visit_expression(&expr_stmt.expression);
        }
    }
}

pub fn walk_expression<V: Visitor>(visitor: &mut V, expression: &Expression) {
    match &expression.kind {
        ExpressionKind::StringPattern(pattern_elements) => {
            for element in pattern_elements {
                match element {
                    PatternElement::RepeatChar {
                        count_expression: Some(count_expression),
                        ..
                    } => {
                        visitor.visit_expression(count_expression);
                    }
                    PatternElement::RepeatGroup { count, .. } => {
                        visitor.visit_expression(count);
                    }
                    _ => {}
                }
            }
        }
        ExpressionKind::List(elements) => {
            for element in elements {
                visitor.visit_expression(&element.value);
                if let Some(weight) = &element.weight {
                    visitor.visit_expression(weight);
                }
            }
        }
        ExpressionKind::Type(data_type) => {
            if let Some(constraints) = &data_type.constraints {
                for constraint in constraints {
                    visitor.visit_expression(&constraint.expression);
                }
            }
        }
        ExpressionKind::Prefix { expression, .. } => {
            visitor.visit_expression(expression);
        }
        ExpressionKind::Infix { left, right, .. } => {
            visitor.visit_expression(left);
            visitor.visit_expression(right);
        }
        ExpressionKind::FuncCall { arguments } => {
            for arg in arguments {
                visitor.visit_expression(arg);
            }
        }
        ExpressionKind::IntLiteral(_)
        | ExpressionKind::FloatLiteral(_)
        | ExpressionKind::StringLiteral(_)
        | ExpressionKind::BooleanLiteral(_)
        | ExpressionKind::Identifier(_) => {
            // No nested expressions
        }
    }
}

pub fn walk_field<V: Visitor>(visitor: &mut V, field: &Field) {
    for attr in &field.attributes {
        visitor.visit_attribute(attr);
    }
    visitor.visit_expression(&field.value);
}
