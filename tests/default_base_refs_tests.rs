use git_iris::Config;
use git_iris::agents::TaskContext;
use git_iris::git::GitRepo;
use git_iris::studio::{Mode, StudioState};
use git2::{BranchType, Repository, build::CheckoutBuilder};
use std::sync::Arc;

#[path = "test_utils.rs"]
mod test_utils;
use test_utils::setup_git_repo;

fn rename_main_to_trunk(repo: &Repository) {
    let head_commit = repo
        .head()
        .expect("HEAD should exist")
        .peel_to_commit()
        .expect("HEAD should resolve to a commit");

    repo.branch("trunk", &head_commit, false)
        .expect("should create trunk branch");
    repo.set_head("refs/heads/trunk")
        .expect("should switch HEAD to trunk");
    repo.checkout_head(Some(CheckoutBuilder::default().force()))
        .expect("should check out trunk");

    let mut main_branch = repo
        .find_branch("main", BranchType::Local)
        .expect("main branch should exist");
    main_branch.delete().expect("should delete main branch");
}

fn create_and_checkout_branch(repo: &Repository, name: &str) {
    let head_commit = repo
        .head()
        .expect("HEAD should exist")
        .peel_to_commit()
        .expect("HEAD should resolve to a commit");

    repo.branch(name, &head_commit, false)
        .expect("should create branch");

    let branch = repo
        .find_branch(name, BranchType::Local)
        .expect("branch should exist");
    let branch_name = branch.get().name().expect("branch ref should be valid");
    repo.set_head(branch_name)
        .expect("should switch HEAD to feature branch");
    repo.checkout_head(Some(CheckoutBuilder::default().force()))
        .expect("should check out feature branch");
}

#[test]
fn git_repo_default_base_ref_supports_trunk_repositories() {
    let (temp_dir, _) = setup_git_repo();
    let repo = Repository::open(temp_dir.path()).expect("should open temp repo");

    rename_main_to_trunk(&repo);
    create_and_checkout_branch(&repo, "feature/neon");

    let git_repo = GitRepo::new(temp_dir.path()).expect("should create GitRepo");
    assert_eq!(
        git_repo
            .get_default_base_ref()
            .expect("should resolve base"),
        "trunk"
    );
}

#[test]
fn studio_state_uses_primary_branch_defaults_on_feature_branches() {
    let (temp_dir, _) = setup_git_repo();
    let repo = Repository::open(temp_dir.path()).expect("should open temp repo");

    rename_main_to_trunk(&repo);
    create_and_checkout_branch(&repo, "feature/neon");

    let git_repo = Arc::new(GitRepo::new(temp_dir.path()).expect("should create GitRepo"));
    let state = StudioState::new(Config::default(), Some(git_repo));

    assert_eq!(state.modes.pr.base_branch, "trunk");
    assert_eq!(state.modes.review.from_ref, "trunk");
    assert_eq!(state.modes.review.to_ref, "HEAD");
}

#[test]
fn studio_state_keeps_primary_branch_review_on_last_commit() {
    let (temp_dir, _) = setup_git_repo();
    let repo = Repository::open(temp_dir.path()).expect("should open temp repo");

    rename_main_to_trunk(&repo);

    let git_repo = Arc::new(GitRepo::new(temp_dir.path()).expect("should create GitRepo"));
    let state = StudioState::new(Config::default(), Some(git_repo));

    assert_eq!(state.modes.pr.base_branch, "trunk");
    assert_eq!(state.modes.review.from_ref, "HEAD~1");
    assert_eq!(state.modes.review.to_ref, "HEAD");
}

#[test]
fn studio_branch_picker_prioritizes_the_resolved_primary_branch() {
    let (temp_dir, _) = setup_git_repo();
    let repo = Repository::open(temp_dir.path()).expect("should open temp repo");

    rename_main_to_trunk(&repo);
    create_and_checkout_branch(&repo, "feature/neon");

    let git_repo = Arc::new(GitRepo::new(temp_dir.path()).expect("should create GitRepo"));
    let state = StudioState::new(Config::default(), Some(git_repo));
    let refs = state.get_branch_refs();

    assert_eq!(refs.first().map(String::as_str), Some("trunk"));
    assert!(refs.iter().any(|reference| reference == "feature/neon"));
}

#[test]
fn studio_suggests_pr_mode_when_feature_branch_is_ahead_of_primary() {
    let (temp_dir, _) = setup_git_repo();
    let repo = Repository::open(temp_dir.path()).expect("should open temp repo");

    rename_main_to_trunk(&repo);
    create_and_checkout_branch(&repo, "feature/neon");

    let git_repo = Arc::new(GitRepo::new(temp_dir.path()).expect("should create GitRepo"));
    let mut state = StudioState::new(Config::default(), Some(git_repo));
    state.git_status.branch = "feature/neon".to_string();
    state.git_status.commits_ahead = 2;

    assert_eq!(state.suggest_initial_mode(), Mode::PR);
}

#[test]
fn task_context_custom_base_overrides_main_defaults() {
    let review = TaskContext::for_review_with_base(
        None,
        None,
        Some("feature/neon".to_string()),
        false,
        "trunk",
    )
    .expect("review context should succeed");
    assert!(
        matches!(review, TaskContext::Range { from, to } if from == "trunk" && to == "feature/neon")
    );

    let pr = TaskContext::for_pr_with_base(None, None, "trunk");
    assert!(matches!(pr, TaskContext::Range { from, to } if from == "trunk" && to == "HEAD"));
}
