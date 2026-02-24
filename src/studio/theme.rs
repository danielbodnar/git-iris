//! `SilkCircuit` Neon theme for Iris Studio
//!
//! Electric meets elegant - the visual identity for git-iris TUI.
//!
//! This module wraps the centralized token-based theme system,
//! providing access to colors and styles through the theme API.

use ratatui::style::{Color, Style};

use crate::theme;

// ═══════════════════════════════════════════════════════════════════════════════
// Semantic Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for commit hashes
pub fn commit_hash() -> Style {
    theme::current().style("commit_hash").into()
}

/// Style for file paths
pub fn file_path() -> Style {
    theme::current().style("file_path").into()
}

/// Style for keywords and important markers
pub fn keyword() -> Style {
    theme::current().style("keyword").into()
}

/// Style for line numbers in code views
pub fn line_number() -> Style {
    theme::current().style("line_number").into()
}

/// Style for selected items
pub fn selected() -> Style {
    theme::current().style("selected").into()
}

/// Style for focused panel border
pub fn focused_border() -> Style {
    theme::current().style("focused_border").into()
}

/// Style for unfocused panel border
pub fn unfocused_border() -> Style {
    theme::current().style("unfocused_border").into()
}

/// Style for success messages
pub fn success() -> Style {
    theme::current().style("success_style").into()
}

/// Style for error messages
pub fn error() -> Style {
    theme::current().style("error_style").into()
}

/// Style for warning messages
pub fn warning() -> Style {
    theme::current().style("warning_style").into()
}

/// Style for timestamps
pub fn timestamp() -> Style {
    theme::current().style("timestamp").into()
}

/// Style for author names
pub fn author() -> Style {
    theme::current().style("author").into()
}

/// Style for dimmed/secondary text
pub fn dimmed() -> Style {
    theme::current().style("dimmed").into()
}

/// Style for inline code in chat/markdown
pub fn inline_code() -> Style {
    theme::current().style("inline_code").into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Git Status Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for git staged files
pub fn git_staged() -> Style {
    theme::current().style("git_staged").into()
}

/// Style for git modified files
pub fn git_modified() -> Style {
    theme::current().style("git_modified").into()
}

/// Style for git untracked files
pub fn git_untracked() -> Style {
    theme::current().style("git_untracked").into()
}

/// Style for git deleted files
pub fn git_deleted() -> Style {
    theme::current().style("git_deleted").into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Diff Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for added lines in diff
pub fn diff_added() -> Style {
    theme::current().style("diff_added").into()
}

/// Style for removed lines in diff
pub fn diff_removed() -> Style {
    theme::current().style("diff_removed").into()
}

/// Style for diff hunk headers
pub fn diff_hunk() -> Style {
    theme::current().style("diff_hunk").into()
}

/// Style for diff context lines
pub fn diff_context() -> Style {
    theme::current().style("diff_context").into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Color Accessors
// ═══════════════════════════════════════════════════════════════════════════════

/// Get primary accent color (Electric Purple)
pub fn accent_primary() -> Color {
    theme::current().color("accent.primary").into()
}

/// Get secondary accent color (Neon Cyan)
pub fn accent_secondary() -> Color {
    theme::current().color("accent.secondary").into()
}

/// Get tertiary accent color (Coral)
pub fn accent_tertiary() -> Color {
    theme::current().color("accent.tertiary").into()
}

/// Get warning color (Electric Yellow)
pub fn warning_color() -> Color {
    theme::current().color("warning").into()
}

/// Get success color (Success Green)
pub fn success_color() -> Color {
    theme::current().color("success").into()
}

/// Get error color (Error Red)
pub fn error_color() -> Color {
    theme::current().color("error").into()
}

/// Get primary text color
pub fn text_primary_color() -> Color {
    theme::current().color("text.primary").into()
}

/// Get secondary text color
pub fn text_secondary_color() -> Color {
    theme::current().color("text.secondary").into()
}

/// Get dim text color
pub fn text_dim_color() -> Color {
    theme::current().color("text.dim").into()
}

/// Get muted text color
pub fn text_muted_color() -> Color {
    theme::current().color("text.muted").into()
}

/// Get highlight background color
pub fn bg_highlight_color() -> Color {
    theme::current().color("bg.highlight").into()
}

/// Get selection background color
pub fn bg_selection_color() -> Color {
    theme::current().color("bg.selection").into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Mode Tab Styles
// ═══════════════════════════════════════════════════════════════════════════════

/// Style for inactive mode tab
pub fn mode_inactive() -> Style {
    theme::current().style("mode_inactive").into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Gradients
// ═══════════════════════════════════════════════════════════════════════════════

/// Get a color for a gradient position (0.0 = start, 1.0 = end)
/// Gradient goes from Electric Purple → Neon Cyan
pub fn gradient_purple_cyan(position: f32) -> Color {
    theme::current().gradient("primary", position).into()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Animation
// ═══════════════════════════════════════════════════════════════════════════════

/// Braille spinner frames for loading indicators
pub const SPINNER_BRAILLE: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
