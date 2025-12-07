use std::{env, fs, path::Path};

use testa_core::{lexer::Lexer, parser::Parser};

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let file_path = get_input_path()?;

    let source =
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer, &source, &file_path);
    parser.parse().map_err(|e| e.to_string())?;

    Ok(())
}

fn get_input_path() -> Result<std::path::PathBuf, String> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(path) => Ok(Path::new(path).to_path_buf()),
        None => Err("Expected file path".into()),
    }
}
