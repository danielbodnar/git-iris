#![allow(clippy::unwrap_used)]

use std::fs;

const OPENAI_DOC_PATHS: &[&str] = &[
    "docs/configuration/providers.md",
    "docs/getting-started/configuration.md",
    "docs/reference/cli.md",
];

#[test]
fn openai_docs_prefer_reasoning_controls_over_legacy_max_tokens_examples() {
    for path in OPENAI_DOC_PATHS {
        let doc = fs::read_to_string(path).unwrap();
        assert!(
            !doc.contains("--param max_tokens="),
            "{path} still teaches legacy max_tokens CLI examples"
        );
        assert!(
            !doc.contains("max_tokens = \""),
            "{path} still teaches legacy max_tokens TOML examples"
        );
    }

    let providers_doc = fs::read_to_string("docs/configuration/providers.md").unwrap();
    assert!(providers_doc.contains("reasoning"));
    assert!(providers_doc.contains("verbosity"));
    assert!(providers_doc.contains("token-limit"));
    assert!(providers_doc.contains("\"effort\":\"medium\""));
    assert!(providers_doc.contains("\"effort\":\"low\""));
    assert!(providers_doc.contains("\"effort\":\"none\""));
}

#[test]
fn architecture_doc_matches_current_provider_and_gitmoji_wiring() {
    let doc = fs::read_to_string("docs/architecture/agent.md").unwrap();

    assert!(doc.contains("apply_completion_params"));
    assert!(doc.contains("CompletionProfile::MainAgent"));
    assert!(doc.contains("CompletionProfile::Subagent"));
    assert!(doc.contains("get_gitmoji_prompt_guide()"));
    assert!(!doc.contains(".max_tokens(16384)"));
    assert!(!doc.contains(".max_tokens(4096)"));
    assert!(!doc.contains("get_gitmoji_list()"));
}
