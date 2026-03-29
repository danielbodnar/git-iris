//! Tests for review functionality
//!
//! Note: Legacy `GeneratedReview` tests removed. `MarkdownReview` is now the active code path.

use git_iris::agents::TaskContext;

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

    let to_only = TaskContext::for_review(None, None, Some("feature".to_string()), false)
        .expect("should succeed");
    assert!(
        matches!(to_only, TaskContext::Range { from, to } if from == "main" && to == "feature")
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
