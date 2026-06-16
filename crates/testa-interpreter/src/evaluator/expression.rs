use crate::{
    evaluator::{
        constrained_type::evaluate_constrained_type,
        context::{Context, State},
        error::EvalError,
        identifier::{evaluate_hir_type, evaluate_item_ref},
        pattern::evaluate_string_pattern,
    },
    object::Object,
};
use testa_hir::module::{Expr, InfixOp, PrefixOp};

pub fn evaluate_expression(
    ctx: &Context,
    state: &mut State,
    expr: &Expr,
) -> Result<Object, EvalError> {
    match expr {
        Expr::Int(n) => Ok(Object::new(*n as isize)),
        Expr::Float(f) => Ok(Object::new(*f as f32)),
        Expr::String(id) => Ok(Object::new(ctx.resolve_local_string(*id).to_string())),
        Expr::Bool(b) => Ok(Object::new(*b)),

        Expr::Type(ty) => evaluate_hir_type(ctx, state, ty),
        Expr::Identifier(item_ref) => evaluate_item_ref(ctx, state, item_ref),
        Expr::StringPattern(parts) => evaluate_string_pattern(ctx, state, parts),
        Expr::ConstrainedType { ty, constraints } => {
            evaluate_constrained_type(ctx, state, ty, constraints)
        }
        Expr::List(elements) => {
            let values = elements
                .iter()
                .map(|e| evaluate_expression(ctx, state, e))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Object::new(values))
        }
        Expr::Range {
            start,
            end,
            inclusive,
        } => {
            let start = evaluate_expression(ctx, state, start)?;
            let end = evaluate_expression(ctx, state, end)?;
            match (start, end) {
                (Object::Int(s), Object::Int(e)) => {
                    let end = if *inclusive { e } else { e - 1 };
                    Ok(Object::new((s, end)))
                }
                _ => Err(EvalError::type_mismatch(
                    "int range".to_string(),
                    "other".to_string(),
                )),
            }
        }
        Expr::Infix { left, op, right } => {
            let left = evaluate_expression(ctx, state, left)?;
            let right = evaluate_expression(ctx, state, right)?;

            if matches!(op, InfixOp::Div) && matches!(right, Object::Int(0) | Object::Float(0.0)) {
                return Err(EvalError::DivisionByZero {
                    span: Default::default(),
                });
            }
            evaluate_infix_expression(&left, op, &right)
        }
        Expr::Prefix { op, expr } => {
            let val = evaluate_expression(ctx, state, expr)?;
            evaluate_prefix_expression(op, &val)
        }
        Expr::Reference { template, field } => state
            .pool
            .sample(&mut state.rng, template, *field)
            .cloned()
            .ok_or_else(|| {
                let template_name = ctx
                    .resolve_item(template)
                    .map(|item| {
                        ctx.module_for(template)
                            .string_pool
                            .resolve(item.name())
                            .to_string()
                    })
                    .unwrap_or_else(|| template.to_string());

                let field_name = ctx
                    .resolve_template(template)
                    .and_then(|t| t.get_field(*field))
                    .map(|f| {
                        ctx.module_for(template)
                            .string_pool
                            .resolve(f.name)
                            .to_string()
                    })
                    .unwrap_or_else(|| field.to_string());

                EvalError::EmptyPool {
                    template: template_name,
                    field: field_name,
                }
            }),
    }
}

fn evaluate_prefix_expression(op: &PrefixOp, val: &Object) -> Result<Object, EvalError> {
    match op {
        PrefixOp::Neg => match val {
            Object::Int(v) => Ok(Object::new(-v)),
            Object::Float(v) => Ok(Object::new(-v)),
            _ => Err(EvalError::type_mismatch(
                "numeric".to_string(),
                format!("{}", val),
            )),
        },
        PrefixOp::Not => match val {
            Object::Boolean(v) => Ok(Object::new(!v)),
            _ => Err(EvalError::type_mismatch(
                "bool".to_string(),
                format!("{}", val),
            )),
        },
        PrefixOp::BitNeg => match val {
            Object::Int(v) => Ok(Object::new(!v)),
            _ => Err(EvalError::type_mismatch(
                "int".to_string(),
                format!("{}", val),
            )),
        },
    }
}

fn evaluate_infix_expression(
    left: &Object,
    op: &InfixOp,
    right: &Object,
) -> Result<Object, EvalError> {
    match (left, right) {
        (Object::Int(l), Object::Int(r)) => match op {
            InfixOp::Add => Ok(Object::new(l + r)),
            InfixOp::Sub => Ok(Object::new(l - r)),
            InfixOp::Mul => Ok(Object::new(l * r)),
            InfixOp::Div => Ok(Object::new(l / r)),
            InfixOp::Mod => Ok(Object::new(l % r)),
            InfixOp::BitAnd => Ok(Object::new(l & r)),
            InfixOp::BitOr => Ok(Object::new(l | r)),
            InfixOp::BitXor => Ok(Object::new(l ^ r)),
            InfixOp::BitLShift => Ok(Object::new(l << r)),
            InfixOp::BitRShift => Ok(Object::new(l >> r)),
            InfixOp::Equal => Ok(Object::new(l == r)),
            InfixOp::NotEqual => Ok(Object::new(l != r)),
            InfixOp::LessThen => Ok(Object::new(l < r)),
            InfixOp::LessThanOrEqual => Ok(Object::new(l <= r)),
            InfixOp::GreaterThan => Ok(Object::new(l > r)),
            InfixOp::GreaterThanOrEqual => Ok(Object::new(l >= r)),
            op => Err(EvalError::unsupported_infix_operator(*l, *op, *r)),
        },
        (Object::Float(l), Object::Float(r)) => match op {
            InfixOp::Add => Ok(Object::new(l + r)),
            InfixOp::Sub => Ok(Object::new(l - r)),
            InfixOp::Mul => Ok(Object::new(l * r)),
            InfixOp::Div => Ok(Object::new(l / r)),
            InfixOp::Mod => Ok(Object::new(l % r)),
            InfixOp::Equal => Ok(Object::new(l == r)),
            InfixOp::NotEqual => Ok(Object::new(l != r)),
            InfixOp::LessThen => Ok(Object::new(l < r)),
            InfixOp::LessThanOrEqual => Ok(Object::new(l <= r)),
            InfixOp::GreaterThan => Ok(Object::new(l > r)),
            InfixOp::GreaterThanOrEqual => Ok(Object::new(l >= r)),
            op => Err(EvalError::unsupported_infix_operator(*l, *op, *r)),
        },
        (Object::String(l), Object::String(r)) => match op {
            InfixOp::Add => Ok(Object::new(format!("{}{}", l, r))),
            InfixOp::Equal => Ok(Object::new(l == r)),
            InfixOp::NotEqual => Ok(Object::new(l != r)),
            _ => Err(EvalError::type_mismatch(
                "string + string".to_string(),
                format!("{:?}", op),
            )),
        },
        _ => Err(EvalError::type_mismatch(
            format!("{}", left),
            format!("{}", right),
        )),
    }
}
