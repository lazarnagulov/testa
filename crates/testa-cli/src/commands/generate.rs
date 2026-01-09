use std::io::Write;
use std::{error::Error, fs::File, path::PathBuf};
use testa_core::diagnostics::Diagnostic;
use testa_generation::generator::FileGenerator;
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

    evaluator.evaluate_directives(&program).unwrap();
    let (format, config, path) = evaluator.output_config();

    let file_generator = create_file_generator(format, config);
    let output_path = path
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("output.{}", format.extension())));

    let mut record_generator = RecordGenerator::new(&mut evaluator);
    record_generator.generate_infos(&program).unwrap();
    write_records_streaming(record_generator, file_generator, output_path).unwrap();

    Ok(())
}

fn write_records_streaming(
    generator: RecordGenerator,
    file_generator: Box<dyn FileGenerator>,
    path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&path)?;
    let field_names = generator.get_field_names()?;

    if let Some(header) = file_generator.generate_header(&field_names) {
        writeln!(file, "{}", header)?;
    }

    let total = generator.len();
    let mut count = 0;
    let mut first = true;

    for result in generator {
        let record = result?;

        count += 1;
        if count % 10000 == 0 {
            println!("Generated {}/{} records...", count, total);
        }

        if !first && file_generator.needs_separator() {
            write!(file, "{}", file_generator.separator())?;
        }
        first = false;

        let line = file_generator.generate(&record)?;
        write!(file, "{}", line)?;
    }

    if let Some(footer) = file_generator.generate_footer() {
        writeln!(file, "{}", footer)?;
    }

    println!("Generated {} records to {:?}", count, path);

    Ok(())
}
