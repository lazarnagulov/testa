mod cli;
mod command;

use crate::cli::Cli;

fn main() {
    if let Err(err) = Cli::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
