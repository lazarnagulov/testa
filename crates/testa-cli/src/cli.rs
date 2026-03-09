use std::path::PathBuf;

use clap::{Parser, Subcommand};
use testa_core::{diagnostics::Diagnostic, utils::Span};
use testa_interpreter::evaluator::context::OutputFormat;

use crate::commands::{
    check::check_command, compile::compile_module, generate::generate_command, info::info_command, init::init_command
};

#[derive(Parser)]
#[command(name = "testa")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run() -> Result<(), Vec<Diagnostic>> {
        let cli = Cli::try_parse()
            .map_err(|err| vec![Diagnostic::info(Span::default(), err.to_string())])?;
        match cli.command {
            Command::Generate { .. } => generate_command(
                cli.command
                    .try_into()
                    .map_err(|err| vec![Diagnostic::error(Span::default(), err)])?,
            ),
            Command::Compile { input, output } => {
                compile_module(input, output)
            },
            Command::Check {
                files,
                syntax_only,
                warnings,
            } => check_command(files, syntax_only, warnings),
            Command::Init {
                directory,
                name,
                with_examples,
            } => init_command(directory, name, with_examples),
            Command::Info { extended } => {
                info_command(extended);
                Ok(())
            }
            _ => todo!(),
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    Generate {
        #[arg(value_name = "FILE")]
        input: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,
        #[arg(short, long, value_name = "COUNT")]
        count: Option<usize>,
        #[arg(short, long, value_name = "SEED")]
        seed: Option<u64>,
    },
    Compile {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Check {
        #[arg(value_name = "FILE", required = true)]
        files: Vec<PathBuf>,
        #[arg(long)]
        syntax_only: bool,
        #[arg(short = 'W', long)]
        warnings: bool,
    },
    Lsp {
        #[arg(long, conflicts_with = "port")]
        stdio: bool,
        #[arg(long, value_name = "PORT")]
        port: Option<u16>,
        #[arg(long, value_name = "FILE")]
        log_file: Option<PathBuf>,
    },
    Init {
        #[arg(value_name = "DIR")]
        directory: Option<PathBuf>,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(long)]
        with_examples: bool,
    },
    Info {
        #[arg(long)]
        extended: bool,
    },
}
