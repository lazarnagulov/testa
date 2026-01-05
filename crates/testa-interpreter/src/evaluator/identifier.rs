use rand::Rng;
use testa_core::{
    analyser::symbol_table::symbol::SymbolKind,
    ast::{Attribute, DataType, DataTypeKind, Expression, ExpressionKind},
    utils::Span,
};

use crate::{
    evaluator::{
        constrained_type::evaluate_constrained_type,
        context::{Context, State},
        enumeration::evaluate_enum,
        error::EvalError,
    },
    object::Object,
    util::generate_random_string,
};

pub(crate) fn evaluate_data_type(
    ctx: &Context,
    state: &mut State,
    data_type: &DataType,
) -> Result<Object, EvalError> {
    if let Some(constraints) = &data_type.constraints {
        return evaluate_constrained_type(ctx, state, &data_type.kind, constraints, data_type.span);
    }
    match &data_type.kind {
        DataTypeKind::Int => Ok(Object::new(state.rng.random::<i32>() as isize)),
        DataTypeKind::Float => Ok(Object::new(state.rng.random::<f32>())),
        DataTypeKind::Boolean => Ok(Object::new(state.rng.random_bool(0.5))),
        DataTypeKind::Str => {
            let size = state.rng.random_range(6..=20);
            Ok(Object::new(generate_random_string(&mut state.rng, size)))
        }
        DataTypeKind::List(inner) => evaluate_list(ctx, state, inner),
        DataTypeKind::Custom(name) => evaluate_identifier(ctx, state, name, data_type.span),
    }
}

pub(crate) fn evaluate_list(
    ctx: &Context,
    state: &mut State,
    data_type: &DataType,
) -> Result<Object, EvalError> {
    let count = state.rng.random_range(0..=16);
    let values = (0..count)
        .map(|_| evaluate_data_type(ctx, state, data_type))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Object::new(values))
}

pub(crate) fn evaluate_identifier(
    ctx: &Context,
    state: &mut State,
    name: &str,
    span: Span,
) -> Result<Object, EvalError> {
    let symbol = ctx
        .symbol_table
        .lookup(name)
        .ok_or_else(|| EvalError::NotDefined(name.to_string(), span))?;

    match &symbol.kind {
        SymbolKind::Enum { variants, .. } => evaluate_enum(ctx, state, variants),
        SymbolKind::TypeAlias {
            name,
            data_type,
            attributes,
        } => evaluate_type_alias(ctx, state, data_type, name, attributes),
        _ => Err(EvalError::NotDefined(name.to_string(), span)),
    }
}

fn evaluate_type_alias(
    ctx: &Context,
    state: &mut State,
    data_type: &Expression,
    _name: &str,
    _attributes: &[Attribute],
) -> Result<Object, EvalError> {
    let ExpressionKind::Type(data_type) = &data_type.kind else {
        return Err(EvalError::MiscellaneousError(
            "Expected DataType".to_string(),
            data_type.span,
        ));
    };
    evaluate_data_type(ctx, state, data_type)
}
