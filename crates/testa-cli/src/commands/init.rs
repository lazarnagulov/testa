use std::{
    fs,
    path::{Path, PathBuf},
};

use testa_core::{
    diagnostics::{Diagnostic, DiagnosticCode},
    utils::Span,
};

pub fn init_command(
    directory: Option<PathBuf>,
    name: Option<String>,
    with_examples: bool,
) -> Result<(), Vec<Diagnostic>> {
    let project_dir = directory.unwrap_or_else(|| PathBuf::from("."));
    let project_name = name.unwrap_or_else(|| {
        project_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-project")
            .to_string()
    });

    fs::create_dir_all(&project_dir).map_err(|err| {
        vec![
            Diagnostic::error(
                Span::default(),
                format!("Failed to create '{}': {}", project_dir.display(), err),
            )
            .with_code(DiagnosticCode::IOError),
        ]
    })?;
    init_project(&project_dir, &project_name)?;

    if with_examples {
        init_examples(&project_dir)?;
    }

    println!("Initialized TestA project at {}", project_dir.display());
    Ok(())
}

pub fn init_project(project_dir: &Path, name: &str) -> Result<(), Vec<Diagnostic>> {
    fs::write(
        project_dir.join("README.md"),
        format!("# {}\n\nA TestA project.\n", name),
    )
    .map_err(|err| {
        vec![
            Diagnostic::error(
                Span::default(),
                format!("Failed to create 'README.md': {}", err),
            )
            .with_code(DiagnosticCode::IOError),
        ]
    })?;

    fs::write(
        project_dir.join("main.testa"),
        include_str!("../templates/main.testa"),
    )
    .map_err(|err| {
        vec![
            Diagnostic::error(
                Span::default(),
                format!("Failed to create 'main.testa': {}", err),
            )
            .with_code(DiagnosticCode::IOError),
        ]
    })?;
    Ok(())
}

pub fn init_examples(project_dir: &Path) -> Result<(), Vec<Diagnostic>> {
    let examples = project_dir.join("examples");
    fs::create_dir_all(&examples).map_err(|err| {
        vec![
            Diagnostic::error(
                Span::default(),
                format!("Failed to create directories: {}", err),
            )
            .with_code(DiagnosticCode::IOError),
        ]
    })?;

    fs::write(
        examples.join("simple.testa"),
        include_str!("../templates/simple.testa"),
    )
    .map_err(|err| {
        vec![
            Diagnostic::error(
                Span::default(),
                format!("Failed to create 'simple.testa': {}", err),
            )
            .with_code(DiagnosticCode::IOError),
        ]
    })?;
    fs::write(
        examples.join("advanced.testa"),
        include_str!("../templates/advanced.testa"),
    )
    .map_err(|err| {
        vec![
            Diagnostic::error(
                Span::default(),
                format!("Failed to create 'advanced.testa' {}", err),
            )
            .with_code(DiagnosticCode::IOError),
        ]
    })?;
    Ok(())
}
