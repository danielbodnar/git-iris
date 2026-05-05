//! Code review types and formatting

use colored::Colorize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::path::PathBuf;

pub const DEFAULT_MIN_FINDING_CONFIDENCE: u8 = 70;

/// Helper to get themed colors for terminal output
mod colors {
    use crate::theme;
    use crate::theme::names::tokens;

    pub fn accent_primary() -> (u8, u8, u8) {
        let c = theme::current().color(tokens::ACCENT_PRIMARY);
        (c.r, c.g, c.b)
    }

    pub fn accent_secondary() -> (u8, u8, u8) {
        let c = theme::current().color(tokens::ACCENT_SECONDARY);
        (c.r, c.g, c.b)
    }

    pub fn accent_tertiary() -> (u8, u8, u8) {
        let c = theme::current().color(tokens::ACCENT_TERTIARY);
        (c.r, c.g, c.b)
    }

    pub fn warning() -> (u8, u8, u8) {
        let c = theme::current().color(tokens::WARNING);
        (c.r, c.g, c.b)
    }

    pub fn error() -> (u8, u8, u8) {
        let c = theme::current().color(tokens::ERROR);
        (c.r, c.g, c.b)
    }

    pub fn text_secondary() -> (u8, u8, u8) {
        let c = theme::current().color(tokens::TEXT_SECONDARY);
        (c.r, c.g, c.b)
    }

    pub fn text_dim() -> (u8, u8, u8) {
        let c = theme::current().color(tokens::TEXT_DIM);
        (c.r, c.g, c.b)
    }
}

/// Structured code review with parseable findings.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Review {
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub findings: Vec<Finding>,
    #[serde(default)]
    pub stats: ReviewStats,
}

impl Review {
    #[must_use]
    pub fn raw_content(&self) -> String {
        self.to_markdown()
    }

    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();
        writeln!(output, "# Code Review").expect("write to string should not fail");

        if !self.summary.trim().is_empty() {
            writeln!(output, "\n## Summary\n\n{}", self.summary.trim())
                .expect("write to string should not fail");
        }

        let visible_findings = self.visible_findings();
        let stats = self.visible_stats();
        writeln!(
            output,
            "\n## Findings\n\nReviewed {} file(s). Found {} issue(s): {} critical, {} high, {} medium, {} low.",
            stats.files_reviewed,
            stats.findings_count,
            stats.critical_count,
            stats.high_count,
            stats.medium_count,
            stats.low_count
        )
        .expect("write to string should not fail");

        if visible_findings.is_empty() {
            output.push_str("\nNo blocking issues found.\n");
            return output;
        }

        for severity in [
            Severity::Critical,
            Severity::High,
            Severity::Medium,
            Severity::Low,
        ] {
            let findings: Vec<&Finding> = visible_findings
                .iter()
                .copied()
                .filter(|finding| finding.severity == severity)
                .collect();

            if findings.is_empty() {
                continue;
            }

            writeln!(output, "\n### {severity}").expect("write to string should not fail");
            for finding in findings {
                writeln!(
                    output,
                    "\n- [{severity}] **{} in `{}`**",
                    finding.title,
                    finding.location()
                )
                .expect("write to string should not fail");
                writeln!(
                    output,
                    "  Category: {}. Confidence: {}%.",
                    finding.category, finding.confidence
                )
                .expect("write to string should not fail");
                writeln!(output, "  {}", finding.body.trim())
                    .expect("write to string should not fail");

                if let Some(fix) = finding
                    .suggested_fix
                    .as_deref()
                    .filter(|fix| !fix.is_empty())
                {
                    writeln!(output, "  **Fix**: {}", fix.trim())
                        .expect("write to string should not fail");
                }

                if !finding.evidence.is_empty() {
                    let evidence = finding
                        .evidence
                        .iter()
                        .map(EvidenceRef::label)
                        .collect::<Vec<_>>()
                        .join(", ");
                    writeln!(output, "  Evidence: {evidence}")
                        .expect("write to string should not fail");
                }
            }
        }

        output
    }

    #[must_use]
    pub fn format(&self) -> String {
        render_markdown_for_terminal(&self.to_markdown())
    }

    #[must_use]
    pub fn effective_stats(&self) -> ReviewStats {
        ReviewStats::from_findings(self.stats.files_reviewed, &self.findings)
    }

    #[must_use]
    pub fn visible_findings(&self) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|finding| finding.confidence >= DEFAULT_MIN_FINDING_CONFIDENCE)
            .collect()
    }

    #[must_use]
    pub fn visible_stats(&self) -> ReviewStats {
        let visible_findings = self.visible_findings();
        let mut stats = ReviewStats {
            files_reviewed: self.effective_stats().files_reviewed,
            findings_count: visible_findings.len(),
            ..ReviewStats::default()
        };

        for finding in visible_findings {
            match finding.severity {
                Severity::Critical => stats.critical_count += 1,
                Severity::High => stats.high_count += 1,
                Severity::Medium => stats.medium_count += 1,
                Severity::Low => stats.low_count += 1,
            }
        }

        stats
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema)]
pub struct Finding {
    pub id: FindingId,
    pub severity: Severity,
    pub confidence: u8,
    pub file: PathBuf,
    pub start_line: u32,
    pub end_line: u32,
    pub category: Category,
    pub title: String,
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_fix: Option<String>,
    #[serde(default)]
    pub evidence: Vec<EvidenceRef>,
}

impl Finding {
    #[must_use]
    pub fn location(&self) -> String {
        let file = self.file.display();
        if self.start_line == self.end_line {
            format!("{file}:{}", self.start_line)
        } else {
            format!("{file}:{}-{}", self.start_line, self.end_line)
        }
    }

    #[must_use]
    pub fn raw_inline_body(&self) -> String {
        let mut body = format!(
            "[{}] **{}**\n\nLocation: `{}`\n\n{}\n\nConfidence: {}%",
            self.severity,
            self.title,
            self.location(),
            self.body.trim(),
            self.confidence
        );

        if let Some(fix) = self.suggested_fix.as_deref().filter(|fix| !fix.is_empty()) {
            write!(body, "\n\n**Fix**: {}", fix.trim()).expect("write to string should not fail");
        }

        body
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema)]
#[serde(transparent)]
pub struct FindingId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema)]
pub struct EvidenceRef {
    pub file: PathBuf,
    pub line: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl EvidenceRef {
    #[must_use]
    pub fn label(&self) -> String {
        let file = self.file.display();
        let line = match self.end_line {
            Some(end_line) if end_line != self.line => format!("{}-{}", self.line, end_line),
            _ => self.line.to_string(),
        };

        match self.note.as_deref().filter(|note| !note.is_empty()) {
            Some(note) => format!("{file}:{line} ({note})"),
            None => format!("{file}:{line}"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Critical => write!(f, "CRITICAL"),
            Self::High => write!(f, "HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::Low => write!(f, "LOW"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Security,
    Performance,
    ErrorHandling,
    Complexity,
    Abstraction,
    Duplication,
    Testing,
    Style,
    ApiContract,
    Concurrency,
    Documentation,
    Other,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Security => write!(f, "security"),
            Self::Performance => write!(f, "performance"),
            Self::ErrorHandling => write!(f, "error handling"),
            Self::Complexity => write!(f, "complexity"),
            Self::Abstraction => write!(f, "abstraction"),
            Self::Duplication => write!(f, "duplication"),
            Self::Testing => write!(f, "testing"),
            Self::Style => write!(f, "style"),
            Self::ApiContract => write!(f, "API contract"),
            Self::Concurrency => write!(f, "concurrency"),
            Self::Documentation => write!(f, "documentation"),
            Self::Other => write!(f, "other"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, JsonSchema)]
pub struct ReviewStats {
    #[serde(default)]
    pub files_reviewed: usize,
    #[serde(default)]
    pub findings_count: usize,
    #[serde(default)]
    pub critical_count: usize,
    #[serde(default)]
    pub high_count: usize,
    #[serde(default)]
    pub medium_count: usize,
    #[serde(default)]
    pub low_count: usize,
}

impl ReviewStats {
    #[must_use]
    pub fn from_findings(files_reviewed: usize, findings: &[Finding]) -> Self {
        let mut stats = Self {
            files_reviewed,
            findings_count: findings.len(),
            ..Self::default()
        };

        for finding in findings {
            match finding.severity {
                Severity::Critical => stats.critical_count += 1,
                Severity::High => stats.high_count += 1,
                Severity::Medium => stats.medium_count += 1,
                Severity::Low => stats.low_count += 1,
            }
        }

        stats
    }
}

/// Render markdown content with `SilkCircuit` terminal styling
///
/// This function parses markdown and applies our color palette for beautiful
/// terminal output. It handles:
/// - Headers (H1, H2, H3) with Electric Purple styling
/// - Bold text with Neon Cyan
/// - Code blocks with dimmed background styling
/// - Bullet lists with Coral bullets
/// - Severity badges [CRITICAL], [HIGH], etc.
#[allow(clippy::too_many_lines)]
#[must_use]
pub fn render_markdown_for_terminal(markdown: &str) -> String {
    let mut output = String::new();
    let mut in_code_block = false;
    let mut code_block_content = String::new();

    for line in markdown.lines() {
        // Handle code blocks
        if line.starts_with("```") {
            if in_code_block {
                // End of code block - output it
                let dim = colors::text_secondary();
                for code_line in code_block_content.lines() {
                    writeln!(output, "  {}", code_line.truecolor(dim.0, dim.1, dim.2))
                        .expect("write to string should not fail");
                }
                code_block_content.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            code_block_content.push_str(line);
            code_block_content.push('\n');
            continue;
        }

        // Handle headers
        if let Some(header) = line.strip_prefix("### ") {
            let cyan = colors::accent_secondary();
            let dim = colors::text_dim();
            writeln!(
                output,
                "\n{} {} {}",
                "─".truecolor(cyan.0, cyan.1, cyan.2),
                style_header_text(header)
                    .truecolor(cyan.0, cyan.1, cyan.2)
                    .bold(),
                "─"
                    .repeat(30usize.saturating_sub(header.len()))
                    .truecolor(dim.0, dim.1, dim.2)
            )
            .expect("write to string should not fail");
        } else if let Some(header) = line.strip_prefix("## ") {
            let purple = colors::accent_primary();
            let dim = colors::text_dim();
            writeln!(
                output,
                "\n{} {} {}",
                "─".truecolor(purple.0, purple.1, purple.2),
                style_header_text(header)
                    .truecolor(purple.0, purple.1, purple.2)
                    .bold(),
                "─"
                    .repeat(32usize.saturating_sub(header.len()))
                    .truecolor(dim.0, dim.1, dim.2)
            )
            .expect("write to string should not fail");
        } else if let Some(header) = line.strip_prefix("# ") {
            // Main title - big and bold
            let purple = colors::accent_primary();
            let cyan = colors::accent_secondary();
            writeln!(
                output,
                "{}  {}  {}",
                "━━━".truecolor(purple.0, purple.1, purple.2),
                style_header_text(header)
                    .truecolor(cyan.0, cyan.1, cyan.2)
                    .bold(),
                "━━━".truecolor(purple.0, purple.1, purple.2)
            )
            .expect("write to string should not fail");
        }
        // Handle bullet points
        else if let Some(content) = line.strip_prefix("- ") {
            let coral = colors::accent_tertiary();
            let styled = style_line_content(content);
            writeln!(
                output,
                "  {} {}",
                "•".truecolor(coral.0, coral.1, coral.2),
                styled
            )
            .expect("write to string should not fail");
        } else if let Some(content) = line.strip_prefix("* ") {
            let coral = colors::accent_tertiary();
            let styled = style_line_content(content);
            writeln!(
                output,
                "  {} {}",
                "•".truecolor(coral.0, coral.1, coral.2),
                styled
            )
            .expect("write to string should not fail");
        }
        // Handle numbered lists
        else if line.chars().next().is_some_and(|c| c.is_ascii_digit()) && line.contains(". ") {
            if let Some((num, rest)) = line.split_once(". ") {
                let coral = colors::accent_tertiary();
                let styled = style_line_content(rest);
                writeln!(
                    output,
                    "  {} {}",
                    format!("{}.", num)
                        .truecolor(coral.0, coral.1, coral.2)
                        .bold(),
                    styled
                )
                .expect("write to string should not fail");
            }
        }
        // Handle empty lines
        else if line.trim().is_empty() {
            output.push('\n');
        }
        // Regular paragraph text
        else {
            let styled = style_line_content(line);
            writeln!(output, "{styled}").expect("write to string should not fail");
        }
    }

    output
}

/// Style header text - uppercase and clean
fn style_header_text(text: &str) -> String {
    text.to_uppercase()
}

/// Style inline content - handles bold, code, severity badges
#[allow(clippy::too_many_lines)]
fn style_line_content(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut current_text = String::new();

    // Get theme colors once for efficiency
    let text_color = colors::text_secondary();
    let error_color = colors::error();
    let warning_color = colors::warning();
    let coral_color = colors::accent_tertiary();
    let cyan_color = colors::accent_secondary();

    while let Some(ch) = chars.next() {
        match ch {
            // Handle severity badges [CRITICAL], [HIGH], [MEDIUM], [LOW]
            '[' => {
                // Flush current text
                if !current_text.is_empty() {
                    result.push_str(
                        &current_text
                            .truecolor(text_color.0, text_color.1, text_color.2)
                            .to_string(),
                    );
                    current_text.clear();
                }

                // Collect badge content
                let mut badge = String::new();
                for c in chars.by_ref() {
                    if c == ']' {
                        break;
                    }
                    badge.push(c);
                }

                // Style based on severity
                let badge_upper = badge.to_uppercase();
                let styled_badge = match badge_upper.as_str() {
                    "CRITICAL" => format!(
                        "[{}]",
                        "CRITICAL"
                            .truecolor(error_color.0, error_color.1, error_color.2)
                            .bold()
                    ),
                    "HIGH" => format!(
                        "[{}]",
                        "HIGH"
                            .truecolor(error_color.0, error_color.1, error_color.2)
                            .bold()
                    ),
                    "MEDIUM" => format!(
                        "[{}]",
                        "MEDIUM"
                            .truecolor(warning_color.0, warning_color.1, warning_color.2)
                            .bold()
                    ),
                    "LOW" => format!(
                        "[{}]",
                        "LOW"
                            .truecolor(coral_color.0, coral_color.1, coral_color.2)
                            .bold()
                    ),
                    _ => format!(
                        "[{}]",
                        badge.truecolor(cyan_color.0, cyan_color.1, cyan_color.2)
                    ),
                };
                result.push_str(&styled_badge);
            }
            // Handle bold text **text**
            '*' if chars.peek() == Some(&'*') => {
                // Flush current text
                if !current_text.is_empty() {
                    result.push_str(
                        &current_text
                            .truecolor(text_color.0, text_color.1, text_color.2)
                            .to_string(),
                    );
                    current_text.clear();
                }

                chars.next(); // consume second *

                // Collect bold content
                let mut bold = String::new();
                while let Some(c) = chars.next() {
                    if c == '*' && chars.peek() == Some(&'*') {
                        chars.next(); // consume closing **
                        break;
                    }
                    bold.push(c);
                }

                result.push_str(
                    &bold
                        .truecolor(cyan_color.0, cyan_color.1, cyan_color.2)
                        .bold()
                        .to_string(),
                );
            }
            // Handle inline code `code`
            '`' => {
                // Flush current text
                if !current_text.is_empty() {
                    result.push_str(
                        &current_text
                            .truecolor(text_color.0, text_color.1, text_color.2)
                            .to_string(),
                    );
                    current_text.clear();
                }

                // Collect code content
                let mut code = String::new();
                for c in chars.by_ref() {
                    if c == '`' {
                        break;
                    }
                    code.push(c);
                }

                result.push_str(
                    &code
                        .truecolor(warning_color.0, warning_color.1, warning_color.2)
                        .to_string(),
                );
            }
            _ => {
                current_text.push(ch);
            }
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        result.push_str(
            &current_text
                .truecolor(text_color.0, text_color.1, text_color.2)
                .to_string(),
        );
    }

    result
}
