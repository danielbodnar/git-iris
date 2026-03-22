use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use opaline::{self as core, OpalineError, OpalineStyle, Theme};

pub use core::{ThemeInfo, ThemeVariant, gradient_string, names};

const APP_NAME: &str = "git-iris";

const TOKEN_GIT_STAGED: &str = "git.staged";
const TOKEN_GIT_MODIFIED: &str = "git.modified";
const TOKEN_GIT_UNTRACKED: &str = "git.untracked";
const TOKEN_GIT_DELETED: &str = "git.deleted";
const TOKEN_DIFF_ADDED: &str = "diff.added";
const TOKEN_DIFF_REMOVED: &str = "diff.removed";
const TOKEN_DIFF_HUNK: &str = "diff.hunk";
const TOKEN_DIFF_CONTEXT: &str = "diff.context";
const TOKEN_MODE_ACTIVE: &str = "mode.active";
const TOKEN_MODE_INACTIVE: &str = "mode.inactive";
const TOKEN_MODE_HOVER: &str = "mode.hover";
const TOKEN_CODE_HASH: &str = "code.hash";
const TOKEN_CODE_PATH: &str = "code.path";

pub(crate) const STYLE_COMMIT_HASH: &str = "commit_hash";
pub(crate) const STYLE_FILE_PATH: &str = "file_path";
pub(crate) const STYLE_TIMESTAMP: &str = "timestamp";
pub(crate) const STYLE_AUTHOR: &str = "author";
pub(crate) const STYLE_GIT_STAGED: &str = "git_staged";
pub(crate) const STYLE_GIT_MODIFIED: &str = "git_modified";
pub(crate) const STYLE_GIT_UNTRACKED: &str = "git_untracked";
pub(crate) const STYLE_GIT_DELETED: &str = "git_deleted";
pub(crate) const STYLE_DIFF_ADDED: &str = "diff_added";
pub(crate) const STYLE_DIFF_REMOVED: &str = "diff_removed";
pub(crate) const STYLE_DIFF_HUNK: &str = "diff_hunk";
pub(crate) const STYLE_DIFF_CONTEXT: &str = "diff_context";
pub(crate) const STYLE_MODE_INACTIVE: &str = "mode_inactive";

fn derive_iris_theme(theme: &mut Theme) {
    use core::names::tokens;

    theme.register_default_token(TOKEN_GIT_STAGED, theme.color(tokens::SUCCESS));
    theme.register_default_token(TOKEN_GIT_MODIFIED, theme.color(tokens::WARNING));
    theme.register_default_token(TOKEN_GIT_UNTRACKED, theme.color(tokens::TEXT_MUTED));
    theme.register_default_token(TOKEN_GIT_DELETED, theme.color(tokens::ERROR));
    theme.register_default_token(TOKEN_DIFF_ADDED, theme.color(tokens::SUCCESS));
    theme.register_default_token(TOKEN_DIFF_REMOVED, theme.color(tokens::ERROR));
    theme.register_default_token(TOKEN_DIFF_HUNK, theme.color(tokens::INFO));
    theme.register_default_token(TOKEN_DIFF_CONTEXT, theme.color(tokens::TEXT_DIM));
    theme.register_default_token(TOKEN_MODE_ACTIVE, theme.color(tokens::ACCENT_PRIMARY));
    theme.register_default_token(TOKEN_MODE_INACTIVE, theme.color(tokens::TEXT_MUTED));
    theme.register_default_token(TOKEN_MODE_HOVER, theme.color(tokens::ACCENT_SECONDARY));
    theme.register_default_token(TOKEN_CODE_HASH, theme.color(tokens::ACCENT_TERTIARY));
    theme.register_default_token(TOKEN_CODE_PATH, theme.color(tokens::ACCENT_SECONDARY));

    theme.register_default_style(
        STYLE_COMMIT_HASH,
        OpalineStyle::fg(theme.color(TOKEN_CODE_HASH)),
    );
    theme.register_default_style(
        STYLE_FILE_PATH,
        OpalineStyle::fg(theme.color(TOKEN_CODE_PATH)),
    );
    theme.register_default_style(
        STYLE_TIMESTAMP,
        OpalineStyle::fg(theme.color(tokens::WARNING)),
    );
    theme.register_default_style(
        STYLE_AUTHOR,
        OpalineStyle::fg(theme.color(tokens::TEXT_PRIMARY)),
    );
    theme.register_default_style(
        STYLE_GIT_STAGED,
        OpalineStyle::fg(theme.color(TOKEN_GIT_STAGED)),
    );
    theme.register_default_style(
        STYLE_GIT_MODIFIED,
        OpalineStyle::fg(theme.color(TOKEN_GIT_MODIFIED)),
    );
    theme.register_default_style(
        STYLE_GIT_UNTRACKED,
        OpalineStyle::fg(theme.color(TOKEN_GIT_UNTRACKED)),
    );
    theme.register_default_style(
        STYLE_GIT_DELETED,
        OpalineStyle::fg(theme.color(TOKEN_GIT_DELETED)),
    );
    theme.register_default_style(
        STYLE_DIFF_ADDED,
        OpalineStyle::fg(theme.color(TOKEN_DIFF_ADDED)),
    );
    theme.register_default_style(
        STYLE_DIFF_REMOVED,
        OpalineStyle::fg(theme.color(TOKEN_DIFF_REMOVED)),
    );
    theme.register_default_style(
        STYLE_DIFF_HUNK,
        OpalineStyle::fg(theme.color(TOKEN_DIFF_HUNK)),
    );
    theme.register_default_style(
        STYLE_DIFF_CONTEXT,
        OpalineStyle::fg(theme.color(TOKEN_DIFF_CONTEXT)),
    );
    theme.register_default_style(
        STYLE_MODE_INACTIVE,
        OpalineStyle::fg(theme.color(TOKEN_MODE_INACTIVE)),
    );
}

fn has_iris_derivations(theme: &Theme) -> bool {
    theme.has_style(STYLE_GIT_STAGED)
        && theme.has_style(STYLE_GIT_MODIFIED)
        && theme.has_style(STYLE_GIT_UNTRACKED)
        && theme.has_style(STYLE_GIT_DELETED)
        && theme.has_style(STYLE_DIFF_ADDED)
        && theme.has_style(STYLE_DIFF_REMOVED)
        && theme.has_style(STYLE_DIFF_HUNK)
        && theme.has_style(STYLE_DIFF_CONTEXT)
        && theme.has_style(STYLE_COMMIT_HASH)
        && theme.has_style(STYLE_FILE_PATH)
        && theme.has_style(STYLE_MODE_INACTIVE)
}

pub fn current() -> Arc<Theme> {
    let current = core::current();
    if has_iris_derivations(&current) {
        return current;
    }

    let mut derived = (*current).clone();
    derive_iris_theme(&mut derived);
    core::set_theme(derived);
    core::current()
}

pub fn set_theme(mut theme: Theme) {
    derive_iris_theme(&mut theme);
    core::set_theme(theme);
}

pub fn load_theme(path: &Path) -> Result<(), OpalineError> {
    let mut theme = core::load_from_file(path)?;
    derive_iris_theme(&mut theme);
    core::set_theme(theme);
    Ok(())
}

pub fn load_theme_by_name(name: &str) -> Result<(), OpalineError> {
    if let Some(mut theme) = load_from_theme_dirs(name, core::app_theme_dirs(APP_NAME))? {
        derive_iris_theme(&mut theme);
        core::set_theme(theme);
        return Ok(());
    }

    if let Some(mut theme) = core::load_by_name(name) {
        derive_iris_theme(&mut theme);
        core::set_theme(theme);
        return Ok(());
    }

    Err(OpalineError::ThemeNotFound {
        name: name.to_string(),
    })
}

pub fn list_available_themes() -> Vec<ThemeInfo> {
    let mut themes = Vec::new();

    for theme in core::list_available_themes() {
        push_or_replace_theme(&mut themes, theme);
    }

    for dir in core::app_theme_dirs(APP_NAME) {
        scan_theme_dir(&mut themes, dir);
    }

    themes
}

fn push_or_replace_theme(themes: &mut Vec<ThemeInfo>, info: ThemeInfo) {
    if let Some(existing) = themes.iter_mut().find(|theme| theme.name == info.name) {
        *existing = info;
    } else {
        themes.push(info);
    }
}

fn scan_theme_dir(themes: &mut Vec<ThemeInfo>, dir: PathBuf) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml")
            && let Some(info) = theme_info_from_path(&path)
        {
            push_or_replace_theme(themes, info);
        }
    }
}

fn theme_info_from_path(path: &Path) -> Option<ThemeInfo> {
    let theme = core::load_from_file(path).ok()?;
    let name = path.file_stem()?.to_string_lossy().into_owned();

    Some(ThemeInfo {
        name,
        display_name: theme.meta.name.clone(),
        variant: theme.meta.variant,
        author: theme.meta.author.clone().unwrap_or_default(),
        description: theme.meta.description.clone().unwrap_or_default(),
        builtin: false,
        path: Some(path.to_path_buf()),
    })
}

fn load_from_theme_dirs<I, P>(name: &str, dirs: I) -> Result<Option<Theme>, OpalineError>
where
    I: IntoIterator<Item = P>,
    P: Into<PathBuf>,
{
    let mut matched_path = None;

    for dir in dirs.into_iter().map(Into::into) {
        let path = dir.join(format!("{name}.toml"));
        if path.exists() {
            matched_path = Some(path);
        }
    }

    matched_path.map_or(Ok(None), |path| core::load_from_file(&path).map(Some))
}
