//! CLI output utilities with `SilkCircuit` Neon theming.
//!
//! This module provides themed CLI output using the centralized theme system.
//! All colors are resolved at runtime from the active theme.

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use parking_lot::Mutex;
use std::fmt::Write;
use std::time::Duration;

use crate::theme;
use crate::theme::gradient_string;
use crate::theme::names::{gradients as gradient_names, tokens};

// ═══════════════════════════════════════════════════════════════════════════════
// Theme-Based RGB Accessors for CLI Output
// ═══════════════════════════════════════════════════════════════════════════════

/// RGB tuple accessors for use with the `colored` crate's `.truecolor()` method.
/// All colors resolve from the current theme at runtime.
pub mod rgb {
    use crate::theme;
    use crate::theme::names::tokens;

    /// Get primary accent color (Electric Purple) RGB from theme
    pub fn accent_primary() -> (u8, u8, u8) {
        theme::current().color(tokens::ACCENT_PRIMARY).to_rgb_tuple()
    }

    /// Get secondary accent color (Neon Cyan) RGB from theme
    pub fn accent_secondary() -> (u8, u8, u8) {
        theme::current().color(tokens::ACCENT_SECONDARY).to_rgb_tuple()
    }

    /// Get tertiary accent color (Coral) RGB from theme
    pub fn accent_tertiary() -> (u8, u8, u8) {
        theme::current().color(tokens::ACCENT_TERTIARY).to_rgb_tuple()
    }

    /// Get warning color (Electric Yellow) RGB from theme
    pub fn warning() -> (u8, u8, u8) {
        theme::current().color(tokens::WARNING).to_rgb_tuple()
    }

    /// Get success color (Success Green) RGB from theme
    pub fn success() -> (u8, u8, u8) {
        theme::current().color(tokens::SUCCESS).to_rgb_tuple()
    }

    /// Get error color (Error Red) RGB from theme
    pub fn error() -> (u8, u8, u8) {
        theme::current().color(tokens::ERROR).to_rgb_tuple()
    }

    /// Get primary text color RGB from theme
    pub fn text_primary() -> (u8, u8, u8) {
        theme::current().color(tokens::TEXT_PRIMARY).to_rgb_tuple()
    }

    /// Get secondary text color RGB from theme
    pub fn text_secondary() -> (u8, u8, u8) {
        theme::current().color(tokens::TEXT_SECONDARY).to_rgb_tuple()
    }

    /// Get muted text color RGB from theme
    pub fn text_muted() -> (u8, u8, u8) {
        theme::current().color(tokens::TEXT_MUTED).to_rgb_tuple()
    }

    /// Get dim text color RGB from theme
    pub fn text_dim() -> (u8, u8, u8) {
        theme::current().color(tokens::TEXT_DIM).to_rgb_tuple()
    }
}

/// Track quiet mode state
static QUIET_MODE: std::sync::LazyLock<Mutex<bool>> =
    std::sync::LazyLock::new(|| Mutex::new(false));

/// Enable or disable quiet mode
pub fn set_quiet_mode(enabled: bool) {
    let mut quiet_mode = QUIET_MODE.lock();
    *quiet_mode = enabled;
}

/// Check if quiet mode is enabled
pub fn is_quiet_mode() -> bool {
    *QUIET_MODE.lock()
}

pub fn create_spinner(message: &str) -> ProgressBar {
    // Don't create a spinner in quiet mode
    if is_quiet_mode() {
        return ProgressBar::hidden();
    }

    let pb = ProgressBar::new_spinner();

    // Use agent-aware spinner if agent mode is enabled
    if crate::agents::status::is_agent_mode_enabled() {
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.bright_cyan.bold} {msg}")
                .expect("Could not set spinner style"),
        );

        // Start with Iris initialization message
        pb.set_message("◎ Iris initializing...");

        // Set up a custom callback to update the message from Iris status
        let pb_clone = pb.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(200));
            loop {
                interval.tick().await;
                let status_message = crate::agents::status::IRIS_STATUS.get_for_spinner();
                pb_clone.set_message(status_message.text);
            }
        });

        pb.enable_steady_tick(Duration::from_millis(100));
    } else {
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("✦✧✶✷✸✹✺✻✼✽")
                .template("{spinner} {msg}")
                .expect("Could not set spinner style"),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
    }

    pb
}

/// Print info message using theme colors
pub fn print_info(message: &str) {
    if !is_quiet_mode() {
        let color = theme::current().color(tokens::INFO);
        println!("{}", message.truecolor(color.r, color.g, color.b).bold());
    }
}

/// Print warning message using theme colors
pub fn print_warning(message: &str) {
    if !is_quiet_mode() {
        let color = theme::current().color(tokens::WARNING);
        println!("{}", message.truecolor(color.r, color.g, color.b).bold());
    }
}

/// Print error message using theme colors
pub fn print_error(message: &str) {
    // Always print errors, even in quiet mode
    let color = theme::current().color(tokens::ERROR);
    eprintln!("{}", message.truecolor(color.r, color.g, color.b).bold());
}

/// Print success message using theme colors
pub fn print_success(message: &str) {
    if !is_quiet_mode() {
        let color = theme::current().color(tokens::SUCCESS);
        println!("{}", message.truecolor(color.r, color.g, color.b).bold());
    }
}

pub fn print_version(version: &str) {
    if !is_quiet_mode() {
        let t = theme::current();
        let purple = t.color(tokens::ACCENT_PRIMARY);
        let cyan = t.color(tokens::ACCENT_SECONDARY);
        let green = t.color(tokens::SUCCESS);

        println!(
            "{} {} {}",
            "🔮 Git-Iris".truecolor(purple.r, purple.g, purple.b).bold(),
            "version".truecolor(cyan.r, cyan.g, cyan.b),
            version.truecolor(green.r, green.g, green.b)
        );
    }
}

/// Print content with decorative borders
pub fn print_bordered_content(content: &str) {
    if !is_quiet_mode() {
        let color = theme::current().color(tokens::ACCENT_PRIMARY);
        println!("{}", "━".repeat(50).truecolor(color.r, color.g, color.b));
        println!("{content}");
        println!("{}", "━".repeat(50).truecolor(color.r, color.g, color.b));
    }
}

/// Print a simple message (respects quiet mode)
pub fn print_message(message: &str) {
    if !is_quiet_mode() {
        println!("{message}");
    }
}

/// Print an empty line (respects quiet mode)
pub fn print_newline() {
    if !is_quiet_mode() {
        println!();
    }
}

/// Create gradient text with `SilkCircuit` Electric Purple -> Neon Cyan
pub fn create_gradient_text(text: &str) -> String {
    if let Some(gradient) = theme::current().get_gradient(gradient_names::PRIMARY) {
        gradient_string(text, gradient)
    } else {
        // Fallback to legacy gradient
        let gradient = vec![
            (225, 53, 255),  // Electric Purple
            (200, 100, 255), // Mid purple
            (180, 150, 250), // Light purple
            (150, 200, 245), // Purple-cyan
            (128, 255, 234), // Neon Cyan
        ];
        apply_gradient(text, &gradient)
    }
}

/// Create secondary gradient with `SilkCircuit` Coral -> Electric Yellow
pub fn create_secondary_gradient_text(text: &str) -> String {
    if let Some(gradient) = theme::current().get_gradient(gradient_names::WARM) {
        gradient_string(text, gradient)
    } else {
        // Fallback to legacy gradient
        let gradient = vec![
            (255, 106, 193), // Coral
            (255, 150, 180), // Light coral
            (255, 200, 160), // Coral-yellow
            (248, 230, 140), // Light yellow
            (241, 250, 140), // Electric Yellow
        ];
        apply_gradient(text, &gradient)
    }
}

fn apply_gradient(text: &str, gradient: &[(u8, u8, u8)]) -> String {
    let chars: Vec<char> = text.chars().collect();
    let chars_len = chars.len();
    let gradient_len = gradient.len();

    let mut result = String::new();

    if chars_len == 0 || gradient_len == 0 {
        return result;
    }

    chars.iter().enumerate().fold(&mut result, |acc, (i, &c)| {
        let index = if chars_len == 1 {
            0
        } else {
            i * (gradient_len - 1) / (chars_len - 1)
        };
        let (r, g, b) = gradient[index];
        write!(acc, "{}", c.to_string().truecolor(r, g, b)).expect("writing to string cannot fail");
        acc
    });

    result
}
