use std::path::PathBuf;

use testa_core::{
    diagnostics::{Diagnostic, DiagnosticCode},
    utils::Span,
};

use crate::compiler::compile_file;

pub fn compile_module(input: PathBuf, output: Option<PathBuf>) -> Result<(), Vec<Diagnostic>> {
    let unit = compile_file(&input)?;
    let output_path = output.unwrap_or_else(|| input.with_extension("tmod"));

    unit.module.save(&output_path).map_err(|err| {
        vec![
            Diagnostic::error(Span::default(), format!("Failed to save module: {}", err))
                .with_code(DiagnosticCode::IOError)
                .with_hint(format!(
                    "Ensure you have write permissions for '{}'",
                    output_path.display()
                )),
        ]
    })
}
