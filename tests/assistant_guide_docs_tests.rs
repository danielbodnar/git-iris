#![allow(clippy::unwrap_used)]

use std::fs;

const GUIDE_PATHS: &[&str] = &["AGENTS.md", "CLAUDE.md"];

#[test]
fn assistant_guides_describe_reducer_architecture_without_pure_reducer_claims() {
    for path in GUIDE_PATHS {
        let doc = fs::read_to_string(path).unwrap();
        assert!(
            !doc.contains("Pure Reducer"),
            "{path} still claims Studio is a pure reducer architecture"
        );
        assert!(
            !doc.contains("Pure state transitions and side effects"),
            "{path} still describes reducer.rs with stale pure-reducer wording"
        );
        assert!(
            doc.contains("Reducer-Centric Event Flow"),
            "{path} should describe Studio as reducer-centric"
        );
    }
}

#[test]
fn assistant_guides_document_openai_reasoning_defaults() {
    for path in GUIDE_PATHS {
        let doc = fs::read_to_string(path).unwrap();
        assert!(
            doc.contains("medium reasoning"),
            "{path} should mention main-agent reasoning"
        );
        assert!(
            doc.contains("low reasoning"),
            "{path} should mention subagent reasoning"
        );
        assert!(
            doc.contains("status messages use none"),
            "{path} should mention status-message reasoning"
        );
    }
}
