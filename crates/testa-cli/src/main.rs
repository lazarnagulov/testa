mod cli;
mod commands;
mod compiler;

use crate::cli::Cli;

// TODO: think about using library for logging (https://docs.rs/fern/latest/fern/)
fn main() {
    if let Err(err) = Cli::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
