use std::{env, fs::File, io::Read};

use testa::{
    evaluator::{context::Context, eval_error::*, evaluator::evaluate},
    parser::{parser::Parser, parser_error::*},
};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Expected file path");
        std::process::exit(1);
    }
    let file_path = &args[1];

    let mut file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to open file: {}", err);
        std::process::exit(1);
    });

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        std::process::exit(1);
    });
    let mut parser = Parser::new(&source);
    let program = parser.parse().unwrap_or_else(|err| {
        let error_message = match err {
            ParserError::Expected { expected, got } => {
                format!("Expected '{}' but got '{}'", expected, got)
            }
            ParserError::InvalidDirective => format!("Invalid directive"),
            ParserError::UnexpectedEOF => format!("Missing enclosing \" or ;"),
            ParserError::Syntax(error) => error,
            ParserError::UndefinedConstraint => format!("Undefined constraint"),
        };
        eprintln!("{}", error_message);
        std::process::exit(1);
    });
    let mut context = Context::default();

    evaluate(program, &mut context).unwrap_or_else(|err| {
        let error_message = match err {
            EvalError::UnsupportedPrefixOperator { operator, object } => {
                format!("Bad operand type for unary {}: '{}'", operator, object)
            }
            EvalError::UnsupportedInfixOperand {
                left,
                operator,
                right,
            } => {
                format!(
                    "Unsupported operand type(s) for {}: {} and {}",
                    operator, left, right
                )
            }
            EvalError::TypeError { expected, got } => {
                format!("Expected '{}' but got '{}'", expected, got)
            }
            EvalError::NotDefined(name) => format!("{} is not defined", name),
            EvalError::General(error) => error,
            EvalError::UncompatibleConstraint {
                data_type,
                constraint,
            } => {
                format!(
                    "Incompatible constraint '{}' for type '{}'",
                    constraint, data_type
                )
            }
        };
        eprintln!("{}", error_message);
        std::process::exit(1);
    });
}
