#![allow(clippy::unwrap_used)]

use std::fs;

const DOC_PATHS: &[&str] = &[
    "docs/architecture/index.md",
    "docs/architecture/capabilities.md",
    "docs/architecture/tools.md",
    "docs/extending/capabilities.md",
    "docs/extending/contributing.md",
];

#[test]
fn docs_no_longer_teach_project_docs_context_as_a_mandatory_first_step() {
    for path in DOC_PATHS {
        let doc = fs::read_to_string(path).unwrap();
        assert!(
            !doc.contains("ALWAYS call `project_docs(doc_type=\"context\")` FIRST"),
            "{path} still teaches the old docs-first prompt contract"
        );
    }
}

#[test]
fn docs_describe_project_docs_context_as_compact_and_targeted() {
    for path in DOC_PATHS {
        let doc = fs::read_to_string(path).unwrap();
        assert!(
            doc.contains("compact") || doc.contains("concise"),
            "{path} should describe docs context as compact or concise"
        );
    }

    let tools_doc = fs::read_to_string("docs/architecture/tools.md").unwrap();
    assert!(!tools_doc.contains("- `claude` —"));
    assert!(tools_doc.contains("- `context` — A concise README + agent-instructions summary"));
}
