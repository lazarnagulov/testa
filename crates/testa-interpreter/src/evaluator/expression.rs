use testa_core::ast::{Expression, InfixOperator, PrefixOperator};

use crate::{
    evaluator::{Evaluator, error::EvalError},
    object::Object,
};

impl Evaluator {
    pub(super) fn evaluate_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Object, EvalError> {
        use testa_core::ast::ExpressionKind::*;
        match &expression.kind {
            IntLiteral(value) => Ok(Object::new(*value)),
            FloatLiteral(value) => {
                let parsed = value.parse::<f32>().map_err(|error| {
                    EvalError::MiscellaneousError(format!("Error parsing float literal: {}", error), expression.span)
                })?;
                Ok(Object::new(parsed))
            }
            StringLiteral(value) => Ok(Object::new(value.to_owned())),
            BooleanLiteral(value) => Ok(Object::new(*value)),
            StringPattern(..) => todo!(),
            Identifier(..) => todo!(),
            Type(..) => todo!(),
            List(_) => todo!("implement list expression evaluation"),
            Prefix {
                operator,
                expression,
            } => {
                let right = self.evaluate_expression(expression)?;
                self.evaluate_prefix_expression(*operator, &right)
            }
            Infix {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate_expression(left)?;
                let right = self.evaluate_expression(right)?;
                self.evaluate_infix_expression(&left, *operator, &right)
            }
            FuncCall { .. } => todo!(),
        }
    }

    fn evaluate_prefix_expression(
        &self,
        operator: PrefixOperator,
        right: &Object,
    ) -> Result<Object, EvalError> {
        match operator {
            PrefixOperator::BitNegate => self.evaluate_bit_negate(right),
            PrefixOperator::LogicalNegate => self.evaluate_logical_negate(right),
            PrefixOperator::Negative => self.evaluate_negate(right),
        }
    }

    fn evaluate_infix_expression(
        &self,
        left: &Object,
        operator: InfixOperator,
        right: &Object,
    ) -> Result<Object, EvalError> {
        match (left, right) {
            (Object::Int(left), Object::Int(right)) => {
                self.evaluate_integer_infix(*left, operator, *right)
            }
            (Object::Float(left), Object::Float(right)) => {
                self.evaluate_float_infix(*left, operator, *right)
            }
            (Object::String(left), Object::String(right)) => {
                self.evaluate_string_infix(left, operator, right)
            }
            (left, right) => Err(EvalError::unsupported_infix_operator(
                format!("{}", *left),
                operator,
                format!("{}", *right),
            )),
        }
    }

    fn evaluate_string_infix(
        &self,
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
        &self,
        left: f32,
        operator: InfixOperator,
        right: f32,
    ) -> Result<Object, EvalError> {
        match operator {
            InfixOperator::Plus => Ok(Object::new(left + right)),
            InfixOperator::Minus => Ok(Object::new(left - right)),
            InfixOperator::Divide => Ok(Object::new(left / right)),
            InfixOperator::Multiply => Ok(Object::new(left * right)),
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
        &self,
        left: isize,
        operator: InfixOperator,
        right: isize,
    ) -> Result<Object, EvalError> {
        match operator {
            InfixOperator::Plus => Ok(Object::new(left + right)),
            InfixOperator::Minus => Ok(Object::new(left - right)),
            InfixOperator::Divide => Ok(Object::new(left / right)),
            InfixOperator::Mod => Ok(Object::new(left % right)),
            InfixOperator::Multiply => Ok(Object::new(left * right)),
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

    fn evaluate_negate(&self, right: &Object) -> Result<Object, EvalError> {
        match right {
            Object::Int(value) => Ok(Object::new(-value)),
            Object::Float(value) => Ok(Object::new(-value)),
            _ => Err(EvalError::unsupported_prefix_operator(
                PrefixOperator::BitNegate,
                format!("{}", right),
            )),
        }
    }

    fn evaluate_logical_negate(&self, right: &Object) -> Result<Object, EvalError> {
        match right {
            Object::Boolean(value) => Ok(Object::new(!value)),
            _ => Err(EvalError::unsupported_prefix_operator(
                PrefixOperator::LogicalNegate,
                format!("{}", right),
            )),
        }
    }

    fn evaluate_bit_negate(&self, right: &Object) -> Result<Object, EvalError> {
        match right {
            Object::Int(value) => Ok(Object::new(!value)),
            _ => Err(EvalError::unsupported_prefix_operator(
                PrefixOperator::BitNegate,
                format!("{}", right),
            )),
        }
    }
}