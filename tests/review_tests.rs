//! Tests for review functionality
//!
//! Note: Legacy `GeneratedReview` tests removed. `Review` is now the active code path.

use git_iris::agents::TaskContext;
use git_iris::{Category, EvidenceRef, Finding, FindingId, Review, ReviewStats, Severity};
use std::path::PathBuf;

fn sample_finding() -> Finding {
    Finding {
        id: FindingId("finding-1".to_string()),
        severity: Severity::High,
        confidence: 86,
        file: PathBuf::from("src/auth.rs"),
        start_line: 42,
        end_line: 44,
        category: Category::Security,
        title: "Missing authorization check".to_string(),
        body: "The changed handler accepts user input before checking access.".to_string(),
        suggested_fix: Some("Check authorization before processing the request.".to_string()),
        evidence: vec![EvidenceRef {
            file: PathBuf::from("src/auth.rs"),
            line: 42,
            end_line: Some(44),
            note: Some("changed handler".to_string()),
        }],
    }
}

fn low_confidence_finding() -> Finding {
    Finding {
        confidence: 42,
        severity: Severity::Medium,
        id: FindingId("finding-2".to_string()),
        file: PathBuf::from("src/auth.rs"),
        start_line: 50,
        end_line: 50,
        category: Category::Testing,
        title: "Possible missing test".to_string(),
        body: "This is too speculative to publish.".to_string(),
        suggested_fix: None,
        evidence: Vec::new(),
    }
}

fn overconfident_finding() -> Finding {
    Finding {
        confidence: 200,
        ..sample_finding()
    }
}

fn finding_from_confidence(confidence: impl Into<serde_json::Value>) -> Finding {
    let confidence = confidence.into();
    serde_json::from_value(serde_json::json!({
        "id": "finding-1",
        "severity": "high",
        "confidence": confidence,
        "file": "src/auth.rs",
        "start_line": 42,
        "end_line": 42,
        "category": "security",
        "title": "Missing authorization check",
        "body": "The changed handler accepts user input before checking access."
    }))
    .expect("finding should deserialize")
}

#[test]
fn structured_review_renders_markdown_from_findings() {
    let finding = sample_finding();
    let review = Review {
        summary: "Adds an auth handler.".to_string(),
        findings: vec![finding],
        stats: ReviewStats::default(),
        parse_failed: false,
    };

    let markdown = review.raw_content();

    assert!(markdown.contains("# Code Review"));
    assert!(markdown.contains("Adds an auth handler."));
    assert!(markdown.contains("[HIGH] **Missing authorization check in `src/auth.rs:42-44`**"));
    assert!(markdown.contains("Category: security. Confidence: 86%."));
    assert!(markdown.contains("Evidence: src/auth.rs:42-44 (changed handler)"));
}

#[test]
fn review_stats_are_derived_when_model_counts_are_missing() {
    let review = Review {
        summary: String::new(),
        findings: vec![sample_finding()],
        stats: ReviewStats::default(),
        parse_failed: false,
    };

    let stats = review.effective_stats();

    assert_eq!(stats.findings_count, 1);
    assert_eq!(stats.high_count, 1);
}

#[test]
fn low_confidence_findings_do_not_render() {
    let review = Review {
        summary: String::new(),
        findings: vec![low_confidence_finding()],
        stats: ReviewStats::default(),
        parse_failed: false,
    };

    let markdown = review.raw_content();

    assert!(markdown.contains("Found 0 issue(s)"));
    assert!(markdown.contains("No blocking issues found."));
    assert!(!markdown.contains("Possible missing test"));
}

#[test]
fn confidence_scores_are_clamped_for_rendering_and_gates() {
    let review = Review {
        summary: String::new(),
        findings: vec![overconfident_finding()],
        stats: ReviewStats::default(),
        parse_failed: false,
    };

    let markdown = review.raw_content();

    assert!(markdown.contains("Confidence: 100%."));
    assert_eq!(review.visible_findings_at(101).len(), 0);
}

#[test]
fn category_accepts_common_aliases() {
    for (raw, expected) in [
        ("\"error-handling\"", Category::ErrorHandling),
        ("\"errorHandling\"", Category::ErrorHandling),
        ("\"ERROR HANDLING\"", Category::ErrorHandling),
        ("\"Security\"", Category::Security),
        ("\"performance \"", Category::Performance),
        ("\"api contract\"", Category::ApiContract),
        ("\"surprise\"", Category::Other),
    ] {
        let category: Category = serde_json::from_str(raw).expect("category should deserialize");
        assert_eq!(category, expected);
    }
}

#[test]
fn unstructured_review_fallback_does_not_claim_success() {
    let review = Review::from_unstructured("{bad json}\n```nested```");

    let markdown = review.raw_content();

    assert!(markdown.contains("Review parsing failed"));
    assert!(markdown.contains("{bad json}"));
    assert!(markdown.contains("```text"));
    assert!(markdown.contains("`\\`\\`nested`\\`\\`"));
    assert!(!markdown.contains("No blocking issues found."));
    assert!(!markdown.contains("Found 0 issue(s)"));
}

#[test]
fn parse_failed_reviews_round_trip() {
    let review = Review::from_unstructured("{bad json}");

    let encoded = serde_json::to_string(&review).expect("review should serialize");
    let decoded: Review = serde_json::from_str(&encoded).expect("review should deserialize");

    assert!(decoded.parse_failed);
    assert!(!decoded.raw_content().contains("No blocking issues found."));
}

#[test]
fn confidence_accepts_common_model_shapes() {
    assert_eq!(
        finding_from_confidence(serde_json::json!(0.85)).confidence,
        85
    );
    assert_eq!(
        finding_from_confidence(serde_json::json!("95.0")).confidence,
        95
    );
    assert_eq!(
        finding_from_confidence(serde_json::json!("72%")).confidence,
        72
    );
    assert_eq!(finding_from_confidence(serde_json::json!(1)).confidence, 1);
    assert_eq!(
        finding_from_confidence(serde_json::json!(250)).confidence,
        100
    );
    assert_eq!(finding_from_confidence(serde_json::json!(-5)).confidence, 0);
}

#[test]
fn test_branch_parameter_validation() {
    let staged = TaskContext::for_review(None, None, None, false).expect("should succeed");
    assert!(matches!(
        staged,
        TaskContext::Staged {
            include_unstaged: false
        }
    ));

    let staged_with_unstaged =
        TaskContext::for_review(None, None, None, true).expect("should succeed");
    assert!(matches!(
        staged_with_unstaged,
        TaskContext::Staged {
            include_unstaged: true
        }
    ));

    let commit = TaskContext::for_review(Some("abc123".to_string()), None, None, false)
        .expect("should succeed");
    assert!(matches!(commit, TaskContext::Commit { commit_id } if commit_id == "abc123"));

    let explicit_range = TaskContext::for_review(
        None,
        Some("main".to_string()),
        Some("feature".to_string()),
        false,
    )
    .expect("should succeed");
    assert!(
        matches!(explicit_range, TaskContext::Range { from, to } if from == "main" && to == "feature")
    );

    let to_only =
        TaskContext::for_review_with_base(None, None, Some("feature".to_string()), false, "trunk")
            .expect("should succeed");
    assert!(
        matches!(to_only, TaskContext::Range { from, to } if from == "trunk" && to == "feature")
    );

    let from_only = TaskContext::for_review(None, Some("main".to_string()), None, false);
    assert!(from_only.is_err());
    assert!(
        from_only
            .expect_err("should fail")
            .to_string()
            .contains("When using --from, you must also specify --to")
    );

    let commit_with_range = TaskContext::for_review(
        Some("abc123".to_string()),
        Some("main".to_string()),
        Some("feature".to_string()),
        false,
    );
    assert!(commit_with_range.is_err());
    assert!(
        commit_with_range
            .expect_err("should fail")
            .to_string()
            .contains("mutually exclusive")
    );

    let to_with_commit = TaskContext::for_review(
        Some("abc123".to_string()),
        None,
        Some("feature".to_string()),
        false,
    );
    assert!(to_with_commit.is_err());

    let unstaged_with_range = TaskContext::for_review(
        None,
        Some("main".to_string()),
        Some("feature".to_string()),
        true,
    );
    assert!(unstaged_with_range.is_err());
    assert!(
        unstaged_with_range
            .expect_err("should fail")
            .to_string()
            .contains("include-unstaged")
    );

    let unstaged_with_to_only =
        TaskContext::for_review(None, None, Some("feature".to_string()), true);
    assert!(unstaged_with_to_only.is_err());
}
