use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use testa_core::{
    analyser::{SemanticAnalyser, result::AnalysisResult},
    ast::Program,
    diagnostics::Severity,
    lexer::Lexer,
    parser::Parser,
};

pub fn generate_command(
    input: PathBuf,
    _output: Option<PathBuf>,
    _format: Option<String>,
    _count: Option<usize>,
    _seed: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    compile_file(&input, false)?;

    // TODO: Generate
    Ok(())
}

pub fn check_command(
    files: Vec<PathBuf>,
    syntax_only: bool,
    warnings: bool,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        if syntax_only {
            parse_file(&file)?;
        } else {
            compile_file(&file, warnings)?;
        }
    }

    Ok(())
}

pub fn lsp_command(
    _stdio: bool,
    _port: Option<u16>,
    _log_file: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

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
    initialize_project(&project_dir, &project_name)?;
    if with_examples {
        initialize_examples(&project_dir)?;
    }
    println!("  Initialized TestA project at {}", project_dir.display());
    println!("  Created: README.md");
    println!("  Created: main.testa");
    if with_examples {
        println!("  Created: examples/simple.testa");
        println!("  Created: examples/advanced.testa");
    }
    Ok(())
}

pub fn info_command(extended: bool) {
    println!("TestA - Test Data Generator");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));

    if extended {
        println!();
        println!("Supported formats:");
        println!("  - CSV");
        println!();
        println!("Features:");
        println!("  - Template-based data generation");
        println!("  - Type constraints and validation");
        println!("  - Template inheritance");
        println!("  - Enum support with weighted variants");
        println!("  - LSP support for editor integration");
        println!();
        println!("Documentation: https://github.com/lazarnagulov/testa");
    }
}

fn parse_file(path: &PathBuf) -> Result<Program, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer, &content);
    Ok(parser.parse()?)
}

fn compile_file(path: &PathBuf, show_warning: bool) -> Result<Program, Box<dyn Error>> {
    let program = parse_file(path)?;
    let AnalysisResult { diagnostics, .. } = SemanticAnalyser::new(&program).analyse();

    let mut has_errors = false;
    for diag in &diagnostics {
        if diag.severity == Severity::Warning && !show_warning {
            continue;
        }
        eprintln!("{}", diag.format_cli());
        has_errors = true;
    }

    if has_errors {
        return Err(format!("Failed to compile {}", path.display()).into());
    }

    Ok(program)
}

fn initialize_project(project_dir: &Path, project_name: &str) -> Result<(), Box<dyn Error>> {
    let readme_path = project_dir.join("README.md");
    fs::write(
        &readme_path,
        format!(
            "# {}\n\nA TestA data generation project.\n\n## Usage\n\n```bash\ntesta generate main.testa -o output.csv\n```\n",
            project_name
        ),
    )?;
    let main_path = project_dir.join("main.testa");
    fs::write(
        &main_path,
        r#"@output csv;
@output_path "./output.csv";

template User {
    id = int [range=1..=1000];
    name = string;
    email = string;
    active = bool;
}

@generate User [10];
        "#,
    )?;
    Ok(())
}

fn initialize_examples(project_dir: &Path) -> Result<(), Box<dyn Error>> {
    let examples_dir = project_dir.join("examples");
    fs::create_dir_all(&examples_dir)?;

    fs::write(
        examples_dir.join("simple.testa"),
r#"@output csv;
@output_path "./simple.csv";

template Product {
    id = $uuid();
    name = string;
    price = float [range=10.0..=1000.0];
}

@generate Product [5];"#,
    )?;

    fs::write(
        examples_dir.join("advanced.testa"),
r#"@output csv;
@output_path "./advanced.csv";

enum Status {
    Active => 70;
    Inactive => 20;
    Pending => 10;
}

template User {
    id = int;
    username = string;
    status = Status;
}

template Admin : User {
    permissions = [string][count=1..=5];
    override status = "Active";
}

@generate Admin [3];
"#,
    )?;
    Ok(())
}
