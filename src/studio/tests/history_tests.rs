//! Tests for the History system

use crate::studio::events::{ContentType, EventSource};
use crate::studio::history::{ChatRole, ContentData, History};
use crate::studio::state::Mode;
use crate::types::GeneratedMessage;

#[test]
fn test_new_history() {
    let history = History::new();
    assert_eq!(history.event_count(), 0);
    assert_eq!(history.chat_messages().len(), 0);
}

#[test]
fn test_add_chat_message() {
    let mut history = History::new();

    history.add_chat_message(ChatRole::User, "Hello, Iris!");
    history.add_chat_message(ChatRole::Iris, "Hello! How can I help?");

    assert_eq!(history.chat_messages().len(), 2);
    assert_eq!(history.chat_messages()[0].role, ChatRole::User);
    assert_eq!(history.chat_messages()[1].role, ChatRole::Iris);
}

#[test]
fn test_record_content() {
    let mut history = History::new();

    let msg = GeneratedMessage {
        emoji: Some("✨".to_string()),
        title: "Add new feature".to_string(),
        message: "Implement the thing".to_string(),
        completion_message: None,
    };

    history.record_content(
        Mode::Commit,
        ContentType::CommitMessage,
        &ContentData::Commit(msg),
        EventSource::Agent,
        "initial_generation",
    );

    assert_eq!(
        history.content_version_count(Mode::Commit, ContentType::CommitMessage),
        1
    );
    assert!(
        history
            .latest_content(Mode::Commit, ContentType::CommitMessage)
            .is_some()
    );
}

#[test]
fn test_content_preview() {
    let msg = GeneratedMessage {
        emoji: Some("🔧".to_string()),
        title: "Fix the bug".to_string(),
        message: "Details here".to_string(),
        completion_message: None,
    };

    let data = ContentData::Commit(msg);
    assert!(data.preview(50).starts_with("🔧 Fix"));
}

#[test]
fn test_history_trimming() {
    let mut history = History::new();

    for i in 0..1_500 {
        let message = format!("Message {}", i);
        history.add_chat_message(ChatRole::User, &message);
    }

    assert!(history.event_count() <= 1_000);
    assert!(history.chat_messages().len() <= 500);
}
