use std::path::PathBuf;

use crate::types::{Category, Finding, FindingId, Review, ReviewStats, Severity};

use super::{
    GitHubRepository, extract_inline_comment_candidates,
    extract_structured_inline_comment_candidates, parse_reviewable_lines,
};

#[test]
fn parses_https_remote() {
    let repo = GitHubRepository::parse("https://github.com/hyperb1iss/git-iris.git")
        .expect("https GitHub remote should parse");

    assert_eq!(repo.owner, "hyperb1iss");
    assert_eq!(repo.name, "git-iris");
}

#[test]
fn parses_ssh_remote() {
    let repo = GitHubRepository::parse("git@github.com:hyperb1iss/git-iris.git")
        .expect("ssh GitHub remote should parse");

    assert_eq!(repo.owner, "hyperb1iss");
    assert_eq!(repo.name, "git-iris");
}

#[test]
fn rejects_non_github_remote() {
    let err = GitHubRepository::parse("https://gitlab.com/hyperb1iss/git-iris.git")
        .expect_err("non-GitHub remotes should be rejected");

    assert!(err.to_string().contains("Only github.com"));
}

#[test]
fn extracts_review_findings_with_locations() {
    let review = r"
## Issues

- [HIGH] **Missing error handling in `src/github.rs:42`**
  This can drop the GitHub failure context.
  **Fix**: Add context before returning.

- [LOW] **Docs typo in docs/user-guide/reviews.md:12**
  Minor polish.
";

    let candidates = extract_inline_comment_candidates(review);

    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[0].path, "src/github.rs");
    assert_eq!(candidates[0].line, 42);
    assert!(candidates[0].body.contains("Missing error handling"));
    assert_eq!(candidates[1].path, "docs/user-guide/reviews.md");
    assert_eq!(candidates[1].line, 12);
}

#[test]
fn extracts_structured_review_findings() {
    let review = Review {
        summary: "Review summary".to_string(),
        findings: vec![Finding {
            id: FindingId("finding-1".to_string()),
            severity: Severity::High,
            confidence: 91,
            file: PathBuf::from("src/github.rs"),
            start_line: 42,
            end_line: 42,
            category: Category::ErrorHandling,
            title: "Missing error context".to_string(),
            body: "The changed path drops useful context.".to_string(),
            suggested_fix: Some("Add context before returning.".to_string()),
            evidence: Vec::new(),
        }],
        stats: ReviewStats::default(),
    };

    let candidates = extract_structured_inline_comment_candidates(&review);

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].path, "src/github.rs");
    assert_eq!(candidates[0].line, 42);
    assert!(
        candidates[0]
            .body
            .contains("[HIGH] **Missing error context**")
    );
    assert!(candidates[0].body.contains("Confidence: 91%"));
}

#[test]
fn parses_reviewable_lines_from_unified_diff() {
    let diff = r"
diff --git a/src/github.rs b/src/github.rs
index 1111111..2222222 100644
--- a/src/github.rs
+++ b/src/github.rs
@@ -40,6 +40,7 @@ impl GitHubClient {
 context line
-old line
+new line
 another context line
diff --git a/docs/user-guide/reviews.md b/docs/user-guide/reviews.md
index 3333333..4444444 100644
--- a/docs/user-guide/reviews.md
+++ b/docs/user-guide/reviews.md
@@ -10,2 +10,3 @@
 docs context
+new docs line
";

    let lines = parse_reviewable_lines(diff);

    assert!(lines["src/github.rs"].contains(&40));
    assert!(lines["src/github.rs"].contains(&41));
    assert!(lines["src/github.rs"].contains(&42));
    assert!(lines["docs/user-guide/reviews.md"].contains(&10));
    assert!(lines["docs/user-guide/reviews.md"].contains(&11));
    assert!(!lines["src/github.rs"].contains(&39));
}
