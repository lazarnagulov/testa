pub mod context;
pub mod error;

#[cfg(test)]
mod tests;

use std::rc::Rc;

use rand::Rng;
use testa_core::ast::{
    DataType, Expression, ExpressionKind, Field, InfixOperator, PatternChar, PatternElement,
    PrefixOperator, Program, Statement,
};

use crate::{
    constraints::constrainted_type::ConstrainedType,
    evaluator::{context::Context, error::EvalError},
    model::{enumeration::Enum, template::Template},
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
        Statement::Template {
            parent,
            name,
            body,
            attributes: _,
            span: _,
        } => {
            let parent = match parent {
                Some(parent_name) => context.get_template(&parent_name).map(Rc::clone),
                None => None,
            };

            let template = Template::new(parent, body);
            context.insert_template(&name, template);
            Ok(Object::NoReturn)
        }
        Statement::Generate {
            template_name,
            body,
            count,
            span: _,
        } => evaluate_generate(template_name, body, &count, context),
        Statement::Enum {
            name,
            variants,
            attributes: _,
            span: _,
        } => {
            let enumeration = Enum::new(variants, context)?;
            context.insert_enum(&name, enumeration);
            Ok(Object::NoReturn)
        }
        Statement::OutputDirective { .. } => {
            todo!()
            // context.target_format = match argument.as_str() {
            //     "csv" => Ok(Target::Csv),
            //     value => Err(EvalError::InvalidTarget(value.to_string())),
            // }?;
            // context.target_config =
            //     options.iter().try_fold(
            //         HashMap::new(),
            //         |mut map, field| match evaluate_expression(&field.value, context) {
            //             Ok(result) => {
            //                 map.insert(field.name.clone(), result);
            //                 Ok(map)
            //             }
            //             Err(e) => Err(e),
            //         },
            //     )?;
            // Ok(Object::NoReturn)
        }
        Statement::Resource { .. } => todo!(),
        Statement::TypeDecl {
            name,
            data_type,
            attributes: _,
            span: _,
        } => {
            let ExpressionKind::Type(data_type) = data_type.kind else {
                unreachable!()
            };
            let data_type = ConstrainedType::new(data_type, context)?;
            context.insert_type(&name, data_type);

            Ok(Object::NoReturn)
        }
        Statement::ConstraintDecl { .. } => todo!(),
        Statement::OutputPathDirective { argument: path, .. } => {
            context.output_path = Some(path);
            Ok(Object::NoReturn)
        }
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
        obj => Err(EvalError::type_mismatch(
            "int".to_owned(),
            format!("{}", *obj),
        )),
    }?;
    let template = template_name.map_or_else(
        || Ok(Template::new(None, body)),
        |name| {
            context
                .get_template(&name)
                .map(|template| Ok((**template).clone()))
                .unwrap_or_else(|| Err(EvalError::NotDefined(name)))
        },
    )?;
    generate_file(&template, cardinality, context)
}

// TODO: BUffering and other fun stuff with files
fn generate_file(
    _template: &Template,
    _cardinality: isize,
    _context: &Context,
) -> Result<Object, EvalError> {
    todo!();
    // let mut file = File::create(context.output_path()).map_err(|error| {
    //     EvalError::MiscellaneousError(format!("Failed to create file: {}", error))
    // })?;
    // let generator = match context.target_format {
    //     Target::Csv => CsvGenerator::default().with_field_names(template.all_field_names()),
    // }
    // .with_config(&context.target_config);

    // if let Some(header) = generator.generate_header() {
    //     writeln!(file, "{}", header)
    //         .map_err(|error| EvalError::FileError(format!("Failed to write to file: {}", error)))?;
    // }
    // for _ in 0..cardinality {
    //     let record = template.visit(context)?;
    //     let line = generator.generate(&record).map_err(|error| {
    //         EvalError::MiscellaneousError(match error {
    //             GenerationError::NotSupported(error) => error.to_owned(),
    //         })
    //     })?;
    //     writeln!(file, "{}", line)
    //         .map_err(|error| EvalError::FileError(format!("Failed to write to file: {}", error)))?;
    // }
    // if let Some(footer) = generator.generate_footer() {
    //     writeln!(file, "{}", footer)
    //         .map_err(|error| EvalError::FileError(format!("Failed to write to file: {}", error)))?;
    // }
}

pub fn evaluate_expression(
    expression: &Expression,
    context: &Context,
) -> Result<Object, EvalError> {
    use testa_core::ast::ExpressionKind::*;
    match &expression.kind {
        IntLiteral(value) => Ok(Object::new(*value)),
        FloatLiteral(value) => {
            let parsed = value.parse::<f32>().map_err(|error| {
                EvalError::MiscellaneousError(format!("Error parsing float literal: {}", error))
            })?;
            Ok(Object::new(parsed))
        }
        StringLiteral(value) => Ok(Object::new(value.to_owned())),
        BooleanLiteral(value) => Ok(Object::new(*value)),
        StringPattern(pattern) => evaluate_string_pattern(pattern, context),
        Identifier(name) => evaluate_identifier(name, context),
        Type(data_type) => evaluate_data_type(data_type, context),
        List(_) => todo!("implement list expression evaluation"),
        Prefix {
            operator,
            expression,
        } => {
            let right = evaluate_expression(expression, context)?;
            evaluate_prefix_expression(*operator, &right)
        }
        Infix {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(left, context)?;
            let right = evaluate_expression(right, context)?;
            evaluate_infix_expression(&left, *operator, &right)
        }
        FuncCall { .. } => todo!(),
    }
}

fn evaluate_string_pattern(
    pattern: &[PatternElement],
    context: &Context,
) -> Result<Object, EvalError> {
    let mut rng = rand::rng();
    let mut result = String::new();

    for element in pattern {
        match element {
            PatternElement::Literal(literal, _) => result.push_str(literal),
            PatternElement::RepeatChar {
                ch,
                count,
                count_expression,
                span: _,
            } => {
                let mut total_count = *count;
                if let Some(expression) = count_expression {
                    total_count += match evaluate_expression(expression, context)? {
                        Object::Int(value) => {
                            if value > 0 {
                                Ok(value as usize)
                            } else {
                                Err(EvalError::MiscellaneousError(
                                    "expected int to be positive".to_owned(),
                                ))
                            }
                        }
                        Object::Range(start, end) => {
                            Ok(rng.random_range(start as i32..=end as i32) as usize)
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
                        PatternChar::Lowercase => result.push(rng.random_range('a'..='z') as char),
                        PatternChar::Uppercase => result.push(rng.random_range('A'..='Z') as char),
                        PatternChar::Digit => result.push(rng.random_range('0'..='9') as char),
                    }
                }
            }
            PatternElement::RepeatGroup { .. } => todo!(),
        };
    }
    Ok(Object::new(result))
}

pub fn evaluate_data_type(_data_type: &DataType, _context: &Context) -> Result<Object, EvalError> {
    // TODO: not cloning here?
    // TODO: add caching
    // let constrainted_type = ConstrainedType::new(data_type.clone(), context)?;
    // constrainted_type.visit(context)
    todo!()
}

pub fn evaluate_identifier(_name: &str, _context: &Context) -> Result<Object, EvalError> {
    // if let Some(enumeration) = context.get_enum(name) {
    //     return Ok(Object::new(enumeration.visit(context)?));
    // }
    // if let Some(data_type) = context.get_type(name) {
    //     return Ok(Object::new(data_type.visit(context)?));
    // }
    // Err(EvalError::NotDefined(name.to_owned()))
    todo!()
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
        InfixOperator::ExclusiveRange => Ok(Object::new((left, right - 1))),
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
