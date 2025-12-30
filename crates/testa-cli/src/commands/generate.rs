use std::{error::Error, path::PathBuf};

use testa_interpreter::evaluator::{Evaluator, context::Context};

use crate::compiler::compile_file;

pub fn generate_command(
    input: PathBuf,
    _output: Option<PathBuf>,
    _format: Option<String>,
    _count: Option<usize>,
    _seed: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    let (program, symbol_table) = compile_file(&input, false)?;
    let context = Context::new(symbol_table);
    let evaluator = Evaluator::new(context);
    evaluator.evaluate(program)?;

    Ok(())
}
