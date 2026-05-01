use rand::Rng;
use testa_hir::module::{PatternChar, PatternPart};

use crate::{
    evaluator::{
        context::{Context, State},
        error::EvalError,
        expression::evaluate_expression,
    },
    object::Object,
};

pub(crate) fn evaluate_string_pattern(
    ctx: &Context,
    state: &mut State,
    parts: &[PatternPart],
) -> Result<Object, EvalError> {
    let mut result = String::new();

    for part in parts {
        match part {
            PatternPart::Literal(id) => {
                result.push_str(ctx.resolve_local_string(*id));
            }
            PatternPart::RepeatChar {
                ch,
                count,
                count_expr,
            } => {
                let mut total_count = *count;

                if let Some(expr) = count_expr {
                    total_count += match evaluate_expression(ctx, state, expr)? {
                        Object::Int(value) if value > 0 => Ok(value as usize),
                        Object::Int(value) => Err(EvalError::MiscellaneousError(
                            format!("expected positive int, got {}", value),
                            Default::default(),
                        )),
                        Object::Range(start, end) => {
                            Ok(state.rng.random_range(start as i32..=end as i32) as usize)
                        }
                        other => Err(EvalError::type_mismatch(
                            "int".to_string(),
                            format!("{}", other),
                        )),
                    }?;
                } else {
                    total_count += 1;
                }

                for _ in 0..total_count {
                    result.push(random_char_for(state, ch));
                }
            }
            PatternPart::RepeatGroup { chars, count } => {
                let total_count = match evaluate_expression(ctx, state, count)? {
                    Object::Int(value) if value > 0 => Ok(value as usize),
                    Object::Int(value) => Err(EvalError::MiscellaneousError(
                        format!("expected positive int, got {}", value),
                        Default::default(),
                    )),
                    Object::Range(start, end) => {
                        Ok(state.rng.random_range(start as i32..=end as i32) as usize)
                    }
                    other => Err(EvalError::type_mismatch(
                        "int".to_string(),
                        format!("{}", other),
                    )),
                }?;

                for _ in 0..total_count - 1 {
                    let idx = state.rng.random_range(0..chars.len());
                    result.push(random_char_for(state, &chars[idx]));
                }
            }
        }
    }

    Ok(Object::new(result))
}

fn random_char_for(state: &mut State, ch: &PatternChar) -> char {
    match ch {
        PatternChar::Lowercase => state.rng.random_range('a'..='z'),
        PatternChar::Uppercase => state.rng.random_range('A'..='Z'),
        PatternChar::Digit => state.rng.random_range('0'..='9'),
    }
}
