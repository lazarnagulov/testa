use rand::Rng;
use testa_core::{
    analyser::symbol_table::symbol::SymbolKind,
    ast::{ConstraintExpression, ConstraintKind, DataType, DataTypeKind, ExpressionKind},
    utils::Span,
};

use crate::{
    evaluator::{
        context::{Context, State},
        error::EvalError,
        expression::evaluate_expression,
        identifier::evaluate_data_type,
    },
    object::Object,
    util::generate_random_string,
};

pub(crate) fn evaluate_constrained_type(
    ctx: &Context,
    state: &mut State,
    kind: &DataTypeKind,
    constraints: &[ConstraintExpression],
    span: Span,
) -> Result<Object, EvalError> {
    match kind {
        DataTypeKind::Int => evaluate_int_with_constraints(ctx, state, constraints, span),
        DataTypeKind::Str => evaluate_string_with_constraints(ctx, state, constraints, span),
        DataTypeKind::Float => evaluate_float_with_constraints(ctx, state, constraints, span),
        DataTypeKind::Boolean => evaluate_boolean_with_constraints(ctx, state, constraints, span),
        DataTypeKind::List(inner) => {
            evaluate_list_with_constraints(ctx, state, inner, constraints, span)
        }
        DataTypeKind::Custom(name) => {
            let symbol = ctx
                .symbol_table
                .lookup(name)
                .ok_or_else(|| EvalError::NotDefined(name.clone(), span))?;

            match &symbol.kind {
                SymbolKind::TypeAlias { data_type, .. } => {
                    let ExpressionKind::Type(base_data_type) = &data_type.kind else {
                        return Err(EvalError::MiscellaneousError(
                            "Expected DataType in type alias".to_string(),
                            data_type.span,
                        ));
                    };

                    let merged_constraints = merge_constraints(
                        base_data_type.constraints.as_deref().unwrap_or(&[]),
                        constraints,
                    );

                    evaluate_constrained_type(
                        ctx,
                        state,
                        &base_data_type.kind,
                        &merged_constraints,
                        span,
                    )
                }
                _ => Err(EvalError::TypeMismatch {
                    expected: "type alias".to_string(),
                    got: "other".to_string(),
                    span,
                }),
            }
        }
    }
}

fn evaluate_boolean_with_constraints(
    ctx: &Context,
    state: &mut State,
    constraints: &[ConstraintExpression],
    span: Span,
) -> Result<Object, EvalError> {
    let mut bias: Option<f64> = None;

    for constraint in constraints {
        match constraint.kind {
            ConstraintKind::Bias => {
                let bias_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match bias_value {
                    Object::Float(b) if (0.0..=1.0).contains(&b) => {
                        bias = Some(b as f64);
                    }
                    Object::Int(b) if (0..=1).contains(&b) => {
                        bias = Some(b as f64);
                    }
                    _ => {
                        return Err(EvalError::MiscellaneousError(
                            format!(
                                "Bias must be a number between 0.0 and 1.0 but got {}",
                                bias_value
                            ),
                            constraint.expression.span,
                        ));
                    }
                }
            }
            _ => {
                return Err(EvalError::UncompatibleConstraint {
                    data_type: "bool".to_string(),
                    constraint: format!("{}", constraint.kind),
                    span,
                });
            }
        }
    }

    let probability = bias.unwrap_or(0.5);
    let value = state.rng.random_bool(probability);
    Ok(Object::Boolean(value))
}

fn evaluate_list_with_constraints(
    ctx: &Context,
    state: &mut State,
    inner_type: &DataType,
    constraints: &[ConstraintExpression],
    span: Span,
) -> Result<Object, EvalError> {
    let mut count: Option<usize> = None;
    let mut min_count: Option<usize> = None;
    let mut max_count: Option<usize> = None;

    for constraint in constraints {
        match constraint.kind {
            ConstraintKind::Count => {
                let count_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match count_value {
                    Object::Int(c) if c >= 0 => {
                        count = Some(c as usize);
                    }
                    Object::Int(c) => {
                        return Err(EvalError::MiscellaneousError(
                            format!("List count must be non-negative, got {}", c),
                            constraint.expression.span,
                        ));
                    }
                    Object::Range(start, end) => {
                        min_count = Some(start.max(0) as usize);
                        max_count = Some(end.max(0) as usize);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "integer or range".to_string(),
                            got: format!("{}", count_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            ConstraintKind::Min => {
                let min_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match min_value {
                    Object::Int(m) if m >= 0 => {
                        min_count = Some(m as usize);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "non-negative integer".to_string(),
                            got: format!("{}", min_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            ConstraintKind::Max => {
                let max_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match max_value {
                    Object::Int(m) if m >= 0 => {
                        max_count = Some(m as usize);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "non-negative integer".to_string(),
                            got: format!("{}", max_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            _ => {
                return Err(EvalError::UncompatibleConstraint {
                    data_type: "list".to_string(),
                    constraint: format!("{:?}", constraint.kind),
                    span: constraint.span,
                });
            }
        }
    }

    // Determine final count
    let final_count = if let Some(c) = count {
        c
    } else {
        let min = min_count.unwrap_or(0);
        let max = max_count.unwrap_or(16);

        if min > max {
            return Err(EvalError::MiscellaneousError(
                format!("Invalid list count range: min ({}) > max ({})", min, max),
                span,
            ));
        }

        state.rng.random_range(min..=max)
    };

    let items = (0..final_count)
        .map(|_| evaluate_data_type(ctx, state, inner_type))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Object::List(items))
}

fn evaluate_float_with_constraints(
    ctx: &Context,
    state: &mut State,
    constraints: &[ConstraintExpression],
    span: Span,
) -> Result<Object, EvalError> {
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;

    for constraint in constraints {
        match constraint.kind {
            ConstraintKind::Range => {
                let range_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match range_value {
                    Object::Range(start, end) => {
                        min = Some(start as f32);
                        max = Some(end as f32);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "range".to_string(),
                            got: format!("{}", range_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            ConstraintKind::Min => {
                let min_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match min_value {
                    Object::Int(m) => {
                        min = Some(min.map_or(m as f32, |current| current.max(m as f32)));
                    }
                    Object::Float(m) => {
                        min = Some(min.map_or(m, |current| current.max(m)));
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "number".to_string(),
                            got: format!("{}", min_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            ConstraintKind::Max => {
                let max_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match max_value {
                    Object::Int(m) => {
                        max = Some(max.map_or(m as f32, |current| current.min(m as f32)));
                    }
                    Object::Float(m) => {
                        max = Some(max.map_or(m, |current| current.min(m)));
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "number".to_string(),
                            got: format!("{}", max_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            _ => {
                return Err(EvalError::UncompatibleConstraint {
                    data_type: "float".to_string(),
                    constraint: format!("{:?}", constraint.kind),
                    span: constraint.span,
                });
            }
        }
    }

    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1000.0);

    if min > max {
        return Err(EvalError::MiscellaneousError(
            format!("Invalid range: min ({}) > max ({})", min, max),
            span,
        ));
    }

    Ok(Object::Float(state.rng.random_range(min..=max)))
}

fn evaluate_string_with_constraints(
    ctx: &Context,
    state: &mut State,
    constraints: &[ConstraintExpression],
    span: Span,
) -> Result<Object, EvalError> {
    let mut length: Option<usize> = None;
    let mut min_length: Option<usize> = None;
    let mut max_length: Option<usize> = None;

    for constraint in constraints {
        match constraint.kind {
            ConstraintKind::Length => {
                let length_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match length_value {
                    Object::Int(len) if len > 0 => {
                        length = Some(len as usize);
                    }
                    Object::Int(len) => {
                        return Err(EvalError::MiscellaneousError(
                            format!("String length must be positive, got {}", len),
                            constraint.span,
                        ));
                    }
                    Object::Range(start, end) => {
                        min_length = Some(start.max(0) as usize);
                        max_length = Some(end.max(0) as usize);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "integer or range".to_string(),
                            got: format!("{}", length_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            ConstraintKind::Min => {
                let min_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match min_value {
                    Object::Int(m) if m > 0 => {
                        min_length = Some(m as usize);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "positive integer".to_string(),
                            got: format!("{}", min_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            ConstraintKind::Max => {
                let max_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match max_value {
                    Object::Int(m) if m > 0 => {
                        max_length = Some(m as usize);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "positive integer".to_string(),
                            got: format!("{}", max_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            _ => {
                return Err(EvalError::UncompatibleConstraint {
                    data_type: "string".to_string(),
                    constraint: format!("{:?}", constraint.kind),
                    span: constraint.span,
                });
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
                span,
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
    constraints: &[ConstraintExpression],
    span: Span,
) -> Result<Object, EvalError> {
    let mut min: Option<isize> = None;
    let mut max: Option<isize> = None;
    let mut multiple_of: Option<isize> = None;

    for constraint in constraints {
        match constraint.kind {
            ConstraintKind::Range => {
                let range_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match range_value {
                    Object::Range(start, end) => {
                        min = Some(start);
                        max = Some(end);
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "range".to_string(),
                            got: format!("{}", range_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }
            ConstraintKind::Min => {
                let min_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match min_value {
                    Object::Int(m) => {
                        min = Some(min.map_or(m, |current| current.max(m)));
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "integer".to_string(),
                            got: format!("{}", min_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }
            ConstraintKind::Max => {
                let max_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match max_value {
                    Object::Int(m) => {
                        max = Some(max.map_or(m, |current| current.min(m)));
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "integer".to_string(),
                            got: format!("{}", max_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }
            ConstraintKind::MultipleOf => {
                let multiple_value = evaluate_expression(ctx, state, &constraint.expression)?;
                match multiple_value {
                    Object::Int(m) if m > 0 => {
                        multiple_of = Some(m);
                    }
                    Object::Int(m) => {
                        return Err(EvalError::MiscellaneousError(
                            format!("multiple_of must be positive, got {}", m),
                            constraint.expression.span,
                        ));
                    }
                    _ => {
                        return Err(EvalError::TypeMismatch {
                            expected: "positive integer".to_string(),
                            got: format!("{}", multiple_value),
                            span: constraint.expression.span,
                        });
                    }
                }
            }

            _ => {
                return Err(EvalError::UncompatibleConstraint {
                    data_type: "int".to_string(),
                    constraint: format!("{:?}", constraint.kind),
                    span: constraint.span,
                });
            }
        }
    }

    let min = min.unwrap_or(0) as i64;
    let max = max.unwrap_or(1000) as i64;

    if min > max {
        return Err(EvalError::MiscellaneousError(
            format!("Invalid range: min ({}) > max ({})", min, max),
            span,
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
                span,
            ));
        }
    }

    Ok(Object::Int(value as isize))
}

// TODO: this is temporary, make it smarter
fn merge_constraints(
    base_constraints: &[ConstraintExpression],
    usage_constraints: &[ConstraintExpression],
) -> Vec<ConstraintExpression> {
    let mut merged = Vec::new();
    let mut seen_kinds = std::collections::HashSet::new();

    for constraint in usage_constraints {
        seen_kinds.insert(constraint.kind.clone());
        merged.push(constraint.clone());
    }

    for constraint in base_constraints {
        if !seen_kinds.contains(&constraint.kind) {
            merged.push(constraint.clone());
        }
    }

    merged
}
