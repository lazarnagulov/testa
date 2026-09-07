use rand::Rng;
use testa_hir::module::node::{Constraint, ConstraintKind, Item, Type};

use crate::{
    evaluator::{
        context::{Context, State},
        error::EvalError,
        expression::evaluate_expression,
        identifier::evaluate_hir_type,
    },
    object::Object,
    util::generate_random_string,
};

pub(crate) fn evaluate_constrained_type(
    ctx: &Context,
    state: &mut State,
    ty: &Type,
    constraints: &[Constraint],
) -> Result<Object, EvalError> {
    match ty {
        Type::Int => evaluate_int_with_constraints(ctx, state, constraints),
        Type::String => evaluate_string_with_constraints(ctx, state, constraints),
        Type::Float => evaluate_float_with_constraints(ctx, state, constraints),
        Type::Bool => evaluate_boolean_with_constraints(ctx, state, constraints),
        Type::List(inner) => evaluate_list_with_constraints(ctx, state, inner, constraints),
        Type::Optional(inner) => {
            if state.rng.random_bool(0.5) {
                evaluate_constrained_type(ctx, state, inner, constraints)
            } else {
                Ok(Object::NoReturn)
            }
        }
        Type::UserDefined(item_ref) => match ctx.resolve_item(item_ref) {
            Some(Item::TypeAlias(t)) => {
                let merged = merge_constraints(&t.constraints, constraints);
                evaluate_constrained_type(ctx, state, &t.target_type, &merged)
            }
            _ => Err(EvalError::type_mismatch(
                "type alias".to_string(),
                "other".to_string(),
            )),
        },
    }
}

fn evaluate_boolean_with_constraints(
    ctx: &Context,
    state: &mut State,
    constraints: &[Constraint],
) -> Result<Object, EvalError> {
    let mut bias: Option<f64> = None;

    for constraint in constraints {
        match &constraint.kind {
            ConstraintKind::Bias => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Float(b) if (0.0..=1.0).contains(&b) => bias = Some(b as f64),
                Object::Int(b) if (0..=1).contains(&b) => bias = Some(b as f64),
                other => {
                    return Err(EvalError::MiscellaneousError(
                        format!("Bias must be between 0.0 and 1.0, got {}", other),
                        Default::default(),
                    ));
                }
            },
            _ => {
                return Err(EvalError::incompatible_constraint(
                    "bool",
                    &format!("{:?}", constraint.kind),
                ));
            }
        }
    }

    Ok(Object::Boolean(state.rng.random_bool(bias.unwrap_or(0.5))))
}

fn evaluate_list_with_constraints(
    ctx: &Context,
    state: &mut State,
    inner: &Type,
    constraints: &[Constraint],
) -> Result<Object, EvalError> {
    let mut min_count: Option<usize> = None;
    let mut max_count: Option<usize> = None;

    for constraint in constraints {
        match &constraint.kind {
            ConstraintKind::Range { min, max } => {
                match (
                    evaluate_expression(ctx, state, min)?,
                    evaluate_expression(ctx, state, max)?,
                ) {
                    (Object::Int(mn), Object::Int(mx)) => {
                        min_count = Some(mn.max(0) as usize);
                        max_count = Some(mx.max(0) as usize);
                    }
                    _ => {
                        return Err(EvalError::type_mismatch(
                            "int range".to_string(),
                            "other".to_string(),
                        ));
                    }
                }
            }
            ConstraintKind::Min => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) if m >= 0 => min_count = Some(m as usize),
                other => {
                    return Err(EvalError::type_mismatch(
                        "non-negative integer".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            ConstraintKind::Max => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) if m >= 0 => max_count = Some(m as usize),
                other => {
                    return Err(EvalError::type_mismatch(
                        "non-negative integer".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            _ => {
                return Err(EvalError::incompatible_constraint(
                    "list",
                    &format!("{:?}", constraint.kind),
                ));
            }
        }
    }

    let min = min_count.unwrap_or(0);
    let max = max_count.unwrap_or(16);
    if min > max {
        return Err(EvalError::MiscellaneousError(
            format!("Invalid list count range: min ({}) > max ({})", min, max),
            Default::default(),
        ));
    }

    let count = state.rng.random_range(min..=max);

    let items = (0..count)
        .map(|_| match inner {
            Type::UserDefined(item_ref) => match ctx.resolve_item(item_ref) {
                Some(Item::TypeAlias(t)) if !t.constraints.is_empty() => {
                    let constraints = t.constraints.clone();
                    let ty = t.target_type.clone();
                    evaluate_constrained_type(ctx, state, &ty, &constraints)
                }
                _ => evaluate_hir_type(ctx, state, inner),
            },
            _ => evaluate_hir_type(ctx, state, inner),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Object::List(items))
}

fn evaluate_float_with_constraints(
    ctx: &Context,
    state: &mut State,
    constraints: &[Constraint],
) -> Result<Object, EvalError> {
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;

    for constraint in constraints {
        match &constraint.kind {
            ConstraintKind::Range { min: mn, max: mx } => {
                let min_val = evaluate_expression(ctx, state, mn)?;
                let max_val = evaluate_expression(ctx, state, mx)?;
                if let (Object::Int(a), Object::Int(b)) = (min_val, max_val) {
                    min = Some(a as f32);
                    max = Some(b as f32);
                }
            }
            ConstraintKind::Min => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) => min = Some(min.map_or(m as f32, |c| c.max(m as f32))),
                Object::Float(m) => min = Some(min.map_or(m, |c| c.max(m))),
                other => {
                    return Err(EvalError::type_mismatch(
                        "number".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            ConstraintKind::Max => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) => max = Some(max.map_or(m as f32, |c| c.min(m as f32))),
                Object::Float(m) => max = Some(max.map_or(m, |c| c.min(m))),
                other => {
                    return Err(EvalError::type_mismatch(
                        "number".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            _ => {
                return Err(EvalError::incompatible_constraint(
                    "float",
                    &format!("{:?}", constraint.kind),
                ));
            }
        }
    }

    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1000.0);

    if min > max {
        return Err(EvalError::MiscellaneousError(
            format!("Invalid range: min ({}) > max ({})", min, max),
            Default::default(),
        ));
    }

    Ok(Object::Float(state.rng.random_range(min..=max)))
}

fn evaluate_string_with_constraints(
    ctx: &Context,
    state: &mut State,
    constraints: &[Constraint],
) -> Result<Object, EvalError> {
    let mut length: Option<usize> = None;
    let mut min_length: Option<usize> = None;
    let mut max_length: Option<usize> = None;

    for constraint in constraints {
        match &constraint.kind {
            ConstraintKind::Length => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(len) if len > 0 => length = Some(len as usize),
                Object::Int(len) => {
                    return Err(EvalError::MiscellaneousError(
                        format!("String length must be positive, got {}", len),
                        Default::default(),
                    ));
                }
                Object::Range(start, end) => {
                    min_length = Some(start.max(0) as usize);
                    max_length = Some(end.max(0) as usize);
                }
                other => {
                    return Err(EvalError::type_mismatch(
                        "integer or range".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            ConstraintKind::Min => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) if m > 0 => min_length = Some(m as usize),
                other => {
                    return Err(EvalError::type_mismatch(
                        "positive integer".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            ConstraintKind::Max => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) if m > 0 => max_length = Some(m as usize),
                other => {
                    return Err(EvalError::type_mismatch(
                        "positive integer".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            _ => {
                return Err(EvalError::incompatible_constraint(
                    "string",
                    &format!("{:?}", constraint.kind),
                ));
            }
        }
    }

    let final_length = if let Some(len) = length {
        len
    } else {
        let min = min_length.unwrap_or(6);
        let max = max_length.unwrap_or(20);
        if min > max {
            return Err(EvalError::MiscellaneousError(
                format!("Invalid string length range: min ({}) > max ({})", min, max),
                Default::default(),
            ));
        }
        state.rng.random_range(min..=max)
    };

    Ok(Object::String(generate_random_string(
        &mut state.rng,
        final_length,
    )))
}

fn evaluate_int_with_constraints(
    ctx: &Context,
    state: &mut State,
    constraints: &[Constraint],
) -> Result<Object, EvalError> {
    let mut min: Option<isize> = None;
    let mut max: Option<isize> = None;
    let mut multiple_of: Option<isize> = None;

    for constraint in constraints {
        match &constraint.kind {
            ConstraintKind::Range { min: mn, max: mx } => {
                let min_val = evaluate_expression(ctx, state, mn)?;
                let max_val = evaluate_expression(ctx, state, mx)?;
                if let (Object::Int(a), Object::Int(b)) = (min_val, max_val) {
                    min = Some(a);
                    max = Some(b);
                }
            }
            ConstraintKind::Min => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) => min = Some(min.map_or(m, |c| c.max(m))),
                other => {
                    return Err(EvalError::type_mismatch(
                        "integer".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            ConstraintKind::Max => match evaluate_expression(ctx, state, &constraint.value)? {
                Object::Int(m) => max = Some(max.map_or(m, |c| c.min(m))),
                other => {
                    return Err(EvalError::type_mismatch(
                        "integer".to_string(),
                        format!("{}", other),
                    ));
                }
            },
            ConstraintKind::MultipleOf => {
                match evaluate_expression(ctx, state, &constraint.value)? {
                    Object::Int(m) if m > 0 => multiple_of = Some(m),
                    Object::Int(m) => {
                        return Err(EvalError::MiscellaneousError(
                            format!("multiple_of must be positive, got {}", m),
                            Default::default(),
                        ));
                    }
                    other => {
                        return Err(EvalError::type_mismatch(
                            "positive integer".to_string(),
                            format!("{}", other),
                        ));
                    }
                }
            }
            _ => {
                return Err(EvalError::incompatible_constraint(
                    "int",
                    &format!("{:?}", constraint.kind),
                ));
            }
        }
    }

    let min = min.unwrap_or(0) as i64;
    let max = max.unwrap_or(1000) as i64;

    if min > max {
        return Err(EvalError::MiscellaneousError(
            format!("Invalid range: min ({}) > max ({})", min, max),
            Default::default(),
        ));
    }

    let mut value = state.rng.random_range(min..=max);

    if let Some(m) = multiple_of {
        let m = m as i64;
        value = (value / m) * m;
        if value < min {
            value += m;
        }
        if value > max {
            value -= m;
        }
        if value < min || value > max {
            return Err(EvalError::MiscellaneousError(
                format!(
                    "Cannot satisfy constraints: no multiple of {} in range {}..={}",
                    m, min, max
                ),
                Default::default(),
            ));
        }
    }

    Ok(Object::Int(value as isize))
}

fn merge_constraints(base: &[Constraint], usage: &[Constraint]) -> Vec<Constraint> {
    let mut merged: Vec<Constraint> = usage.to_vec();
    let usage_kinds: std::collections::HashSet<std::mem::Discriminant<ConstraintKind>> = usage
        .iter()
        .map(|c| std::mem::discriminant(&c.kind))
        .collect();

    for constraint in base {
        if !usage_kinds.contains(&std::mem::discriminant(&constraint.kind)) {
            merged.push(constraint.clone());
        }
    }

    merged
}
