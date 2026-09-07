use crate::module::{
    Module,
    node::{
        Attribute, Constraint, ConstraintKind, Directive, Expr, Field, GlobalItemId, Item,
        ModuleId, PatternPart, Type,
    },
};

pub fn patch_module(module: &mut Module, self_id: ModuleId, import_ids: &[ModuleId]) {
    for directive in &mut module.directives {
        match directive {
            Directive::Output { options, .. } => {
                for (_, expr) in options {
                    patch_expr(expr, self_id, import_ids);
                }
            }
            Directive::Generate { template, count } => {
                patch_global_id(template, self_id, import_ids);
                patch_expr(count, self_id, import_ids);
            }
            Directive::OutputPath(_) | Directive::Import(_) => {}
        }
    }

    for item in &mut module.items {
        match item {
            Item::Template(t) => {
                if let Some(parent) = &mut t.parent {
                    patch_global_id(parent, self_id, import_ids);
                }
                for field in &mut t.fields {
                    patch_field(field, self_id, import_ids);
                }
                patch_attributes(&mut t.attributes, self_id, import_ids);
            }
            Item::Struct(s) => {
                for field in &mut s.fields {
                    patch_field(field, self_id, import_ids);
                }
            }
            Item::Enum(e) => {
                for variant in &mut e.variants {
                    if let Some(weight) = &mut variant.weight {
                        patch_expr(weight, self_id, import_ids);
                    }
                }
                patch_attributes(&mut e.attributes, self_id, import_ids);
            }
            Item::TypeAlias(ta) => {
                patch_type(&mut ta.target_type, self_id, import_ids);
                for c in &mut ta.constraints {
                    patch_constraint(c, self_id, import_ids);
                }
                patch_attributes(&mut ta.attributes, self_id, import_ids);
                if let Some(expr) = &mut ta.expr {
                    patch_expr(expr, self_id, import_ids);
                }
            }
        }
    }

    module.metadata.id = self_id;
}

fn patch_global_id(id: &mut GlobalItemId, self_id: ModuleId, import_ids: &[ModuleId]) {
    id.module = match id.module.0 {
        0 => self_id,
        n => import_ids
            .get((n - 1) as usize)
            .copied()
            .unwrap_or_default(),
    };
}

fn patch_type(ty: &mut Type, self_id: ModuleId, import_ids: &[ModuleId]) {
    match ty {
        Type::Optional(inner) | Type::List(inner) => patch_type(inner, self_id, import_ids),
        Type::UserDefined(id) => patch_global_id(id, self_id, import_ids),
        Type::Int | Type::String | Type::Bool | Type::Float => {}
    }
}

fn patch_expr(expr: &mut Expr, self_id: ModuleId, import_ids: &[ModuleId]) {
    match expr {
        Expr::Type(ty) => patch_type(ty, self_id, import_ids),
        Expr::Identifier(id) => patch_global_id(id, self_id, import_ids),
        Expr::Reference { template, .. } => patch_global_id(template, self_id, import_ids),
        Expr::Infix { left, right, .. } => {
            patch_expr(left, self_id, import_ids);
            patch_expr(right, self_id, import_ids);
        }
        Expr::Prefix { expr: inner, .. } => patch_expr(inner, self_id, import_ids),
        Expr::List(items) => {
            for item in items {
                patch_expr(item, self_id, import_ids);
            }
        }
        Expr::Range { start, end, .. } => {
            patch_expr(start, self_id, import_ids);
            patch_expr(end, self_id, import_ids);
        }
        Expr::ConstrainedType { ty, constraints } => {
            patch_type(ty, self_id, import_ids);
            for c in constraints {
                patch_constraint(c, self_id, import_ids);
            }
        }
        Expr::StringPattern(parts) => {
            for part in parts {
                match part {
                    PatternPart::RepeatChar {
                        count_expr: Some(e),
                        ..
                    } => patch_expr(e, self_id, import_ids),
                    PatternPart::RepeatGroup { count, .. } => {
                        patch_expr(count, self_id, import_ids)
                    }
                    _ => {}
                }
            }
        }
        Expr::Int(_) | Expr::Float(_) | Expr::String(_) | Expr::Bool(_) => {}
    }
}

fn patch_constraint(constraint: &mut Constraint, self_id: ModuleId, import_ids: &[ModuleId]) {
    if let ConstraintKind::Range { min, max } = &mut constraint.kind {
        patch_expr(min, self_id, import_ids);
        patch_expr(max, self_id, import_ids);
    }
    patch_expr(&mut constraint.value, self_id, import_ids);
}

fn patch_attributes(attributes: &mut [Attribute], self_id: ModuleId, import_ids: &[ModuleId]) {
    for attr in attributes {
        for arg in &mut attr.args {
            patch_expr(arg, self_id, import_ids);
        }
    }
}

fn patch_field(field: &mut Field, self_id: ModuleId, import_ids: &[ModuleId]) {
    patch_type(&mut field.ty, self_id, import_ids);
    patch_expr(&mut field.value, self_id, import_ids);
    patch_attributes(&mut field.attributes, self_id, import_ids);
}
