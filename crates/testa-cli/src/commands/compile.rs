use std::path::PathBuf;

use testa_core::{
    diagnostics::{Diagnostic, DiagnosticCode},
    utils::Span,
};
use testa_hir::AstLowering;

use crate::compiler::compile_file;

pub fn compile_module(input: PathBuf, output: Option<PathBuf>) -> Result<(), Vec<Diagnostic>> {
    let unit = compile_file(&input)?;
    let imported_refs = unit.imported.iter().map(|(k, v)| (k.clone(), v)).collect();

    let lowering = AstLowering::new(input.clone());
    let hir = lowering.lower(&unit.program, &unit.analysis, &unit.source, &imported_refs);
    let output_path = output.unwrap_or_else(|| input.with_extension("tmod"));

    hir.save(&output_path).map_err(|err| {
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
