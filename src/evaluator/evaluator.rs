use std::{fs::File, io::Write};

use crate::{
    constraints::constrainted_type::ConstrainedType, enumeration::enumeration::Enum, parser::ast::{
        Expression, ExpressionKind::*, Field, InfixOperator, PrefixOperator, Program, Statement,
    }, template::template::Template
};

use super::{
    context::{Context, Visitor},
    eval_error::EvalError,
    object::Object,
};

pub fn evaluate(program: Program, context: &mut Context) -> Result<Object, EvalError> {
    let mut result = Object::NoReturn;
    for statement in program.0 {
        result = evaluate_statement(statement, context)?;
    }
    Ok(result)
}

fn evaluate_statement(statment: Statement, context: &mut Context) -> Result<Object, EvalError> {
    match statment {
        Statement::Expression(expression_statement) => {
            evaluate_expression(&expression_statement.expression, context)
        }
        Statement::Template { name, body } => {
            let template = Template::new(body);
            context.insert_template(&name, template);
            Ok(Object::NoReturn)
        }
        Statement::Generate {
            template_name,
            body,
            count,
        } => evaluate_generate(template_name, body, &count, context),
        Statement::Enum { name, variants } => {
            let enumeration = Enum::new(variants, context)?;
            context.insert_enum(&name, enumeration);
            Ok(Object::NoReturn)
        }
        Statement::OutputDirective { .. } => todo!(),
        Statement::Resource { .. } => todo!(),
        Statement::TypeDecl { name, data_type } => {
            let Type(data_type) = data_type.kind else {
                unreachable!()
            };
            let data_type = ConstrainedType::new(data_type, context)?;
            context.insert_type(&name, data_type);
            Ok(Object::NoReturn)
        }
        Statement::ConstraintDecl { .. } => todo!(),
    }
}

fn evaluate_generate(
    template_name: Option<String>,
    body: Vec<Field>,
    count: &Expression,
    context: &Context,
) -> Result<Object, EvalError> {
    let cardinality = match &evaluate_expression(count, context)? {
        Object::Int(value) => Ok(*value),
        obj => Err(EvalError::type_error("int".to_owned(), format!("{}", *obj))),
    }?;
    let template = template_name.map_or_else(
        || Ok(Template::new(body)),
        |name| {
            context
                .get_template(&name)
                .map(|template| Ok(template.clone()))
                .unwrap_or_else(|| Err(EvalError::NotDefined(name)))
        },
    )?;
    Ok(generate_csv(&template, cardinality, context)?)
}

fn generate_csv(
    template: &Template,
    cardinality: isize,
    context: &Context,
) -> Result<Object, EvalError> {
    let mut file = File::create("test.csv")
        .map_err(|error| EvalError::General(format!("Failed to create file: {}", error)))?;
    let header = template.field_names().collect::<Vec<_>>().join(",");

    writeln!(file, "{}", header)
        .map_err(|error| EvalError::General(format!("Failed to write to file: {}", error)))?;
    for _ in 0..cardinality {
        let line = template.visit(context).map(|fields| fields.join(","))?;
        writeln!(file, "{}", line)
            .map_err(|error| EvalError::General(format!("Failed to write to file: {}", error)))?;
    }
    Ok(Object::NoReturn)
}

pub fn evaluate_expression(
    expression: &Expression,
    context: &Context,
) -> Result<Object, EvalError> {
    match &expression.kind {
        IntLiteral(value) => Ok(Object::new(*value)),
        FloatLiteral(value) => {
            let parsed = value.parse::<f32>().map_err(|error| {
                EvalError::General(format!("Error parsing float literal: {}", error))
            })?;
            Ok(Object::new(parsed))
        }
        StringLiteral(value) => Ok(Object::new(value.to_owned())),
        BooleanLiteral(value) => Ok(Object::new(*value)),
        Identifier(name) => evaluate_identifier(name, context),
        Type(..) => todo!(),
        Prefix {
            operator,
            expression,
        } => {
            let right = evaluate_expression(&expression, context)?;
            evaluate_prefix_expression(*operator, &right)
        }
        Infix {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(&left, context)?;
            let right = evaluate_expression(&right, context)?;
            evaluate_infix_expression(&left, *operator, &right)
        }
        FuncCall { .. } => todo!(),
    }
}

fn evaluate_identifier(name: &str, context: &Context) -> Result<Object, EvalError> {
    // TODO: Add support for more identifiers - now it works only for enumerations
    let value = context
        .get_enum(name)
        .ok_or_else(|| EvalError::General("Only enums are supported for now".to_owned()))?
        .visit(context)?;

    Ok(Object::new(value))
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
        (Object::Int(left), Object::Int(right)) => evaluate_integer_infix(*left, operator, *right),
        (Object::Float(left), Object::Float(right)) => {
            evaluate_float_infix(*left, operator, *right)
        }
        (Object::String(left), Object::String(right)) => {
            evaluate_string_infix(left, operator, right)
        }
        (left, right) => Err(EvalError::unsupported_infix_operator(
            format!("{}", *left),
            operator,
            format!("{}", *right),
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
        InfixOperator::ExclusiveRange => Ok(Object::new((left, right-1))),
        InfixOperator::InclusiveRange => Ok(Object::new((left, right))),
        _ => Err(EvalError::unsupported_infix_operator(left, operator, right)),
    }
}

fn evaluate_negate(right: &Object) -> Result<Object, EvalError> {
    match right {
        Object::Int(value) => Ok(Object::new(-value)),
        Object::Float(value) => Ok(Object::new(-value)),
        _ => Err(EvalError::unsupported_prefix_operator(
            PrefixOperator::BitNegate,
            format!("{}", right),
        )),
    }
}

fn evaluate_logical_negate(right: &Object) -> Result<Object, EvalError> {
    match right {
        Object::Boolean(value) => Ok(Object::new(!value)),
        _ => Err(EvalError::unsupported_prefix_operator(
            PrefixOperator::LogicalNegate,
            format!("{}", right),
        )),
    }
}

fn evaluate_bit_negate(right: &Object) -> Result<Object, EvalError> {
    match right {
        Object::Int(value) => Ok(Object::new(!value)),
        _ => Err(EvalError::unsupported_prefix_operator(
            PrefixOperator::BitNegate,
            format!("{}", right),
        )),
    }
}
