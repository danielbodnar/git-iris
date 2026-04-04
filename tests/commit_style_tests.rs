mod context {
    pub use git_iris::context::*;
}

#[path = "../src/agents/commit_style.rs"]
mod commit_style;

use commit_style::detect_conventional_commit_style;
use git_iris::context::RecentCommit;

fn commit(message: &str) -> RecentCommit {
    RecentCommit {
        hash: "abc1234".to_string(),
        message: message.to_string(),
        author: "Bliss".to_string(),
        timestamp: "2026-04-04T00:00:00Z".to_string(),
    }
}

#[test]
fn detects_conventional_history_with_scopes() {
    let commits = vec![
        commit("feat(studio): add release panel"),
        commit("fix(agent): avoid duplicate prompts"),
        commit("docs: update quick start"),
        commit("chore: bump rig-core"),
        commit("Merge branch 'main' into feature/release"),
    ];

    let detection =
        detect_conventional_commit_style(&commits).expect("should detect conventional history");

    assert_eq!(
        detection.examples(),
        &[
            "feat(studio): add release panel".to_string(),
            "fix(agent): avoid duplicate prompts".to_string(),
            "docs: update quick start".to_string(),
        ]
    );
}

#[test]
fn does_not_treat_gitmoji_history_as_conventional() {
    let commits = vec![
        commit("✨ Add release notes contributor list"),
        commit("🐛 Fix duplicate gitmoji output"),
        commit("📝 Update documentation"),
        commit("♻️ Refactor commit service"),
    ];

    assert!(detect_conventional_commit_style(&commits).is_none());
}

#[test]
fn requires_high_confidence_before_detecting_conventional_history() {
    let commits = vec![
        commit("feat: add release notes contributor list"),
        commit("docs: update CLI guide"),
        commit("✨ Add commit style detector"),
        commit("Merge branch 'main' into feature/detector"),
        commit("Improve release note wording"),
    ];

    assert!(detect_conventional_commit_style(&commits).is_none());
}
