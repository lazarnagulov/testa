use rand::Rng;
use testa_hir::module::node::{GlobalItemId, Item, Type};

use crate::{
    evaluator::{
        constrained_type::evaluate_constrained_type,
        context::{Context, State},
        enumeration::evaluate_enum,
        error::EvalError,
        expression::evaluate_expression,
    },
    object::Object,
    util::generate_random_string,
};

pub(crate) fn evaluate_hir_type(
    ctx: &Context,
    state: &mut State,
    ty: &Type,
) -> Result<Object, EvalError> {
    match ty {
        Type::Int => Ok(Object::new(state.rng.random::<i32>() as isize)),
        Type::Float => Ok(Object::new(state.rng.random::<f32>())),
        Type::Bool => Ok(Object::new(state.rng.random_bool(0.5))),
        Type::String => {
            let size = state.rng.random_range(6..=20);
            Ok(Object::new(generate_random_string(&mut state.rng, size)))
        }
        Type::List(inner) => {
            let count = state.rng.random_range(0..=16);
            let values = (0..count)
                .map(|_| evaluate_hir_type(ctx, state, inner))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Object::new(values))
        }
        Type::Optional(inner) => {
            if state.rng.random_bool(0.5) {
                evaluate_hir_type(ctx, state, inner)
            } else {
                Ok(Object::NoReturn)
            }
        }
        Type::UserDefined(global_id) => evaluate_global_id(ctx, state, global_id),
    }
}

pub(crate) fn evaluate_global_id(
    ctx: &Context,
    state: &mut State,
    global_id: &GlobalItemId,
) -> Result<Object, EvalError> {
    let item = ctx
        .resolve_item(global_id)
        .ok_or_else(|| EvalError::NotDefined(format!("{}", global_id), Default::default()))?;

    match item {
        Item::Enum(e) => evaluate_enum(ctx, state, e, global_id),
        Item::TypeAlias(t) => {
            if let Some(expr) = &t.expr {
                return evaluate_expression(ctx, state, expr);
            }

            if t.constraints.is_empty() {
                evaluate_hir_type(ctx, state, &t.target_type)
            } else {
                evaluate_constrained_type(ctx, state, &t.target_type, &t.constraints)
            }
        }
        Item::Template(t) => {
            let module = ctx.module_for(global_id);
            let fields = t
                .fields
                .iter()
                .map(|field| {
                    let name = module.string_pool.resolve(field.name).to_string();
                    let value = evaluate_expression(ctx, state, &field.value)?;
                    Ok((name, value))
                })
                .collect::<Result<_, EvalError>>()?;
            Ok(Object::Struct(
                module.string_pool.resolve(t.name).to_string(),
                fields,
            ))
        }
        Item::Struct(s) => {
            let module = ctx.module_for(global_id);
            s.fields
                .iter()
                .map(|field| {
                    let name = module.string_pool.resolve(field.name).to_string();
                    let value = evaluate_expression(ctx, state, &field.value)?;
                    Ok((name, value))
                })
                .collect::<Result<_, EvalError>>()
                .map(|fields| {
                    Object::Struct(module.string_pool.resolve(s.name).to_string(), fields)
                })
        }
    }
}
