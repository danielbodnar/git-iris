#![allow(clippy::unwrap_used)]

use git_iris::{
    agents::{TaskContext, setup::IrisAgentService},
    common::CommonParams,
    git::GitRepo,
};
use std::env;
use std::sync::{Mutex, MutexGuard, OnceLock};

#[path = "test_utils.rs"]
mod test_utils;
use test_utils::{GitTestHelper, setup_git_repo};

fn cwd_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn pick_fast_provider() -> Option<(&'static str, &'static str)> {
    if env::var("ANTHROPIC_API_KEY").is_ok() {
        Some(("anthropic", "claude-haiku-4-5-20251001"))
    } else if env::var("OPENAI_API_KEY").is_ok() {
        Some(("openai", "gpt-4o-mini"))
    } else if env::var("GOOGLE_API_KEY").is_ok() {
        Some(("google", "gemini-2.5-flash"))
    } else {
        None
    }
}

fn build_service(
    repo_path: &std::path::Path,
    provider: &str,
    model: &str,
) -> IrisAgentService {
    let common = CommonParams {
        provider: Some(provider.to_string()),
        model: Some(model.to_string()),
        ..Default::default()
    };

    let git_repo = GitRepo::new(repo_path).expect("git repo");
    let mut config = git_iris::config::Config::load().unwrap_or_default();
    common
        .apply_to_config(&mut config)
        .expect("apply params");

    let backend =
        git_iris::agents::core::AgentBackend::from_config(&config).expect("backend");

    let mut service = IrisAgentService::new(
        config,
        backend.provider_name,
        backend.model,
        backend.fast_model,
    );
    service.set_git_repo(git_repo);
    service
}

/// Repo with pure gitmoji history → agent should produce a commit with emoji.
#[tokio::test]
#[ignore] // requires live API key — run with: cargo test -- --ignored
async fn gitmoji_history_produces_emoji_commit() {
    let _guard = cwd_lock();
    let Some((provider, model)) = pick_fast_provider() else {
        return;
    };

    let (temp_dir, _repo) = setup_git_repo();
    let helper = GitTestHelper::new(&temp_dir).unwrap();

    for (i, msg) in [
        "✨ Add user authentication module",
        "🐛 Fix session expiration handling",
        "📝 Update API documentation",
        "♻️ Refactor database connection pool",
        "✅ Add integration tests for auth flow",
        "💄 Improve login page styling",
        "🔧 Update CI configuration",
        "🚀 Deploy v2.1.0 release",
    ]
    .iter()
    .enumerate()
    {
        helper
            .create_and_stage_file(
                &format!("file_{i}.txt"),
                &format!("content {i}"),
            )
            .unwrap();
        helper.commit(msg).unwrap();
    }

    helper
        .create_and_stage_file("new_feature.rs", "fn magic() {}\n")
        .unwrap();

    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(temp_dir.path()).unwrap();

    let service = build_service(temp_dir.path(), provider, model);
    let response = service
        .execute_task("commit", TaskContext::for_gen())
        .await
        .expect("agent should generate commit");

    env::set_current_dir(original_dir).unwrap();

    let msg = match response {
        git_iris::agents::StructuredResponse::CommitMessage(m) => m,
        other => panic!("expected CommitMessage, got {other:?}"),
    };

    assert!(
        msg.emoji.is_some(),
        "gitmoji repo should produce an emoji, got title: {:?}",
        msg.title
    );
}

/// Repo with conventional commits → agent should produce type-prefix format with no emoji.
#[tokio::test]
#[ignore]
async fn conventional_history_produces_no_emoji_commit() {
    let _guard = cwd_lock();
    let Some((provider, model)) = pick_fast_provider() else {
        return;
    };

    let (temp_dir, _repo) = setup_git_repo();
    let helper = GitTestHelper::new(&temp_dir).unwrap();

    for (i, msg) in [
        "feat: add user authentication module",
        "fix: resolve session expiration handling",
        "docs: update API documentation",
        "refactor: simplify database connection pool",
        "test: add integration tests for auth flow",
        "style: improve login page styling",
        "ci: update CI configuration",
        "chore: bump dependencies to latest versions",
    ]
    .iter()
    .enumerate()
    {
        helper
            .create_and_stage_file(
                &format!("file_{i}.txt"),
                &format!("content {i}"),
            )
            .unwrap();
        helper.commit(msg).unwrap();
    }

    helper
        .create_and_stage_file("new_feature.rs", "fn magic() {}\n")
        .unwrap();

    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(temp_dir.path()).unwrap();

    let service = build_service(temp_dir.path(), provider, model);
    let response = service
        .execute_task("commit", TaskContext::for_gen())
        .await
        .expect("agent should generate commit");

    env::set_current_dir(original_dir).unwrap();

    let msg = match response {
        git_iris::agents::StructuredResponse::CommitMessage(m) => m,
        other => panic!("expected CommitMessage, got {other:?}"),
    };

    assert!(
        msg.emoji.is_none(),
        "conventional repo should not produce an emoji, got emoji: {:?}, title: {:?}",
        msg.emoji,
        msg.title
    );

    let has_type_prefix = msg.title.contains(':')
        && msg.title.split(':').next().unwrap().chars().all(|c| c.is_ascii_lowercase() || c == '(' || c == ')' || c == '-');
    assert!(
        has_type_prefix,
        "conventional repo should produce a type-prefix title, got: {:?}",
        msg.title
    );
}

/// Mixed gitmoji + conventional history (like git-iris) → gitmoji presence should win.
#[tokio::test]
#[ignore]
async fn mixed_history_with_gitmoji_produces_emoji_commit() {
    let _guard = cwd_lock();
    let Some((provider, model)) = pick_fast_provider() else {
        return;
    };

    let (temp_dir, _repo) = setup_git_repo();
    let helper = GitTestHelper::new(&temp_dir).unwrap();

    for (i, msg) in [
        "refactor(gitmoji): remove unused functions",
        "✨ feat: add automatic commit style detection",
        "test: stabilize default-branch fixtures in CI",
        "🔨 chore: clear pedantic clippy lint backlog",
        "docs: align architecture guides",
        "fix: add sane OpenAI reasoning defaults",
        "docs: align prompt guidance with compact context",
        "fix(studio): restore repo-aware PR suggestions",
    ]
    .iter()
    .enumerate()
    {
        helper
            .create_and_stage_file(
                &format!("file_{i}.txt"),
                &format!("content {i}"),
            )
            .unwrap();
        helper.commit(msg).unwrap();
    }

    helper
        .create_and_stage_file("new_feature.rs", "fn magic() {}\n")
        .unwrap();

    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(temp_dir.path()).unwrap();

    let service = build_service(temp_dir.path(), provider, model);
    let response = service
        .execute_task("commit", TaskContext::for_gen())
        .await
        .expect("agent should generate commit");

    env::set_current_dir(original_dir).unwrap();

    let msg = match response {
        git_iris::agents::StructuredResponse::CommitMessage(m) => m,
        other => panic!("expected CommitMessage, got {other:?}"),
    };

    assert!(
        msg.emoji.is_some(),
        "mixed repo with gitmoji should produce an emoji, got title: {:?}",
        msg.title
    );
}
