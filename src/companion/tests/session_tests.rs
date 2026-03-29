use crate::companion::SessionState;
use chrono::{Duration, Utc};
use std::path::PathBuf;
use tempfile::TempDir;

fn make_session() -> (TempDir, SessionState) {
    let repo_dir = tempfile::tempdir().expect("temp repo");
    let session = SessionState::new(repo_dir.path().to_path_buf(), "main".to_string());
    (repo_dir, session)
}

#[test]
fn touch_file_normalizes_absolute_paths_to_repo_relative() {
    let (repo_dir, mut session) = make_session();
    let absolute = repo_dir.path().join("src/lib.rs");
    let relative = PathBuf::from("src/lib.rs");

    session.touch_file(absolute);
    session.touch_file(relative);

    assert_eq!(session.files_count(), 1);

    let files = session.recent_files();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, PathBuf::from("src/lib.rs"));
    assert_eq!(files[0].touch_count, 2);
}

#[test]
fn set_branch_resets_session_scoped_state() {
    let (_repo_dir, mut session) = make_session();
    let original_session_id = session.session_id;
    let original_started_at = session.started_at;

    session.touch_file(PathBuf::from("src/lib.rs"));
    session.record_commit("abc1234".to_string());
    session.set_branch("feature/refactor".to_string());

    assert_eq!(session.branch, "feature/refactor");
    assert_ne!(session.session_id, original_session_id);
    assert!(session.started_at >= original_started_at);
    assert!(session.files_touched.is_empty());
    assert!(session.commits_made.is_empty());
    assert!(session.last_commit_at.is_none());
}

#[test]
fn time_since_last_commit_uses_the_commit_timestamp() {
    let (_repo_dir, mut session) = make_session();
    session.last_commit_at = Some(Utc::now() - Duration::minutes(12));

    session.touch_file(PathBuf::from("src/lib.rs"));

    let elapsed = session
        .time_since_last_commit()
        .expect("commit timestamp should be tracked");

    assert!(elapsed.num_minutes() >= 11);
}
