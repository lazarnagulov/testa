use std::{rc::Rc};

use rand::{distr::Alphanumeric, Rng};

use crate::parser::ast::{DataType, Expression, ExpressionKind::*, Field, InfixOperator, PrefixOperator, Program, Statement};

use super::{context::{Context, Template}, object::{EvalError, Object}};

pub fn evaluate(program: Program, context: &mut Context) -> Result<Rc<Object>, EvalError> {
    let mut result = Rc::from(Object::NoReturn);
    for statement in program.0 {
        result = evaluate_statement(statement, context)?;
    }
    Ok(result)
}

fn evaluate_statement(statment: Statement, context: &mut Context) -> Result<Rc<Object>, EvalError> {
    match statment {
        Statement::Expression(expression_statement) => evaluate_expression(expression_statement.expression),
        Statement::Template { name, body} => evaluate_template(&name, body, context),
        Statement::OutputDirective { .. } => todo!(),
        Statement::Enum { .. } => todo!(),
        Statement::Resource { .. } => todo!(),
        Statement::Generate { .. } => todo!(),
    }
}

fn evaluate_template(name: &str, body: Vec<Field>, context: &mut Context) -> Result<Rc<Object>, EvalError> {
    let template = Template::new( body);
    context.insert_template(name, template);
    Ok(Rc::from(Object::NoReturn))
}

fn evaluate_expression(expression: Expression) -> Result<Rc<Object>, EvalError> {
    match expression.kind {
        IntLiteral(value) => Ok(Rc::from(Object::new(value))),
        FloatLiteral(value) => {
            let parsed = match value.parse::<f32>() {
                Ok(val) => val,
                Err(_) => return Err("Error parsing float literal".to_string()),
            };
            Ok(Rc::from(Object::new(parsed)))
        },
        StringLiteral(value) => Ok(Rc::from(Object::new(value.to_owned()))),
        BooleanLiteral(value) => Ok(Rc::from(Object::new(value))),
        Identifier(_) => todo!(),
        Type(data_type) => evaluate_data_type(data_type),
        Prefix { operator, expression } => {
            let right = evaluate_expression(*expression)?;
            evaluate_prefix_expression(operator, &right)
        },
        Infix { left, operator, right } => {
            let left = evaluate_expression(*left)?;
            let right = evaluate_expression(*right)?;
            evaluate_infix_expression(&left, operator, &right)
        }
        FuncCall { .. } => todo!()
    }
}

fn evaluate_data_type(data_type: DataType) -> Result<Rc<Object>, String> {
    let mut rng = rand::rng();
    match data_type {
        DataType::Int => Ok(Rc::from(Object::new(rng.random::<i32>() as isize))),
        DataType::Str => {
            let size = rng.random_range(1..=16);
            let value: String = rng.sample_iter(&Alphanumeric)
                            .take(size)
                            .map(char::from)
                            .collect();
            Ok(Rc::from(Object::new(value)))
        },
        DataType::Float => Ok(Rc::from(Object::new(rng.random::<f32>()))),
    }
}

fn evaluate_prefix_expression(operator: PrefixOperator, right: &Rc<Object>) -> Result<Rc<Object>, EvalError> {
    match operator {
        PrefixOperator::BitNegate => evaluate_bit_negate(right),
        PrefixOperator::LogicalNegate => evaluate_logical_negate(right),
        PrefixOperator::Negative => evaluate_negate(right),
    }
}

fn evaluate_infix_expression(left: &Object, operator: InfixOperator, right: &Object) -> Result<Rc<Object>, EvalError> {
    match (left, right) {
        (Object::Int(left), Object::Int(right)) => evaluate_integer_infix(*left, operator, *right),
        (Object::Float(left), Object::Float(right)) => evaluate_float_infix(*left, operator, *right),
        (Object::String(left), Object::String(right)) => evaluate_string_infix(left, operator, right),
        (left, right) => Err(format!("Unsupported operand type(s) for {}: {} and {}", operator, left, right))
    }
}

fn evaluate_string_infix(left: &str, operator: InfixOperator, right: &str) -> Result<Rc<Object>, String> {
    match operator {
        InfixOperator::Plus => Ok(Rc::from(Object::new(format!("{}{}", left, right)))),
        InfixOperator::Equal => Ok(Rc::from(Object::new(left == right))),
        InfixOperator::NotEqual => Ok(Rc::from(Object::new(left != right))),
        InfixOperator::LessThan => Ok(Rc::from(Object::new(left < right))),
        InfixOperator::GreaterThan => Ok(Rc::from(Object::new(left > right))),
        InfixOperator::LessThanOrEqual => Ok(Rc::from(Object::new(left <= right))),
        InfixOperator::GreaterThanOrEqual => Ok(Rc::from(Object::new(left >= right))),
        op => Err(format!("Unsupported operand type(s) for {}: string and string", op))    
    }
}

fn evaluate_float_infix(left: f32, operator: InfixOperator, right: f32) -> Result<Rc<Object>, String> {
    match operator {
        InfixOperator::Plus => Ok(Rc::from(Object::new(left + right))),
        InfixOperator::Minus => Ok(Rc::from(Object::new(left - right))),
        InfixOperator::Divide => Ok(Rc::from(Object::new(left / right))),
        InfixOperator::Multiply => Ok(Rc::from(Object::new(left * right))),
        InfixOperator::Equal => Ok(Rc::from(Object::new(left == right))),
        InfixOperator::NotEqual => Ok(Rc::from(Object::new(left != right))),
        InfixOperator::LessThan => Ok(Rc::from(Object::new(left < right))),
        InfixOperator::GreaterThan => Ok(Rc::from(Object::new(left > right))),
        InfixOperator::LessThanOrEqual => Ok(Rc::from(Object::new(left <= right))),
        InfixOperator::GreaterThanOrEqual => Ok(Rc::from(Object::new(left >= right))),
        op => Err(format!("Unsupported operand type(s) for {}: int and int", op))
    }
}


fn evaluate_integer_infix(left: isize, operator: InfixOperator, right: isize) -> Result<Rc<Object>, EvalError> {
    match operator {
        InfixOperator::Plus => Ok(Rc::from(Object::new(left + right))),
        InfixOperator::Minus => Ok(Rc::from(Object::new(left - right))),
        InfixOperator::Divide => Ok(Rc::from(Object::new(left / right))),
        InfixOperator::Multiply => Ok(Rc::from(Object::new(left * right))),
        InfixOperator::BitAnd => Ok(Rc::from(Object::new(left & right))),
        InfixOperator::BitOr => Ok(Rc::from(Object::new(left | right))),
        InfixOperator::BitXor => Ok(Rc::from(Object::new(left ^ right))),
        InfixOperator::BitLShift => Ok(Rc::from(Object::new(left << right))),
        InfixOperator::BitRShift => Ok(Rc::from(Object::new(left >> right))),
        InfixOperator::Equal => Ok(Rc::from(Object::new(left == right))),
        InfixOperator::NotEqual => Ok(Rc::from(Object::new(left != right))),
        InfixOperator::LessThan => Ok(Rc::from(Object::new(left < right))),
        InfixOperator::GreaterThan => Ok(Rc::from(Object::new(left > right))),
        InfixOperator::LessThanOrEqual => Ok(Rc::from(Object::new(left <= right))),
        InfixOperator::GreaterThanOrEqual => Ok(Rc::from(Object::new(left >= right))),
        InfixOperator::ExclusiveRange => todo!(),
        InfixOperator::InclusiveRange => todo!(),
        op => Err(format!("Unsupported operand type(s) for {}: int and int", op))
    }
}

fn evaluate_negate(right: &Object) -> Result<Rc<Object>, EvalError> {
    match right {
        Object::Int(value) => Ok(Rc::from(Object::new(-value))),
        Object::Float(value) => Ok(Rc::from(Object::new(-value))),
        obj => Err(format!("Bad operand type for unary -: '{}'", obj))
    }
}

fn evaluate_logical_negate(right: &Object) -> Result<Rc<Object>, EvalError> {
    match right {
        Object::Boolean(value) => Ok(Rc::from(Object::new(!value))),
        obj => Err(format!("Bad operand type for unary !: '{}'", obj))
    }
}

fn evaluate_bit_negate(right: &Object) -> Result<Rc<Object>, EvalError> {
    match right {
        Object::Int(value) => Ok(Rc::from(Object::new(!value))),
        obj => Err(format!("Bad operand type for unary ~: '{}'", obj))
    }
}

