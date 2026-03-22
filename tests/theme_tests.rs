use std::sync::{Mutex, MutexGuard, OnceLock};

use git_iris::theme;
use opaline::{OpalineColor, Theme};

fn theme_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().expect("lock")
}

fn base_theme() -> Theme {
    Theme::builder("Derived Test")
        .token("text.primary", OpalineColor::new(240, 240, 240))
        .token("text.muted", OpalineColor::new(120, 120, 120))
        .token("text.dim", OpalineColor::new(90, 90, 90))
        .token("accent.primary", OpalineColor::new(225, 53, 255))
        .token("accent.secondary", OpalineColor::new(128, 255, 234))
        .token("accent.tertiary", OpalineColor::new(255, 106, 193))
        .token("success", OpalineColor::new(80, 250, 123))
        .token("warning", OpalineColor::new(241, 250, 140))
        .token("error", OpalineColor::new(255, 99, 99))
        .token("info", OpalineColor::new(100, 200, 255))
        .build()
}

#[test]
fn theme_wrapper_derives_iris_specific_tokens_and_styles() {
    let _guard = theme_lock();
    let previous = theme::current();

    theme::set_theme(base_theme());
    let current = theme::current();

    assert_eq!(
        current.color("git.staged"),
        current.color(theme::names::tokens::SUCCESS)
    );
    assert_eq!(
        current.color("code.hash"),
        current.color(theme::names::tokens::ACCENT_TERTIARY)
    );
    assert_eq!(
        current.color("mode.inactive"),
        current.color(theme::names::tokens::TEXT_MUTED)
    );
    assert!(current.has_style("git_staged"));
    assert!(current.has_style("diff_removed"));
    assert!(current.has_style("commit_hash"));
    assert!(current.has_style("file_path"));
    assert!(current.has_style("mode_inactive"));

    theme::set_theme((*previous).clone());
}
