use std::{env, fs, path::Path, process::exit};

use testa_core::{parser::Parser, symbol_table::symbol_table_builder::SymbolTableBuilder};
use testa_interpreter::evaluator::error::EvalError;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let file_path = get_input_path()?;

    let source =
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut parser = Parser::new(&source, &file_path);
    let program = parser.parse().map_err(|e| e.to_string())?;
    let symbol_table = match SymbolTableBuilder::new().build(&program) {
        Ok(table) => table,
        Err(errors) => {
            for error in errors {
                println!("{}", error.to_diagnostic().format_cli());
            }
            exit(1);
        }
    };

    symbol_table.dump();

    // let mut context = Context::default();

    // evaluator::evaluate(program, &mut context).map_err(format_eval_error)?;

    Ok(())
}

fn get_input_path() -> Result<std::path::PathBuf, String> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(path) => Ok(Path::new(path).to_path_buf()),
        None => Err("Expected file path".into()),
    }
}

fn _format_eval_error(err: EvalError) -> String {
    match err {
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
        EvalError::NotDefined(name) => format!("{name} is not defined"),
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
    }
}
