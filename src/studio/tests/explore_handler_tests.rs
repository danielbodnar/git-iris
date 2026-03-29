use crate::config::Config;
use crate::studio::events::SideEffect;
use crate::studio::handlers::handle_explore_key;
use crate::studio::state::{Mode, NotificationLevel, PanelId, StudioState};
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
