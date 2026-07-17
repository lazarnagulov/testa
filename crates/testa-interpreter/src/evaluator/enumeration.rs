use rand::Rng;
use testa_hir::module::node::{Enum, GlobalItemId};

use crate::{
    evaluator::{
        context::{Context, State},
        error::EvalError,
        expression::evaluate_expression,
    },
    object::Object,
};

pub(crate) fn evaluate_enum(
    ctx: &Context,
    state: &mut State,
    enumeration: &Enum,
    global_id: &GlobalItemId,
) -> Result<Object, EvalError> {
    let module = ctx.module_for(global_id);
    let mut total_weight = 0.0;
    let mut cumulative_weights = Vec::with_capacity(enumeration.variants.len());

    for variant in &enumeration.variants {
        let weight = if let Some(weight_expr) = &variant.weight {
            match evaluate_expression(ctx, state, weight_expr)? {
                Object::Int(w) if w > 0 => w as f64,
                Object::Int(w) => {
                    return Err(EvalError::InvalidWeight {
                        variant: module.string_pool.resolve(variant.name).to_string(),
                        value: w,
                        span: Default::default(),
                    });
                }
                _ => {
                    return Err(EvalError::type_mismatch(
                        "positive integer".to_string(),
                        "other".to_string(),
                    ));
                }
            }
        } else {
            1.0
        };

        total_weight += weight;
        cumulative_weights.push(total_weight);
    }

    let rand_val = state.rng.random::<f64>() * total_weight;
    let index = cumulative_weights
        .binary_search_by(|&w| w.partial_cmp(&rand_val).unwrap())
        .unwrap_or_else(|i| i);

    Ok(Object::String(
        module
            .string_pool
            .resolve(enumeration.variants[index].name)
            .to_string(),
    ))
}
