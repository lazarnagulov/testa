use std::{error::Error, path::PathBuf};

pub fn generate_command(
    _input: PathBuf,
    _output: Option<PathBuf>,
    _format: Option<String>,
    _count: Option<usize>,
    _seed: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn check_command(
    _files: Vec<PathBuf>,
    _syntax_only: bool,
    _warnings: bool,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn lsp_command(
    _stdio: bool,
    _port: Option<u16>,
    _log_file: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn init_command(
    _directory: Option<PathBuf>,
    _name: Option<String>,
    _with_examples: bool,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
