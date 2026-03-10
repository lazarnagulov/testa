use rand::Rng;
use testa_core::ast::{PatternChar, PatternElement};

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
    elements: &[PatternElement],
) -> Result<Object, EvalError> {
    let mut result = String::new();
    for element in elements {
        match element {
            PatternElement::Literal(literal, _) => result.push_str(literal),
            PatternElement::RepeatChar {
                ch,
                count,
                count_expression,
                span,
            } => {
                let mut total_count = *count;
                if let Some(expression) = count_expression {
                    total_count += match evaluate_expression(ctx, state, expression)? {
                        Object::Int(value) => {
                            if value > 0 {
                                Ok(value as usize)
                            } else {
                                Err(EvalError::MiscellaneousError(
                                    "expected int to be positive".to_owned(),
                                    *span,
                                ))
                            }
                        }
                        Object::Range(start, end) => {
                            Ok(state.rng.random_range(start as i32..=end as i32) as usize)
                        }
                        obj => Err(EvalError::type_mismatch(
                            "int".to_owned(),
                            format!("{}", obj),
                        )),
                    }?;
                } else {
                    total_count += 1;
                }

                for _ in 0..total_count - 1 {
                    match ch {
                        PatternChar::Lowercase => result.push(state.rng.random_range('a'..='z')),
                        PatternChar::Uppercase => result.push(state.rng.random_range('A'..='Z')),
                        PatternChar::Digit => result.push(state.rng.random_range('0'..='9')),
                    }
                }
            }
            PatternElement::RepeatGroup { .. } => todo!(),
        };
    }
    Ok(Object::new(result))
}
