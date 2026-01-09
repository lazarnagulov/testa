use std::path::PathBuf;

use testa_core::diagnostics::Diagnostic;

use crate::compiler::{compile_file, parse_file};

pub fn check_command(
    files: Vec<PathBuf>,
    syntax_only: bool,
    warnings: bool,
) -> Result<(), Vec<Diagnostic>> {
    for file in files {
        if syntax_only {
            parse_file(&file)?;
        } else {
            compile_file(&file, warnings)?;
        }
    }
    Ok(())
}
