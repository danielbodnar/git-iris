use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;

use crate::agents::tools::repo_map::{RepoMapArgs, RepoMapTool};

#[test]
fn repo_map_ranks_mentioned_files_and_extracts_symbols() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("src dir should be created");
    fs::write(
        src_dir.join("lib.rs"),
        "use crate::agents::IrisAgent;\n\npub struct RepoMapTool;\npub fn build_map() {}\n",
    )
    .expect("lib file should be written");
    fs::write(src_dir.join("tiny.rs"), "pub fn helper() {}\n")
        .expect("tiny file should be written");

    let map = RepoMapTool::build(
        temp_dir.path(),
        &RepoMapArgs {
            token_budget: 500,
            mentioned_files: vec![PathBuf::from("src/lib.rs")],
            max_files: 10,
        },
    )
    .expect("repo map should build");

    assert_eq!(map.files_analyzed, 2);
    assert!(map.content.contains("src/lib.rs [mentioned]"));
    assert!(map.content.contains("defs: RepoMapTool, build_map"));
    assert!(map.content.contains("refs: crate::agents::IrisAgent"));
}

#[test]
fn repo_map_respects_output_budget() {
    let temp_dir = TempDir::new().expect("temp dir should be created");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("src dir should be created");

    for index in 0..20 {
        fs::write(
            src_dir.join(format!("file_{index}.rs")),
            format!("pub struct File{index};\npub fn function_{index}() {{}}\n"),
        )
        .expect("source file should be written");
    }

    let map = RepoMapTool::build(
        temp_dir.path(),
        &RepoMapArgs {
            token_budget: 50,
            mentioned_files: Vec::new(),
            max_files: 20,
        },
    )
    .expect("repo map should build");

    assert!(map.content.chars().count() <= 225);
    assert!(map.content.contains("[repo_map truncated]"));
}
