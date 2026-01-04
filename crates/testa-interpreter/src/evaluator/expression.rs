use crate::{
    evaluator::{
        context::{Context, State},
        error::EvalError,
    },
    object::Object,
    util::generate_random_string,
};
use rand::Rng;
use testa_core::{
    analyser::symbol_table::symbol::SymbolKind,
    ast::{
        DataType, DataTypeKind, Expression, InfixOperator, PrefixOperator,
        expression::ExpressionKind,
    },
    utils::Span,
};

pub fn evaluate_expression(
    ctx: &Context,
    state: &mut State,
    expression: &Expression,
) -> Result<Object, EvalError> {
    match &expression.kind {
        ExpressionKind::IntLiteral(value) => Ok(Object::new(*value)),

        ExpressionKind::FloatLiteral(value) => {
            let parsed = value.parse::<f32>().map_err(|error| {
                EvalError::MiscellaneousError(
                    format!("Error parsing float literal: {}", error),
                    expression.span,
                )
            })?;
            Ok(Object::new(parsed))
        }

        ExpressionKind::StringLiteral(value) => Ok(Object::new(value.clone())),
        ExpressionKind::BooleanLiteral(value) => Ok(Object::new(*value)),

        ExpressionKind::Identifier(name) => evaluate_identifier(ctx, name, expression.span),

        ExpressionKind::Prefix {
            operator,
            expression,
        } => {
            let right = evaluate_expression(ctx, state, expression)?;
            evaluate_prefix_expression(*operator, &right)
        }

        ExpressionKind::Infix {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(ctx, state, left)?;
            let right = evaluate_expression(ctx, state, right)?;

            if matches!(operator, InfixOperator::Divide)
                && matches!(right, Object::Int(0) | Object::Float(0.0))
            {
                return Err(EvalError::DivisionByZero {
                    span: expression.span,
                });
            }

            evaluate_infix_expression(&left, *operator, &right)
        }

        ExpressionKind::Type(data_type) => evaluate_data_type(ctx, state, data_type),

        ExpressionKind::List(_) => todo!("implement list expression evaluation"),
        ExpressionKind::StringPattern(..) => todo!(),
        ExpressionKind::FuncCall { .. } => todo!(),
    }
}

fn evaluate_prefix_expression(
    operator: PrefixOperator,
    right: &Object,
) -> Result<Object, EvalError> {
    match operator {
        PrefixOperator::BitNegate => evaluate_bit_negate(right),
        PrefixOperator::LogicalNegate => evaluate_logical_negate(right),
        PrefixOperator::Negative => evaluate_negate(right),
    }
}

fn evaluate_infix_expression(
    left: &Object,
    operator: InfixOperator,
    right: &Object,
) -> Result<Object, EvalError> {
    match (left, right) {
        (Object::Int(l), Object::Int(r)) => evaluate_integer_infix(*l, operator, *r),

        (Object::Boolean(l), Object::Boolean(r)) => evaluate_boolean_infix(*l, operator, *r),

        (Object::Float(l), Object::Float(r)) => evaluate_float_infix(*l, operator, *r),

        (Object::String(l), Object::String(r)) => evaluate_string_infix(l, operator, r),

        _ => Err(EvalError::unsupported_infix_operator(
            format!("{}", left),
            operator,
            format!("{}", right),
        )),
    }
}

fn evaluate_string_infix(
    left: &str,
    operator: InfixOperator,
    right: &str,
) -> Result<Object, EvalError> {
    match operator {
        InfixOperator::Plus => Ok(Object::new(format!("{}{}", left, right))),
        InfixOperator::Equal => Ok(Object::new(left == right)),
        InfixOperator::NotEqual => Ok(Object::new(left != right)),
        InfixOperator::LessThan => Ok(Object::new(left < right)),
        InfixOperator::GreaterThan => Ok(Object::new(left > right)),
        InfixOperator::LessThanOrEqual => Ok(Object::new(left <= right)),
        InfixOperator::GreaterThanOrEqual => Ok(Object::new(left >= right)),
        _ => Err(EvalError::unsupported_infix_operator(left, operator, right)),
    }
}

fn evaluate_float_infix(
    left: f32,
    operator: InfixOperator,
    right: f32,
) -> Result<Object, EvalError> {
    match operator {
        InfixOperator::Plus => Ok(Object::new(left + right)),
        InfixOperator::Minus => Ok(Object::new(left - right)),
        InfixOperator::Multiply => Ok(Object::new(left * right)),
        InfixOperator::Divide => Ok(Object::new(left / right)),
        InfixOperator::Equal => Ok(Object::new(left == right)),
        InfixOperator::NotEqual => Ok(Object::new(left != right)),
        InfixOperator::LessThan => Ok(Object::new(left < right)),
        InfixOperator::GreaterThan => Ok(Object::new(left > right)),
        InfixOperator::LessThanOrEqual => Ok(Object::new(left <= right)),
        InfixOperator::GreaterThanOrEqual => Ok(Object::new(left >= right)),
        _ => Err(EvalError::unsupported_infix_operator(left, operator, right)),
    }
}

fn evaluate_integer_infix(
    left: isize,
    operator: InfixOperator,
    right: isize,
) -> Result<Object, EvalError> {
    match operator {
        InfixOperator::Plus => Ok(Object::new(left + right)),
        InfixOperator::Minus => Ok(Object::new(left - right)),
        InfixOperator::Multiply => Ok(Object::new(left * right)),
        InfixOperator::Divide => Ok(Object::new(left / right)),
        InfixOperator::Mod => Ok(Object::new(left % right)),
        InfixOperator::BitAnd => Ok(Object::new(left & right)),
        InfixOperator::BitOr => Ok(Object::new(left | right)),
        InfixOperator::BitXor => Ok(Object::new(left ^ right)),
        InfixOperator::BitLShift => Ok(Object::new(left << right)),
        InfixOperator::BitRShift => Ok(Object::new(left >> right)),
        InfixOperator::Equal => Ok(Object::new(left == right)),
        InfixOperator::NotEqual => Ok(Object::new(left != right)),
        InfixOperator::LessThan => Ok(Object::new(left < right)),
        InfixOperator::GreaterThan => Ok(Object::new(left > right)),
        InfixOperator::LessThanOrEqual => Ok(Object::new(left <= right)),
        InfixOperator::GreaterThanOrEqual => Ok(Object::new(left >= right)),
        InfixOperator::ExclusiveRange => Ok(Object::new((left, right - 1))),
        InfixOperator::InclusiveRange => Ok(Object::new((left, right))),
        _ => Err(EvalError::unsupported_infix_operator(left, operator, right)),
    }
}

fn evaluate_boolean_infix(
    left: bool,
    operator: InfixOperator,
    right: bool,
) -> Result<Object, EvalError> {
    match operator {
        InfixOperator::Equal => Ok(Object::new(left == right)),
        InfixOperator::NotEqual => Ok(Object::new(left != right)),
        InfixOperator::And => Ok(Object::new(left && right)),
        InfixOperator::Or => Ok(Object::new(left || right)),
        _ => Err(EvalError::unsupported_infix_operator(left, operator, right)),
    }
}

fn evaluate_negate(right: &Object) -> Result<Object, EvalError> {
    match right {
        Object::Int(v) => Ok(Object::new(-v)),
        Object::Float(v) => Ok(Object::new(-v)),
        _ => Err(EvalError::unsupported_prefix_operator(
            PrefixOperator::Negative,
            format!("{}", right),
        )),
    }
}

fn evaluate_logical_negate(right: &Object) -> Result<Object, EvalError> {
    match right {
        Object::Boolean(v) => Ok(Object::new(!v)),
        _ => Err(EvalError::unsupported_prefix_operator(
            PrefixOperator::LogicalNegate,
            format!("{}", right),
        )),
    }
}

fn evaluate_bit_negate(right: &Object) -> Result<Object, EvalError> {
    match right {
        Object::Int(v) => Ok(Object::new(!v)),
        _ => Err(EvalError::unsupported_prefix_operator(
            PrefixOperator::BitNegate,
            format!("{}", right),
        )),
    }
}

fn evaluate_identifier(ctx: &Context, name: &str, span: Span) -> Result<Object, EvalError> {
    let symbol = ctx
        .symbol_table
        .lookup(name)
        .ok_or_else(|| EvalError::NotDefined(name.to_string(), span))?;

    match &symbol.kind {
        SymbolKind::Enum { variants, .. } => {
            let idx = rand::random_range(0..variants.len());
            Ok(Object::new(variants[idx].name.clone()))
        }
        _ => Err(EvalError::NotDefined(name.to_string(), span)),
    }
}

fn evaluate_data_type(
    ctx: &Context,
    state: &mut State,
    data_type: &DataType,
) -> Result<Object, EvalError> {
    match &data_type.kind {
        DataTypeKind::Int => Ok(Object::new(state.rng.random::<i32>() as isize)),
        DataTypeKind::Float => Ok(Object::new(state.rng.random::<f32>())),
        DataTypeKind::Boolean => Ok(Object::new(state.rng.random_bool(0.5))),
        DataTypeKind::Str => {
            let size = state.rng.random_range(6..=20);
            Ok(Object::new(generate_random_string(&mut state.rng, size)))
        }
        DataTypeKind::List(inner) => evaluate_list(ctx, state, inner),
        DataTypeKind::Custom(name) => evaluate_identifier(ctx, name, data_type.span),
    }
}

fn evaluate_list(
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
