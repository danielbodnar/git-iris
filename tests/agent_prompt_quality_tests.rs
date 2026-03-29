#![allow(clippy::unwrap_used)]

const COMMIT_PROMPT: &str = include_str!("../src/agents/capabilities/commit.toml");
const REVIEW_PROMPT: &str = include_str!("../src/agents/capabilities/review.toml");
const PR_PROMPT: &str = include_str!("../src/agents/capabilities/pr.toml");
const CHANGELOG_PROMPT: &str = include_str!("../src/agents/capabilities/changelog.toml");
const RELEASE_NOTES_PROMPT: &str = include_str!("../src/agents/capabilities/release_notes.toml");
const CHAT_PROMPT: &str = include_str!("../src/agents/capabilities/chat.toml");
const IRIS_SOURCE: &str = include_str!("../src/agents/iris.rs");

fn prompts() -> [(&'static str, &'static str); 6] {
    [
        ("commit", COMMIT_PROMPT),
        ("review", REVIEW_PROMPT),
        ("pr", PR_PROMPT),
        ("changelog", CHANGELOG_PROMPT),
        ("release_notes", RELEASE_NOTES_PROMPT),
        ("chat", CHAT_PROMPT),
    ]
}

#[test]
fn capability_prompts_no_longer_force_project_docs_as_the_first_tool_call() {
    for (name, prompt) in prompts() {
        assert!(
            !prompt.contains("## MANDATORY FIRST STEP"),
            "{name} still has a mandatory first-step docs block"
        );
        assert!(
            !prompt.contains("ALWAYS call `project_docs(doc_type=\"context\")` FIRST"),
            "{name} still forces `project_docs(doc_type=\"context\")` as the first call"
        );
    }
}

#[test]
fn capability_prompts_describe_context_as_compact_and_targeted() {
    for (name, prompt) in prompts() {
        assert!(
            prompt.contains("compact"),
            "{name} should describe project_docs context as compact"
        );
        assert!(
            prompt.contains("project_docs(doc_type=\"context\")"),
            "{name} should keep the context tool available"
        );
    }
}

#[test]
fn iris_preamble_prefers_git_evidence_before_repo_docs() {
    assert!(IRIS_SOURCE.contains("- Use git_diff to get changes first - it includes file content"));
    assert!(IRIS_SOURCE.contains(
        "- Use project_docs when repository conventions or product framing matter; do not front-load docs if the diff already answers the question"
    ));
}
