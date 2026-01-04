
use rand::Rng;
use testa_core::analyser::symbol_table::symbol::VariantInfo;

use crate::{evaluator::{context::{Context, State}, error::EvalError, expression::evaluate_expression}, object::Object};

pub(crate) fn evaluate_enum(
    ctx: &Context,
    state: &mut State,
    variants: &[VariantInfo],
) -> Result<Object, EvalError> {
    let mut cumulative_weights = Vec::with_capacity(variants.len());
    let mut total_weight = 0.0;

    for variant in variants {
        let weight = if let Some(weight_expr) = &variant.weight {
            match evaluate_expression(ctx, state, weight_expr)? {
                Object::Int(w) if w > 0 => w as f64,
                Object::Int(w) => return Err(EvalError::InvalidWeight {
                    variant: variant.name.clone(),
                    value: w,
                    span: weight_expr.span,
                }),
                _ => return Err(EvalError::TypeMismatch {
                    expected: "positive integer".to_string(),
                    got: "other".to_string(),
                    span: weight_expr.span,
                }),
            }
        } else {
            1.0
        };

        total_weight += weight;
        cumulative_weights.push(total_weight);
    }

    let rand_val = state.rng.random::<f64>() * total_weight;

    match cumulative_weights.binary_search_by(|&cumulative_weight| cumulative_weight.partial_cmp(&rand_val).unwrap()) {
        Ok(index) | Err(index) => {
            Ok(Object::String(variants[index].name.clone()))
        }
    }
}
