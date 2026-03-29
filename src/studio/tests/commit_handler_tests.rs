use crate::config::Config;
use crate::studio::components::{DiffViewState, FileDiff, FileTreeState};
use crate::studio::events::SideEffect;
use crate::studio::handlers::handle_commit_key;
use crate::studio::state::{Mode, PanelId, StudioState};
use crossterm::event::{KeyCode, KeyEvent};
use std::path::PathBuf;

fn test_state() -> StudioState {
    let mut state = StudioState::new(Config::default(), None);
    state.active_mode = Mode::Commit;
    state
}

fn set_commit_diffs(state: &mut StudioState, paths: &[&str]) {
    let diffs = paths.iter().map(FileDiff::new).collect();
    state.modes.commit.diff_view = DiffViewState::new();
    state.modes.commit.diff_view.set_diffs(diffs);
    let tree_paths = paths.iter().map(PathBuf::from).collect::<Vec<_>>();
    state.modes.commit.file_tree = FileTreeState::from_paths(&tree_paths, &[]);
    state.modes.commit.file_tree.expand_all();
}

#[test]
fn diff_panel_stage_shortcut_uses_current_diff_path() {
    let mut state = test_state();
    state.focused_panel = PanelId::Right;
    set_commit_diffs(&mut state, &["src/lib.rs"]);

    let effects = handle_commit_key(&mut state, KeyEvent::from(KeyCode::Char('s')));

    assert!(effects.iter().any(|effect| matches!(
        effect,
        SideEffect::GitStage(path) if path == &PathBuf::from("src/lib.rs")
    )));
}

#[test]
fn diff_panel_unstage_shortcut_uses_current_diff_path() {
    let mut state = test_state();
    state.focused_panel = PanelId::Right;
    set_commit_diffs(&mut state, &["src/main.rs"]);

    let effects = handle_commit_key(&mut state, KeyEvent::from(KeyCode::Char('u')));

    assert!(effects.iter().any(|effect| matches!(
        effect,
        SideEffect::GitUnstage(path) if path == &PathBuf::from("src/main.rs")
    )));
}

#[test]
fn diff_panel_file_navigation_syncs_file_tree_selection() {
    let mut state = test_state();
    state.focused_panel = PanelId::Right;
    set_commit_diffs(&mut state, &["src/lib.rs", "src/main.rs"]);

    let effects = handle_commit_key(&mut state, KeyEvent::from(KeyCode::Char('n')));

    assert!(effects.is_empty());
    assert_eq!(
        state
            .modes
            .commit
            .diff_view
            .current_diff()
            .map(|diff| diff.path.clone()),
        Some(PathBuf::from("src/main.rs"))
    );
    assert_eq!(
        state.modes.commit.file_tree.selected_path(),
        Some(PathBuf::from("src/main.rs"))
    );
}
