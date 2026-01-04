use testa_core::{
    ast::{ConstraintExpression, DataType, DataTypeKind},
    utils::Span,
};

use crate::{
    evaluator::{
        context::{Context, State},
        error::EvalError,
    },
    object::Object,
};

pub(crate) fn evaluate_constrained_type(
    ctx: &Context,
    state: &mut State,
    kind: &DataTypeKind,
    constraints: &[ConstraintExpression],
    span: Span,
) -> Result<Object, EvalError> {
    match kind {
        DataTypeKind::Int => evaluate_int_with_constraints(ctx, state, constraints),
        DataTypeKind::Str => evaluate_string_with_constraints(ctx, state, constraints),
        DataTypeKind::Float => evaluate_float_with_constraints(ctx, state, constraints),
        DataTypeKind::Boolean => evaluate_boolean_with_constraints(ctx, state, constraints),
        DataTypeKind::List(inner) => evaluate_list_with_constraints(ctx, state, inner, constraints),
        _ => Err(EvalError::MiscellaneousError(
            "Constraints not supported for this type".to_string(),
            span,
        )),
    }
}

fn evaluate_boolean_with_constraints(
    _ctx: &Context,
    _state: &mut State,
    _constraints: &[ConstraintExpression],
) -> Result<Object, EvalError> {
    todo!()
}

fn evaluate_list_with_constraints(
    _ctx: &Context,
    _state: &mut State,
    _inner: &DataType,
    _constraints: &[ConstraintExpression],
) -> Result<Object, EvalError> {
    todo!()
}

fn evaluate_float_with_constraints(
    _ctx: &Context,
    _state: &mut State,
    _constraints: &[ConstraintExpression],
) -> Result<Object, EvalError> {
    todo!()
}

fn evaluate_string_with_constraints(
    _ctx: &Context,
    _state: &mut State,
    _constraints: &[ConstraintExpression],
) -> Result<Object, EvalError> {
    todo!()
}

fn evaluate_int_with_constraints(
    _ctx: &Context,
    _state: &mut State,
    _constraints: &[ConstraintExpression],
) -> Result<Object, EvalError> {
    todo!()
}
