use git_iris::{GeneratedMessage, format_commit_message};

fn generated_message(emoji: Option<&str>, title: &str, message: &str) -> GeneratedMessage {
    GeneratedMessage {
        emoji: emoji.map(str::to_string),
        title: title.to_string(),
        message: message.to_string(),
        completion_message: None,
    }
}

#[test]
fn format_commit_message_does_not_duplicate_emoji_from_title() {
    let message = generated_message(
        Some("🧹"),
        "🧹 Tighten clippy restriction lints",
        "Relax assertions in tests.",
    );

    assert_eq!(
        format_commit_message(&message),
        "🧹 Tighten clippy restriction lints\n\nRelax assertions in tests.\n"
    );
}

#[test]
fn format_commit_message_collapses_repeated_title_emoji() {
    let message = generated_message(Some("🧹"), "🧹 🧹 Tighten clippy lints", "Clean it up.");

    assert_eq!(
        format_commit_message(&message),
        "🧹 Tighten clippy lints\n\nClean it up.\n"
    );
}

#[test]
fn format_commit_message_keeps_plain_title_when_emoji_is_absent() {
    let message = generated_message(None, "Tighten clippy lints", "Clean it up.");

    assert_eq!(
        format_commit_message(&message),
        "Tighten clippy lints\n\nClean it up.\n"
    );
}
