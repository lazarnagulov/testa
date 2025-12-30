use std::{error::Error, path::PathBuf};

use crate::compiler::compile_file;

pub fn generate_command(
    input: PathBuf,
    _output: Option<PathBuf>,
    _format: Option<String>,
    _count: Option<usize>,
    _seed: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    compile_file(&input, false)?;
    // TODO: generation
    Ok(())
}
