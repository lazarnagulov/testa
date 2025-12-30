use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::commands::{
    check::check_command, generate::generate_command, info::info_command, init::init_command,
    lsp::lsp_command,
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
    pub fn run() -> Result<(), Box<dyn Error>> {
        let cli = Cli::try_parse()?;
        match cli.command {
            Command::Generate {
                input,
                output,
                format,
                count,
                seed,
            } => generate_command(input, output, format, count, seed),
            Command::Check {
                files,
                syntax_only,
                warnings,
            } => check_command(files, syntax_only, warnings),
            Command::Lsp {
                stdio,
                port,
                log_file,
            } => lsp_command(stdio, port, log_file),
            Command::Init {
                directory,
                name,
                with_examples,
            } => init_command(directory, name, with_examples),
            Command::Info { extended } => {
                info_command(extended);
                Ok(())
            }
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
        #[arg(short, long, value_enum, default_value = "csv")]
        format: Option<String>,
        #[arg(short, long, value_name = "COUNT")]
        count: Option<usize>,
        #[arg(short, long, value_name = "SEED")]
        seed: Option<u64>,
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
