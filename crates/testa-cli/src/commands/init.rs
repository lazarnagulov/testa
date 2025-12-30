use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn init_command(
    directory: Option<PathBuf>,
    name: Option<String>,
    with_examples: bool,
) -> Result<(), Box<dyn Error>> {
    let project_dir = directory.unwrap_or_else(|| PathBuf::from("."));
    let project_name = name.unwrap_or_else(|| {
        project_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-project")
            .to_string()
    });

    fs::create_dir_all(&project_dir)?;
    init_project(&project_dir, &project_name)?;

    if with_examples {
        init_examples(&project_dir)?;
    }

    println!("Initialized TestA project at {}", project_dir.display());
    Ok(())
}

pub fn init_project(project_dir: &Path, name: &str) -> Result<(), Box<dyn Error>> {
    fs::write(
        project_dir.join("README.md"),
        format!("# {}\n\nA TestA project.\n", name),
    )?;

    fs::write(
        project_dir.join("main.testa"),
        include_str!("../templates/main.testa"),
    )?;
    Ok(())
}

pub fn init_examples(project_dir: &Path) -> Result<(), Box<dyn Error>> {
    let examples = project_dir.join("examples");
    fs::create_dir_all(&examples)?;

    fs::write(
        examples.join("simple.testa"),
        include_str!("../templates/simple.testa"),
    )?;
    fs::write(
        examples.join("advanced.testa"),
        include_str!("../templates/advanced.testa"),
    )?;
    Ok(())
}
