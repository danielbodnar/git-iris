use crate::theme;
use crate::theme::names::tokens;
use rand::prelude::*;
use ratatui::style::Color;
use std::sync::LazyLock;

/// A message with a theme-based color token
#[derive(Clone)]
pub struct ColoredMessage {
    pub text: String,
    pub token: &'static str,
}

impl ColoredMessage {
    /// Get the resolved color from the current theme
    pub fn color(&self) -> Color {
        Color::from(theme::current().color(self.token))
    }
}

static WAITING_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        // Cosmic vibes
        ColoredMessage {
            text: "🔮 Consulting the commit oracle...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "✨ Weaving stardust into your message...".to_string(),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: "🌌 Exploring the commit-verse...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🔭 Peering through the code telescope...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "⭐ Aligning the celestial diffs...".to_string(),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: "🌙 Reading your changes by moonlight...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        // Nerdy & clever
        ColoredMessage {
            text: "🎲 Rolling for commit inspiration...".to_string(),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: "🧬 Decoding the DNA of your changes...".to_string(),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: "🔬 Analyzing diff particles...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "📡 Tuning into the commit frequency...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🧪 Distilling the essence of your changes...".to_string(),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: "⚡ Parsing the diff matrix...".to_string(),
            token: tokens::WARNING,
        },
        // Warm & grounded
        ColoredMessage {
            text: "☕ Brewing a fresh commit message...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🎨 Painting your changes in prose...".to_string(),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: "🧩 Piecing together the story...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🎵 Composing a commit symphony...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "💎 Polishing your commit to a shine...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🌿 Growing ideas from your diff...".to_string(),
            token: tokens::SUCCESS,
        },
        // Playful
        ColoredMessage {
            text: "🚀 Launching into commit space...".to_string(),
            token: tokens::ERROR,
        },
        ColoredMessage {
            text: "🗺️ Charting the diff territory...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🌊 Riding the code waves...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🦉 Consulting the git guardians...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🧭 Calibrating the commit compass...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🔐 Unlocking the secrets of your diff...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "🎁 Wrapping up your changes nicely...".to_string(),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: "🏄 Surfing the staged changes...".to_string(),
            token: tokens::SUCCESS,
        },
    ]
});

static REVIEW_WAITING_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        // Cosmic & mystical
        ColoredMessage {
            text: "🔮 Gazing into the code quality crystal...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "✨ Illuminating the hidden corners...".to_string(),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: "🌟 Channeling review wisdom...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🌙 Meditating on your abstractions...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🔭 Scanning the code horizon...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "⭐ Reading the code constellations...".to_string(),
            token: tokens::TEXT_PRIMARY,
        },
        // Nerdy & technical
        ColoredMessage {
            text: "🔬 Analyzing code under the microscope...".to_string(),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: "🧬 Sequencing your code genome...".to_string(),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: "📡 Scanning for code anomalies...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🧪 Running quality experiments...".to_string(),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: "⚡ Tracing the logic pathways...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🎲 Rolling perception checks...".to_string(),
            token: tokens::WARNING,
        },
        // Exploratory
        ColoredMessage {
            text: "🗺️ Mapping your code architecture...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🔍 Hunting for hidden issues...".to_string(),
            token: tokens::ERROR,
        },
        ColoredMessage {
            text: "🧭 Navigating your control flow...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "🏊 Diving into the logic depths...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "⛏️ Mining for code gems...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🌊 Flowing through your functions...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        // Warm & grounded
        ColoredMessage {
            text: "☕ Taking a thoughtful look...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🎨 Appreciating your code craft...".to_string(),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: "🧩 Piecing together the full picture...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "💎 Searching for rough edges to polish...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🦉 Consulting the wise owl...".to_string(),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: "📜 Checking against best practices...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🎵 Listening to your code's rhythm...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "🌿 Tending the code garden...".to_string(),
            token: tokens::SUCCESS,
        },
    ]
});

static USER_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        ColoredMessage {
            text: "🚀 Launching...".to_string(),
            token: tokens::ERROR,
        },
        ColoredMessage {
            text: "✨ Working magic...".to_string(),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: "🔮 Divining...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "⚡ Processing...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🌌 Exploring...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🔬 Analyzing...".to_string(),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: "☕ Brewing...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🎨 Crafting...".to_string(),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: "🧩 Piecing...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "💎 Polishing...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: "🎵 Composing...".to_string(),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: "🌊 Flowing...".to_string(),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: "🔭 Scanning...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🧪 Testing...".to_string(),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: "🌿 Growing...".to_string(),
            token: tokens::SUCCESS,
        },
    ]
});

pub fn get_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "Processing your request...".to_string(),
            token: tokens::WARNING,
        })
}

pub fn get_review_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    REVIEW_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "Analyzing your code quality...".to_string(),
            token: tokens::ACCENT_DEEP,
        })
}

/// Get a waiting message appropriate for the given capability
pub fn get_capability_message(capability: &str) -> ColoredMessage {
    match capability {
        "review" => get_review_waiting_message(),
        "pr" => get_pr_waiting_message(),
        "changelog" => get_changelog_waiting_message(),
        "release_notes" => get_release_notes_waiting_message(),
        // "commit" and any other capability use the default cosmic messages
        _ => get_waiting_message(),
    }
}

static PR_WAITING_MESSAGES: std::sync::LazyLock<Vec<ColoredMessage>> =
    std::sync::LazyLock::new(|| {
        vec![
            ColoredMessage {
                text: "🔮 Crafting your PR narrative...".to_string(),
                token: tokens::ACCENT_DEEP,
            },
            ColoredMessage {
                text: "✨ Weaving your commits into a story...".to_string(),
                token: tokens::TEXT_PRIMARY,
            },
            ColoredMessage {
                text: "📝 Summarizing your brilliant work...".to_string(),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: "🎯 Distilling the essence of your changes...".to_string(),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: "🌟 Highlighting your contributions...".to_string(),
                token: tokens::SUCCESS,
            },
            ColoredMessage {
                text: "📋 Building your PR description...".to_string(),
                token: tokens::WARNING,
            },
            ColoredMessage {
                text: "🎨 Painting the PR picture...".to_string(),
                token: tokens::ACCENT_TERTIARY,
            },
            ColoredMessage {
                text: "🧵 Threading your commits together...".to_string(),
                token: tokens::WARNING,
            },
        ]
    });

static CHANGELOG_WAITING_MESSAGES: std::sync::LazyLock<Vec<ColoredMessage>> =
    std::sync::LazyLock::new(|| {
        vec![
            ColoredMessage {
                text: "📜 Chronicling your changes...".to_string(),
                token: tokens::ACCENT_DEEP,
            },
            ColoredMessage {
                text: "✨ Cataloging your accomplishments...".to_string(),
                token: tokens::TEXT_PRIMARY,
            },
            ColoredMessage {
                text: "📖 Writing the history of your code...".to_string(),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: "🏛️ Archiving your progress...".to_string(),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: "🔖 Tagging your milestones...".to_string(),
                token: tokens::SUCCESS,
            },
            ColoredMessage {
                text: "📝 Documenting the journey...".to_string(),
                token: tokens::WARNING,
            },
            ColoredMessage {
                text: "🗂️ Organizing your achievements...".to_string(),
                token: tokens::ACCENT_TERTIARY,
            },
            ColoredMessage {
                text: "⚡ Capturing the deltas...".to_string(),
                token: tokens::WARNING,
            },
        ]
    });

static RELEASE_NOTES_WAITING_MESSAGES: std::sync::LazyLock<Vec<ColoredMessage>> =
    std::sync::LazyLock::new(|| {
        vec![
            ColoredMessage {
                text: "🚀 Preparing launch notes...".to_string(),
                token: tokens::ERROR,
            },
            ColoredMessage {
                text: "✨ Polishing the release highlights...".to_string(),
                token: tokens::TEXT_PRIMARY,
            },
            ColoredMessage {
                text: "📣 Announcing your achievements...".to_string(),
                token: tokens::ACCENT_DEEP,
            },
            ColoredMessage {
                text: "🎉 Celebrating the release...".to_string(),
                token: tokens::SUCCESS,
            },
            ColoredMessage {
                text: "📦 Packaging the release story...".to_string(),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: "🌟 Showcasing new features...".to_string(),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: "📢 Composing the release fanfare...".to_string(),
                token: tokens::WARNING,
            },
            ColoredMessage {
                text: "🎊 Wrapping up the release...".to_string(),
                token: tokens::ACCENT_TERTIARY,
            },
        ]
    });

fn get_pr_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    PR_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "Building PR description...".to_string(),
            token: tokens::ACCENT_DEEP,
        })
}

fn get_changelog_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    CHANGELOG_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "Generating changelog...".to_string(),
            token: tokens::ACCENT_SECONDARY,
        })
}

fn get_release_notes_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    RELEASE_NOTES_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "Creating release notes...".to_string(),
            token: tokens::SUCCESS,
        })
}

pub fn get_user_message() -> ColoredMessage {
    let mut rng = rand::rng();
    USER_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "What would you like to do?".to_string(),
            token: tokens::ACCENT_SECONDARY,
        })
}
