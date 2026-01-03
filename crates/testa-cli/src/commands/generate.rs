use std::io::Write;
use std::{error::Error, fs::File, path::PathBuf};
use testa_generation::generator::FileGenerator;
use testa_generation::generator::create_file_generator;
use testa_interpreter::{
    evaluator::{Evaluator, context::Context},
    generator::RecordGenerator,
};

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
    let mut evaluator = Evaluator::new(context);

    evaluator.evaluate_directives(&program)?;
    let (format, config, path) = evaluator.output_config();

    let file_generator = create_file_generator(format, config);
    let output_path = path
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("output.{}", format.extension())));

    let mut record_generator = RecordGenerator::new(&evaluator);
    record_generator.generate_infos(&program)?;
    write_records_streaming(record_generator, file_generator, output_path)?;

    Ok(())
}

fn write_records_streaming(
    generator: RecordGenerator,
    file_generator: Box<dyn FileGenerator>,
    path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&path)?;
    let field_names = generator.get_field_names_from_generator()?;

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
        writeln!(file, "{}", line)?;
    }

    if let Some(footer) = file_generator.generate_footer() {
        writeln!(file, "{}", footer)?;
    }

    println!("Generated {} records to {:?}", count, path);

    Ok(())
}
