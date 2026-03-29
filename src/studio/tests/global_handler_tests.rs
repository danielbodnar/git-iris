use crate::config::Config;
use crate::studio::events::{DataType, SideEffect};
use crate::studio::handlers::handle_key_event;
use crate::studio::state::{Modal, Mode, StudioState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn test_state() -> StudioState {
    StudioState::new(Config::default(), None)
}

#[test]
fn shifted_lowercase_settings_shortcut_opens_modal() {
    let mut state = test_state();

    let effects = handle_key_event(
        &mut state,
        KeyEvent::new(KeyCode::Char('s'), KeyModifiers::SHIFT),
    );

    assert!(effects.is_empty());
    assert!(matches!(state.modal, Some(Modal::Settings(_))));
}

#[test]
fn uppercase_settings_shortcut_without_shift_still_opens_modal() {
    let mut state = test_state();

    let effects = handle_key_event(
        &mut state,
        KeyEvent::new(KeyCode::Char('S'), KeyModifiers::NONE),
    );

    assert!(effects.is_empty());
    assert!(matches!(state.modal, Some(Modal::Settings(_))));
}

#[test]
fn shifted_lowercase_review_shortcut_switches_modes() {
    let mut state = test_state();

    let effects = handle_key_event(
        &mut state,
        KeyEvent::new(KeyCode::Char('r'), KeyModifiers::SHIFT),
    );

    assert_eq!(state.active_mode, Mode::Review);
    assert!(effects.iter().any(|effect| matches!(
        effect,
        SideEffect::LoadData {
            data_type: DataType::ReviewDiff,
            ..
        }
    )));
}
