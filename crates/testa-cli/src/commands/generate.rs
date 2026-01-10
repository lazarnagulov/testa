use std::path::PathBuf;
use testa_core::diagnostics::{Diagnostic, DiagnosticCode};
use testa_core::utils::Span;
use testa_generation::generator::create_file_generator;
use testa_interpreter::evaluator::context::GenerateOptions;
use testa_interpreter::evaluator::{Evaluator, context::Context};
use testa_interpreter::generator::RecordGenerator;

use crate::cli::Command;
use crate::compiler::compile_file;

impl TryFrom<Command> for GenerateOptions {
    type Error = &'static str;

    fn try_from(cmd: Command) -> Result<Self, Self::Error> {
        match cmd {
            Command::Generate {
                input,
                output,
                format,
                count,
                seed,
            } => Ok(Self {
                input,
                output_path: output,
                format,
                count,
                seed,
            }),
            _ => Err("Not a generate command"),
        }
    }
}

pub fn generate_command(options: GenerateOptions) -> Result<(), Vec<Diagnostic>> {
    let (program, symbol_table) = compile_file(&options.input, false)?;
    let context = Context::new(symbol_table);
    let mut evaluator = Evaluator::new(context, options.seed);

    evaluator
        .evaluate_directives(&program)
        .map_err(|err| vec![err.to_diagnostic()])?;
    let (format, config, path) = evaluator.output_config();

    let file_generator = create_file_generator(format, config);
    let output_path = path
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("output.{}", format.extension())));

    let mut record_generator = RecordGenerator::new(&mut evaluator);
    record_generator
        .generate_infos(&program)
        .map_err(|err| vec![err.to_diagnostic()])?;
    record_generator
        .write_records(file_generator, output_path)
        .map_err(|err| {
            vec![
                Diagnostic::error(Span::default(), err.to_string())
                    .with_code(DiagnosticCode::IOError),
            ]
        })?;
    Ok(())
}
