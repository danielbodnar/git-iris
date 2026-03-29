#![allow(clippy::unwrap_used)]

use std::fs;

const CAPABILITY_PATHS: &[&str] = &[
    "src/agents/capabilities/commit.toml",
    "src/agents/capabilities/review.toml",
    "src/agents/capabilities/pr.toml",
    "src/agents/capabilities/changelog.toml",
    "src/agents/capabilities/release_notes.toml",
];

#[test]
fn capability_prompts_do_not_force_context_as_the_first_tool_call() {
    for path in CAPABILITY_PATHS {
        let prompt = fs::read_to_string(path).unwrap();
        assert!(
            !prompt.contains("## MANDATORY FIRST STEP"),
            "{path} still forces a docs-first prompt contract"
        );
        assert!(
            !prompt.contains("ALWAYS call `project_docs(doc_type=\"context\")` FIRST"),
            "{path} still instructs Iris to call project_docs context first"
        );
        assert!(
            prompt.contains("project_docs(doc_type=\"context\")"),
            "{path} should still mention the compact project_docs context tool"
        );
    }
}
