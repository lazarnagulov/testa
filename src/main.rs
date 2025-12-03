use std::{env, fs::File, io::Read, path::Path};

use testa::{
    core::parser::Parser,
    interpreter::{context::Context, eval_error::EvalError, evaluator},
};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Expected file path");
        std::process::exit(1);
    }
    let file_path = Path::new(&args[1]);

    let mut file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to open file: {}", err);
        std::process::exit(1);
    });

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        std::process::exit(1);
    });
    let mut parser = Parser::new(&source, file_path);
    let program = parser.parse().unwrap();

    let mut context = Context::default();

    evaluator::evaluate(program, &mut context).unwrap_or_else(|err| {
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
            EvalError::TypeMismatch { expected, got } => {
                format!("Expected '{}' but got '{}'", expected, got)
            }
            EvalError::NotDefined(name) => format!("{} is not defined", name),
            EvalError::MiscellaneousError(error) => error,
            EvalError::UncompatibleConstraint {
                data_type,
                constraint,
            } => {
                format!(
                    "Incompatible constraint '{}' for type '{}'",
                    constraint, data_type
                )
            }
            EvalError::FileError(error) => error,
            EvalError::InvalidTarget(error) => format!("Invalid target {}", error),
        };
        eprintln!("{}", error_message);
        std::process::exit(1);
    });
}
