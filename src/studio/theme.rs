//! `SilkCircuit` Neon theme for Iris Studio
//!
//! Electric meets elegant - the visual identity for git-iris TUI.
//!
//! This module wraps the centralized token-based theme system,
//! providing access to colors and styles through the theme API.

use ratatui::style::{Color, Style};

use crate::theme;
use crate::theme::names::{gradients, styles, tokens};

// ═══════════════════════════════════════════════════════════════════════════════
// Semantic Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for commit hashes
pub fn commit_hash() -> Style {
    theme::current().style(styles::COMMIT_HASH).into()
}

/// Style for file paths
pub fn file_path() -> Style {
    theme::current().style(styles::FILE_PATH).into()
}

/// Style for keywords and important markers
pub fn keyword() -> Style {
    theme::current().style(styles::KEYWORD).into()
}

/// Style for line numbers in code views
pub fn line_number() -> Style {
    theme::current().style(styles::LINE_NUMBER).into()
}

/// Style for selected items
pub fn selected() -> Style {
    theme::current().style(styles::SELECTED).into()
}

/// Style for focused panel border
pub fn focused_border() -> Style {
    theme::current().style(styles::FOCUSED_BORDER).into()
}

/// Style for unfocused panel border
pub fn unfocused_border() -> Style {
    theme::current().style(styles::UNFOCUSED_BORDER).into()
}

/// Style for success messages
pub fn success() -> Style {
    theme::current().style(styles::SUCCESS_STYLE).into()
}

/// Style for error messages
pub fn error() -> Style {
    theme::current().style(styles::ERROR_STYLE).into()
}

/// Style for warning messages
pub fn warning() -> Style {
    theme::current().style(styles::WARNING_STYLE).into()
}

/// Style for timestamps
pub fn timestamp() -> Style {
    theme::current().style(styles::TIMESTAMP).into()
}

/// Style for author names
pub fn author() -> Style {
    theme::current().style(styles::AUTHOR).into()
}

/// Style for dimmed/secondary text
pub fn dimmed() -> Style {
    theme::current().style(styles::DIMMED).into()
}

/// Style for inline code in chat/markdown
pub fn inline_code() -> Style {
    theme::current().style(styles::INLINE_CODE).into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Git Status Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for git staged files
pub fn git_staged() -> Style {
    theme::current().style(styles::GIT_STAGED).into()
}

/// Style for git modified files
pub fn git_modified() -> Style {
    theme::current().style(styles::GIT_MODIFIED).into()
}

/// Style for git untracked files
pub fn git_untracked() -> Style {
    theme::current().style(styles::GIT_UNTRACKED).into()
}

/// Style for git deleted files
pub fn git_deleted() -> Style {
    theme::current().style(styles::GIT_DELETED).into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Diff Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for added lines in diff
pub fn diff_added() -> Style {
    theme::current().style(styles::DIFF_ADDED).into()
}

/// Style for removed lines in diff
pub fn diff_removed() -> Style {
    theme::current().style(styles::DIFF_REMOVED).into()
}

/// Style for diff hunk headers
pub fn diff_hunk() -> Style {
    theme::current().style(styles::DIFF_HUNK).into()
}

/// Style for diff context lines
pub fn diff_context() -> Style {
    theme::current().style(styles::DIFF_CONTEXT).into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Color Accessors
// ═══════════════════════════════════════════════════════════════════════════════

/// Get primary accent color (Electric Purple)
pub fn accent_primary() -> Color {
    theme::current().color(tokens::ACCENT_PRIMARY).into()
}

/// Get secondary accent color (Neon Cyan)
pub fn accent_secondary() -> Color {
    theme::current().color(tokens::ACCENT_SECONDARY).into()
}

/// Get tertiary accent color (Coral)
pub fn accent_tertiary() -> Color {
    theme::current().color(tokens::ACCENT_TERTIARY).into()
}

/// Get warning color (Electric Yellow)
pub fn warning_color() -> Color {
    theme::current().color(tokens::WARNING).into()
}

/// Get success color (Success Green)
pub fn success_color() -> Color {
    theme::current().color(tokens::SUCCESS).into()
}

/// Get error color (Error Red)
pub fn error_color() -> Color {
    theme::current().color(tokens::ERROR).into()
}

/// Get primary text color
pub fn text_primary_color() -> Color {
    theme::current().color(tokens::TEXT_PRIMARY).into()
}

/// Get secondary text color
pub fn text_secondary_color() -> Color {
    theme::current().color(tokens::TEXT_SECONDARY).into()
}

/// Get dim text color
pub fn text_dim_color() -> Color {
    theme::current().color(tokens::TEXT_DIM).into()
}

/// Get muted text color
pub fn text_muted_color() -> Color {
    theme::current().color(tokens::TEXT_MUTED).into()
}

/// Get highlight background color
pub fn bg_highlight_color() -> Color {
    theme::current().color(tokens::BG_HIGHLIGHT).into()
}

/// Get selection background color
pub fn bg_selection_color() -> Color {
    theme::current().color(tokens::BG_SELECTION).into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Mode Tab Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for inactive mode tab
pub fn mode_inactive() -> Style {
    theme::current().style(styles::MODE_INACTIVE).into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Gradients
// ═══════════════════════════════════════════════════════════════════════════════

/// Get a color for a gradient position (0.0 = start, 1.0 = end)
/// Gradient goes from Electric Purple → Neon Cyan
pub fn gradient_purple_cyan(position: f32) -> Color {
    theme::current().gradient(gradients::PRIMARY, position).into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Animation
// ═══════════════════════════════════════════════════════════════════════════════

/// Braille spinner frames for loading indicators
pub const SPINNER_BRAILLE: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
