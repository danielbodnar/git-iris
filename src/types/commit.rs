//! Commit message types and formatting

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use textwrap;

/// Model for commit message generation results
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct GeneratedMessage {
    /// Optional emoji for the commit message
    pub emoji: Option<String>,
    /// Commit message title/subject line
    pub title: String,
    /// Detailed commit message body
    pub message: String,
    /// Brief completion status message for the UI (e.g., "Auth refactor ready.")
    #[serde(default)]
    pub completion_message: Option<String>,
}

impl GeneratedMessage {
    pub(crate) fn subject(&self) -> String {
        let title = self.title_without_repeated_emoji();

        match self
            .emoji
            .as_deref()
            .map(str::trim)
            .filter(|emoji| !emoji.is_empty())
        {
            Some(emoji) => format!("{emoji} {title}"),
            None => title.to_string(),
        }
    }

    pub(crate) fn title_without_repeated_emoji(&self) -> &str {
        let mut title = self.title.trim_start();

        let Some(emoji) = self
            .emoji
            .as_deref()
            .map(str::trim)
            .filter(|emoji| !emoji.is_empty())
        else {
            return title;
        };

        while let Some(rest) = title.strip_prefix(emoji) {
            title = rest.trim_start();
        }

        title
    }
}

/// Formats a commit message from a `GeneratedMessage`
#[must_use]
pub fn format_commit_message(response: &GeneratedMessage) -> String {
    let mut message = String::new();

    write!(&mut message, "{}", response.subject()).expect("write to string should not fail");
    message.push_str("\n\n");

    let wrapped_message = textwrap::wrap(&response.message, 78);
    for line in wrapped_message {
        message.push_str(&line);
        message.push('\n');
    }

    message
}
