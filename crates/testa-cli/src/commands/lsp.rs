use std::{error::Error, path::PathBuf};

pub fn lsp_command(
    _stdio: bool,
    _port: Option<u16>,
    _log_file: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    todo!()
}
