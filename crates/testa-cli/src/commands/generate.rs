use testa_core::diagnostics::Diagnostic;
use testa_interpreter::evaluator::context::GenerateOptions;

use crate::cli::Command;

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

pub fn generate_command(_options: GenerateOptions) -> Result<(), Vec<Diagnostic>> {
    todo!()
    // let unit = compile_file(&options.input)?;

    // let symbol_table = unit.analysis.symbol_table;
    // let context = Context::new(symbol_table);
    // let mut evaluator = Evaluator::new(context, options.seed);

    // evaluator
    //     .evaluate_directives(&unit.program)
    //     .map_err(|err| vec![err.to_diagnostic()])?;
    // let (format, config, path) = evaluator.output_config();

    // let file_generator = create_file_generator(format, config);
    // let output_path = path
    //     .clone()
    //     .unwrap_or_else(|| PathBuf::from(format!("output.{}", format.extension())));

    // let mut record_generator = RecordGenerator::new(&mut evaluator);
    // record_generator
    //     .generate_infos(&unit.program)
    //     .map_err(|err| vec![err.to_diagnostic()])?;
    // record_generator
    //     .write_records(file_generator, output_path)
    //     .map_err(|err| {
    //         vec![
    //             Diagnostic::error(Span::default(), err.to_string())
    //                 .with_code(DiagnosticCode::IOError),
    //         ]
    //     })?;
    // Ok(())
}
