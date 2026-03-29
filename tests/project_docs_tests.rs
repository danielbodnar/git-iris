#![allow(clippy::unwrap_used)]

use git_iris::agents::tools::{
    ProjectDocs,
    docs::{DocType, ProjectDocsArgs},
    with_active_repo_root,
};
use rig::tool::Tool;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn project_docs_context_returns_a_compact_snapshot() {
    let temp_dir = TempDir::new().unwrap();
    let readme = format!("# Example Repo\n\n{}\n", "A".repeat(7_500));
    let agents = format!("# Agent Rules\n\n{}\n", "B".repeat(9_000));

    fs::write(temp_dir.path().join("README.md"), readme).unwrap();
    fs::write(temp_dir.path().join("AGENTS.md"), agents).unwrap();

    let output = with_active_repo_root(temp_dir.path(), async {
        ProjectDocs
            .call(ProjectDocsArgs {
                doc_type: DocType::Context,
                max_chars: 20_000,
            })
            .await
            .unwrap()
    })
    .await;

    assert!(output.contains("Project context snapshot."));
    assert!(output.contains("=== README.md ==="));
    assert!(output.contains("=== AGENTS.md ==="));
    assert!(output.contains("project_docs(doc_type=\"readme\")"));
    assert!(output.contains("project_docs(doc_type=\"agents\")"));
    assert!(output.contains("context snapshot truncated"));
    assert!(output.chars().count() < 9_500);
}
