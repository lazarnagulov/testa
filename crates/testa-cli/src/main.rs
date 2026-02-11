mod cli;
mod commands;
mod compiler;

use crate::cli::Cli;

// TODO: think about using library for logging (https://docs.rs/fern/latest/fern/)
fn main() {
    if let Err(errors) = Cli::run() {
        for error in errors {
            eprintln!("{}", error.format_cli())
        }
        std::process::exit(1);
    }
}
