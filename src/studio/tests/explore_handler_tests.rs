use crate::config::Config;
use crate::studio::events::SideEffect;
use crate::studio::handlers::handle_explore_key;
use crate::studio::state::{FileLogEntry, Mode, NotificationLevel, PanelId, StudioState};
use crossterm::event::{KeyCode, KeyEvent};
use std::path::PathBuf;

fn test_state() -> StudioState {
    let mut state = StudioState::new(Config::default(), None);
    state.active_mode = Mode::Explore;
    state.focused_panel = PanelId::Center;
    state
}

#[test]
fn open_in_editor_copies_editor_command_for_selected_file() {
    let mut state = test_state();
    state.modes.explore.current_file = Some(PathBuf::from("src/main.rs"));
    state.modes.explore.current_line = 42;

    let effects = handle_explore_key(&mut state, KeyEvent::from(KeyCode::Char('o')));

    let copied = effects
        .iter()
        .find_map(|effect| match effect {
            SideEffect::CopyToClipboard(command) => Some(command),
            _ => None,
        })
        .expect("editor command should be copied");

    assert!(copied.contains("+42"));
    assert!(copied.contains("src/main.rs"));

    let notification = state
        .notifications
        .back()
        .expect("success notification should be emitted");
    assert_eq!(notification.level, NotificationLevel::Success);
    assert_eq!(notification.message, "Editor command copied");
}

#[test]
fn open_in_editor_without_selection_warns() {
    let mut state = test_state();

    let effects = handle_explore_key(&mut state, KeyEvent::from(KeyCode::Char('o')));

    assert!(effects.is_empty());

    let notification = state
        .notifications
        .back()
        .expect("warning notification should be emitted");
    assert_eq!(notification.level, NotificationLevel::Warning);
    assert_eq!(notification.message, "No file selected");
}

#[test]
fn enter_on_file_history_copies_git_show_command() {
    let mut state = test_state();
    state.focused_panel = PanelId::Right;
    state.modes.explore.current_file = Some(PathBuf::from("src/main.rs"));
    state.modes.explore.file_log = vec![FileLogEntry {
        hash: "abcdef1234567890".to_string(),
        short_hash: "abcdef1".to_string(),
        message: "Add feature".to_string(),
        author: "Bliss".to_string(),
        relative_time: "today".to_string(),
        additions: Some(5),
        deletions: Some(1),
    }];

    let effects = handle_explore_key(&mut state, KeyEvent::from(KeyCode::Enter));

    let copied = effects
        .iter()
        .find_map(|effect| match effect {
            SideEffect::CopyToClipboard(command) => Some(command),
            _ => None,
        })
        .expect("git show command should be copied");

    assert_eq!(copied, "git show abcdef1234567890 -- 'src/main.rs'");

    let notification = state
        .notifications
        .back()
        .expect("success notification should be emitted");
    assert_eq!(notification.level, NotificationLevel::Success);
    assert_eq!(notification.message, "git show command copied");
}

#[test]
fn enter_on_global_history_uses_global_log_entry() {
    let mut state = test_state();
    state.focused_panel = PanelId::Right;
    state.modes.explore.show_global_log = true;
    state.modes.explore.file_log = vec![FileLogEntry {
        hash: "wrong111111111111".to_string(),
        short_hash: "wrong11".to_string(),
        message: "Wrong entry".to_string(),
        author: "Bliss".to_string(),
        relative_time: "today".to_string(),
        additions: None,
        deletions: None,
    }];
    state.modes.explore.global_log = vec![FileLogEntry {
        hash: "fedcba0987654321".to_string(),
        short_hash: "fedcba0".to_string(),
        message: "Global entry".to_string(),
        author: "Bliss".to_string(),
        relative_time: "today".to_string(),
        additions: None,
        deletions: None,
    }];

    let effects = handle_explore_key(&mut state, KeyEvent::from(KeyCode::Enter));

    let copied = effects
        .iter()
        .find_map(|effect| match effect {
            SideEffect::CopyToClipboard(command) => Some(command),
            _ => None,
        })
        .expect("git show command should be copied");

    assert_eq!(copied, "git show fedcba0987654321");
}
