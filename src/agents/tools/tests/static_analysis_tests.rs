use std::collections::BTreeSet;
use std::fs;

use tempfile::TempDir;

use crate::agents::tools::static_analysis::{
    StaticAnalyzer, executable_exists, select_analysis_commands, unavailable_analysis_summary,
};

fn availability(commands: &[&str]) -> impl Fn(&str) -> bool {
    let commands = commands.iter().copied().collect::<BTreeSet<_>>();
    move |command| commands.contains(command)
}

#[test]
fn static_analysis_auto_selects_installed_direct_linters() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname='demo'\nversion='0.1.0'\n",
    )
    .expect("Cargo.toml should be written");
    fs::write(temp_dir.path().join("pyproject.toml"), "[tool.ruff]\n")
        .expect("pyproject should be written");
    fs::write(temp_dir.path().join("package.json"), "{}\n").expect("package should be written");
    fs::write(temp_dir.path().join("go.mod"), "module demo\n").expect("go.mod should be written");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Auto,
        availability(&["cargo", "ruff", "biome", "golangci-lint", "go"]),
    );

    let names = commands
        .iter()
        .map(|command| command.name)
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "Rust clippy",
            "Python ruff",
            "JavaScript/TypeScript biome",
            "Go golangci-lint"
        ]
    );
}

#[test]
fn static_analysis_uses_oxlint_and_go_vet_fallbacks() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    fs::write(temp_dir.path().join("package.json"), "{}\n").expect("package should be written");
    fs::write(temp_dir.path().join("go.mod"), "module demo\n").expect("go.mod should be written");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Auto,
        availability(&["oxlint", "go"]),
    );

    assert_eq!(commands[0].executable, "oxlint");
    assert_eq!(commands[0].args, vec!["."]);
    assert_eq!(commands[1].executable, "go");
    assert_eq!(commands[1].args, vec!["vet", "./..."]);
}

#[test]
fn static_analysis_respects_requested_analyzer() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname='demo'\nversion='0.1.0'\n",
    )
    .expect("Cargo.toml should be written");
    fs::write(temp_dir.path().join("pyproject.toml"), "[tool.ruff]\n")
        .expect("pyproject should be written");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Python,
        availability(&["cargo", "ruff"]),
    );

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].name, "Python ruff");
}

#[test]
fn static_analysis_returns_empty_without_markers_or_commands() {
    let temp_dir = TempDir::new().expect("temp dir should be created");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Auto,
        availability(&["cargo", "ruff", "biome", "golangci-lint"]),
    );

    assert!(commands.is_empty());
}

#[test]
fn static_analysis_prefers_biome_over_oxlint() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    fs::write(temp_dir.path().join("package.json"), "{}\n").expect("package should be written");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Javascript,
        availability(&["biome", "oxlint"]),
    );

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].executable, "biome");
}

#[test]
fn static_analysis_prefers_golangci_lint_over_go_vet() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    fs::write(temp_dir.path().join("go.mod"), "module demo\n").expect("go.mod should be written");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Go,
        availability(&["golangci-lint", "go"]),
    );

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].executable, "golangci-lint");
}

#[test]
fn static_analysis_returns_empty_when_requested_command_is_missing() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname='demo'\nversion='0.1.0'\n",
    )
    .expect("Cargo.toml should be written");

    let commands =
        select_analysis_commands(temp_dir.path(), StaticAnalyzer::Rust, availability(&[]));

    assert!(commands.is_empty());
}

#[test]
fn static_analysis_explains_missing_commands_for_detected_projects() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname='demo'\nversion='0.1.0'\n",
    )
    .expect("Cargo.toml should be written");
    fs::write(temp_dir.path().join("package.json"), "{}\n").expect("package should be written");

    let notes =
        unavailable_analysis_summary(temp_dir.path(), StaticAnalyzer::Auto, availability(&[]));

    assert_eq!(
        notes,
        vec![
            "Cargo.toml detected but cargo is not on PATH.",
            "package.json detected but biome and oxlint are not on PATH.",
        ]
    );
}

#[test]
fn static_analysis_explains_auto_mode_without_project_markers() {
    let temp_dir = TempDir::new().expect("temp dir should be created");

    let notes =
        unavailable_analysis_summary(temp_dir.path(), StaticAnalyzer::Auto, availability(&[]));

    assert_eq!(
        notes,
        vec!["No matching project markers detected for auto mode."]
    );
}

#[test]
fn static_analysis_runs_explicit_python_when_ruff_is_installed() {
    let temp_dir = TempDir::new().expect("temp dir should be created");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Python,
        availability(&["ruff"]),
    );

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].executable, "ruff");
    assert_eq!(commands[0].reason, "Python analyzer requested");
}

#[test]
fn static_analysis_runs_explicit_rust_without_project_marker() {
    let temp_dir = TempDir::new().expect("temp dir should be created");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Rust,
        availability(&["cargo"]),
    );

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].executable, "cargo");
    assert_eq!(commands[0].reason, "Rust analyzer requested");
}

#[test]
fn static_analysis_runs_explicit_javascript_without_project_marker() {
    let temp_dir = TempDir::new().expect("temp dir should be created");

    let commands = select_analysis_commands(
        temp_dir.path(),
        StaticAnalyzer::Javascript,
        availability(&["oxlint"]),
    );

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].executable, "oxlint");
    assert_eq!(
        commands[0].reason,
        "JavaScript analyzer requested and oxlint is installed"
    );
}

#[test]
fn static_analysis_runs_explicit_go_without_project_marker() {
    let temp_dir = TempDir::new().expect("temp dir should be created");

    let commands =
        select_analysis_commands(temp_dir.path(), StaticAnalyzer::Go, availability(&["go"]));

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].executable, "go");
    assert_eq!(
        commands[0].reason,
        "Go analyzer requested and go is installed"
    );
}

#[cfg(unix)]
#[test]
fn static_analysis_ignores_non_executable_files_on_unix() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().expect("temp dir should be created");
    let tool_path = temp_dir.path().join("fake-tool");
    fs::write(&tool_path, "#!/bin/sh\n").expect("tool should be written");
    fs::set_permissions(&tool_path, fs::Permissions::from_mode(0o644))
        .expect("tool permissions should be set");

    assert!(!executable_exists(&tool_path));

    fs::set_permissions(&tool_path, fs::Permissions::from_mode(0o755))
        .expect("tool permissions should be set");

    assert!(executable_exists(&tool_path));
}
