use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, BufWriter, Write, Error};
use std::path::Path;
use std::process::Command as ProcessCommand;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: crust-trust <project> <crates:dependencies>");
        std::process::exit(1);
    }

    let project_name = &args[1];
    let crate_args = &args[2..];

    let workspace_path = Path::new(project_name);
    create_workspace(workspace_path)?;

    let crates_data = Arc::new(Mutex::new(collect_crates_data(crate_args.iter())));

    let handles: Vec<_> = crates_data.lock().unwrap()
        .clone()
        .into_iter()
        .map(|(crate_name, dependencies)| {
            let workspace_path = workspace_path.to_path_buf();
            let crates_data = Arc::clone(&crates_data);
            thread::spawn(move || {
                if create_crate_with_dependencies(&workspace_path, &crate_name, &dependencies).is_err() {
                    eprintln!("Error creating crate '{}'", crate_name);
                }
                add_crate_to_workspace(&workspace_path, &crate_name).unwrap();
                communicate_between_crates(crates_data, crate_name);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked while creating crates");
    }

    run_cargo_check(workspace_path)?;
    optimize_dependencies(workspace_path)?;
    version_management(workspace_path)?;
    run_benchmark(workspace_path)?;
    generate_dependency_graph(workspace_path)?;

    println!("Workspace '{}' created successfully!", project_name);
    Ok(())
}

/// Creates the workspace and its Cargo.toml if it doesn't already exist.
fn create_workspace(path: &Path) -> Result<(), Error> {
    create_dir_all(path)?;
    let cargo_toml_path = path.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        create_file(&cargo_toml_path, "[workspace]\nmembers = []\n")?;
        println!("Created new workspace at '{}'.", path.display());
    }
    Ok(())
}

/// Collects the crate names and dependencies from user input, or provides default.
fn collect_crates_data<'a>(crates_input: impl Iterator<Item = &'a String>) -> HashMap<String, Vec<String>> {
    let mut crates_data = HashMap::new();
    let crates_input: Vec<&'a String> = crates_input.collect();
    if crates_input.is_empty() {
        crates_data.insert("default_project".to_string(), vec![]);
    } else {
        for crate_input in crates_input {
            if let Some((crate_name, deps)) = crate_input.split_once(':') {
                crates_data.insert(crate_name.to_string(), deps.split(',').map(String::from).collect());
            }
        }
    }
    crates_data
}

/// Creates a crate with dependencies in the workspace and updates the workspace Cargo.toml.
fn create_crate_with_dependencies(
    workspace_path: &Path,
    crate_name: &str,
    dependencies: &[String],
) -> Result<(), Error> {
    let crate_path = workspace_path.join(crate_name);
    create_dir_all(&crate_path)?;

    let dependencies_toml = dependencies.iter().map(|dep| format!("{} = \"*\"\n", dep)).collect::<String>();
    let crate_toml_content = format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n{}", crate_name, dependencies_toml);
    create_file(&crate_path.join("Cargo.toml"), &crate_toml_content)?;

    create_dir_all(crate_path.join("src"))?;
    create_file(&crate_path.join("src/lib.rs"), "// Default lib.rs content\n")?;
    Ok(())
}

/// Add the crate to the workspace Cargo.toml.
fn add_crate_to_workspace(workspace_path: &Path, crate_name: &str) -> Result<(), Error> {
    let workspace_toml_path = workspace_path.join("Cargo.toml");
    let mut workspace_toml = std::fs::read_to_string(&workspace_toml_path)?;

    if let Some(members_line) = workspace_toml.lines().find(|line| line.trim().starts_with("members = [")) {
        let updated_members = if members_line.trim() == "members = [" {
            format!("members = [\"{}\"]", crate_name)
        } else {
            format!("{}, \"{}\"]", members_line.trim_end_matches(']'), crate_name)
        };
        workspace_toml = workspace_toml.replace(members_line, &updated_members);
    }
    create_file(&workspace_toml_path, &workspace_toml)?;
    Ok(())
}

/// Runs `cargo check` to verify the workspace setup.
fn run_cargo_check(workspace_path: &Path) -> Result<(), Error> {
    let output = ProcessCommand::new("cargo").arg("check").arg("--workspace").current_dir(workspace_path).output()?;
    if !output.status.success() {
        eprintln!("`cargo check` failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    println!("`cargo check` passed.");
    Ok(())
}

/// Optimizes dependencies across crates to avoid duplication and inconsistencies.
fn optimize_dependencies(workspace_path: &Path) -> Result<(), Error> {
    let mut shared_dependencies = HashSet::new();
    for entry in std::fs::read_dir(workspace_path)?.filter_map(Result::ok).filter(|e| e.path().is_dir()) {
        if let Ok(file) = File::open(entry.path().join("Cargo.toml")) {
            for line in BufReader::new(file).lines().filter_map(Result::ok) {
                if let Some(dep) = line.split('=').next() {
                    shared_dependencies.insert(dep.trim().to_string());
                }
            }
        }
    }
    println!("Shared dependencies: {:?}", shared_dependencies);
    Ok(())
}

/// Simulates inter-crate communication or sharing of components.
fn communicate_between_crates(crates_data: Arc<Mutex<HashMap<String, Vec<String>>>>, crate_name: String) {
    let crates_data = crates_data.lock().unwrap();
    println!(
        "Crate '{}' is communicating with other crates: {:?}",
        crate_name,
        crates_data.keys().collect::<Vec<&String>>()
    );
}

/// Automatic dependency version management.
fn version_management(workspace_path: &Path) -> Result<(), Error> {
    println!("Running version management...");
    let output = ProcessCommand::new("cargo").arg("update").current_dir(workspace_path).output()?;
    if !output.status.success() {
        eprintln!("Version management failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    println!("Dependency versions updated.");
    Ok(())
}

/// Runs benchmarks on the entire workspace.
fn run_benchmark(workspace_path: &Path) -> Result<(), Error> {
    println!("Running `cargo bench`...");
    let output = ProcessCommand::new("cargo").arg("bench").current_dir(workspace_path).output()?;
    if !output.status.success() {
        eprintln!("`cargo bench` failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    println!("Benchmarking completed.");
    Ok(())
}

/// Generates a dependency graph for the entire workspace.
fn generate_dependency_graph(workspace_path: &Path) -> Result<(), Error> {
    println!("Generating dependency graph...");
    let output = ProcessCommand::new("cargo").arg("metadata").arg("--format-version=1").current_dir(workspace_path).output()?;
    if !output.status.success() {
        eprintln!("Failed to generate dependency graph: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    let metadata = String::from_utf8_lossy(&output.stdout);
    println!("Dependency Graph: {}", metadata);
    Ok(())
}

/// Creates or overwrites a file with the specified content.
fn create_file(path: &Path, content: &str) -> Result<(), Error> {
    let mut writer = BufWriter::new(File::create(path)?);
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    Ok(())
}
