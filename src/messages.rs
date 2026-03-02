use crate::theme;
use crate::theme::names::tokens;
use rand::prelude::*;
use ratatui::style::Color;
use std::borrow::Cow;
use std::sync::LazyLock;

/// A message with a theme-based color token
#[derive(Clone)]
pub struct ColoredMessage {
    pub text: Cow<'static, str>,
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
            text: Cow::Borrowed("🔮 Consulting the commit oracle..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("✨ Weaving stardust into your message..."),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌌 Exploring the commit-verse..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔭 Peering through the code telescope..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("⭐ Aligning the celestial diffs..."),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌙 Reading your changes by moonlight..."),
            token: tokens::ACCENT_SECONDARY,
        },
        // Nerdy & clever
        ColoredMessage {
            text: Cow::Borrowed("🎲 Rolling for commit inspiration..."),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧬 Decoding the DNA of your changes..."),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔬 Analyzing diff particles..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("📡 Tuning into the commit frequency..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧪 Distilling the essence of your changes..."),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: Cow::Borrowed("⚡ Parsing the diff matrix..."),
            token: tokens::WARNING,
        },
        // Warm & grounded
        ColoredMessage {
            text: Cow::Borrowed("☕ Brewing a fresh commit message..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎨 Painting your changes in prose..."),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧩 Piecing together the story..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎵 Composing a commit symphony..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("💎 Polishing your commit to a shine..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌿 Growing ideas from your diff..."),
            token: tokens::SUCCESS,
        },
        // Playful
        ColoredMessage {
            text: Cow::Borrowed("🚀 Launching into commit space..."),
            token: tokens::ERROR,
        },
        ColoredMessage {
            text: Cow::Borrowed("🗺️ Charting the diff territory..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌊 Riding the code waves..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🦉 Consulting the git guardians..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧭 Calibrating the commit compass..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔐 Unlocking the secrets of your diff..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎁 Wrapping up your changes nicely..."),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🏄 Surfing the staged changes..."),
            token: tokens::SUCCESS,
        },
    ]
});

static REVIEW_WAITING_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        // Cosmic & mystical
        ColoredMessage {
            text: Cow::Borrowed("🔮 Gazing into the code quality crystal..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("✨ Illuminating the hidden corners..."),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌟 Channeling review wisdom..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌙 Meditating on your abstractions..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔭 Scanning the code horizon..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("⭐ Reading the code constellations..."),
            token: tokens::TEXT_PRIMARY,
        },
        // Nerdy & technical
        ColoredMessage {
            text: Cow::Borrowed("🔬 Analyzing code under the microscope..."),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧬 Sequencing your code genome..."),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("📡 Scanning for code anomalies..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧪 Running quality experiments..."),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: Cow::Borrowed("⚡ Tracing the logic pathways..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎲 Rolling perception checks..."),
            token: tokens::WARNING,
        },
        // Exploratory
        ColoredMessage {
            text: Cow::Borrowed("🗺️ Mapping your code architecture..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔍 Hunting for hidden issues..."),
            token: tokens::ERROR,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧭 Navigating your control flow..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("🏊 Diving into the logic depths..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("⛏️ Mining for code gems..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌊 Flowing through your functions..."),
            token: tokens::ACCENT_SECONDARY,
        },
        // Warm & grounded
        ColoredMessage {
            text: Cow::Borrowed("☕ Taking a thoughtful look..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎨 Appreciating your code craft..."),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧩 Piecing together the full picture..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("💎 Searching for rough edges to polish..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🦉 Consulting the wise owl..."),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: Cow::Borrowed("📜 Checking against best practices..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎵 Listening to your code's rhythm..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌿 Tending the code garden..."),
            token: tokens::SUCCESS,
        },
    ]
});

static USER_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        ColoredMessage {
            text: Cow::Borrowed("🚀 Launching..."),
            token: tokens::ERROR,
        },
        ColoredMessage {
            text: Cow::Borrowed("✨ Working magic..."),
            token: tokens::TEXT_PRIMARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔮 Divining..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("⚡ Processing..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌌 Exploring..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔬 Analyzing..."),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: Cow::Borrowed("☕ Brewing..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎨 Crafting..."),
            token: tokens::ACCENT_TERTIARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧩 Piecing..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("💎 Polishing..."),
            token: tokens::ACCENT_SECONDARY,
        },
        ColoredMessage {
            text: Cow::Borrowed("🎵 Composing..."),
            token: tokens::ACCENT_DEEP,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌊 Flowing..."),
            token: tokens::SUCCESS,
        },
        ColoredMessage {
            text: Cow::Borrowed("🔭 Scanning..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🧪 Testing..."),
            token: tokens::WARNING,
        },
        ColoredMessage {
            text: Cow::Borrowed("🌿 Growing..."),
            token: tokens::SUCCESS,
        },
    ]
});

pub fn get_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or(ColoredMessage {
            text: Cow::Borrowed("Processing your request..."),
            token: tokens::WARNING,
        })
}

pub fn get_review_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    REVIEW_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or(ColoredMessage {
            text: Cow::Borrowed("Analyzing your code quality..."),
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
                text: Cow::Borrowed("🔮 Crafting your PR narrative..."),
                token: tokens::ACCENT_DEEP,
            },
            ColoredMessage {
                text: Cow::Borrowed("✨ Weaving your commits into a story..."),
                token: tokens::TEXT_PRIMARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("📝 Summarizing your brilliant work..."),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("🎯 Distilling the essence of your changes..."),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("🌟 Highlighting your contributions..."),
                token: tokens::SUCCESS,
            },
            ColoredMessage {
                text: Cow::Borrowed("📋 Building your PR description..."),
                token: tokens::WARNING,
            },
            ColoredMessage {
                text: Cow::Borrowed("🎨 Painting the PR picture..."),
                token: tokens::ACCENT_TERTIARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("🧵 Threading your commits together..."),
                token: tokens::WARNING,
            },
        ]
    });

static CHANGELOG_WAITING_MESSAGES: std::sync::LazyLock<Vec<ColoredMessage>> =
    std::sync::LazyLock::new(|| {
        vec![
            ColoredMessage {
                text: Cow::Borrowed("📜 Chronicling your changes..."),
                token: tokens::ACCENT_DEEP,
            },
            ColoredMessage {
                text: Cow::Borrowed("✨ Cataloging your accomplishments..."),
                token: tokens::TEXT_PRIMARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("📖 Writing the history of your code..."),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("🏛️ Archiving your progress..."),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("🔖 Tagging your milestones..."),
                token: tokens::SUCCESS,
            },
            ColoredMessage {
                text: Cow::Borrowed("📝 Documenting the journey..."),
                token: tokens::WARNING,
            },
            ColoredMessage {
                text: Cow::Borrowed("🗂️ Organizing your achievements..."),
                token: tokens::ACCENT_TERTIARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("⚡ Capturing the deltas..."),
                token: tokens::WARNING,
            },
        ]
    });

static RELEASE_NOTES_WAITING_MESSAGES: std::sync::LazyLock<Vec<ColoredMessage>> =
    std::sync::LazyLock::new(|| {
        vec![
            ColoredMessage {
                text: Cow::Borrowed("🚀 Preparing launch notes..."),
                token: tokens::ERROR,
            },
            ColoredMessage {
                text: Cow::Borrowed("✨ Polishing the release highlights..."),
                token: tokens::TEXT_PRIMARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("📣 Announcing your achievements..."),
                token: tokens::ACCENT_DEEP,
            },
            ColoredMessage {
                text: Cow::Borrowed("🎉 Celebrating the release..."),
                token: tokens::SUCCESS,
            },
            ColoredMessage {
                text: Cow::Borrowed("📦 Packaging the release story..."),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("🌟 Showcasing new features..."),
                token: tokens::ACCENT_SECONDARY,
            },
            ColoredMessage {
                text: Cow::Borrowed("📢 Composing the release fanfare..."),
                token: tokens::WARNING,
            },
            ColoredMessage {
                text: Cow::Borrowed("🎊 Wrapping up the release..."),
                token: tokens::ACCENT_TERTIARY,
            },
        ]
    });

fn get_pr_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    PR_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or(ColoredMessage {
            text: Cow::Borrowed("Building PR description..."),
            token: tokens::ACCENT_DEEP,
        })
}

fn get_changelog_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    CHANGELOG_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or(ColoredMessage {
            text: Cow::Borrowed("Generating changelog..."),
            token: tokens::ACCENT_SECONDARY,
        })
}

fn get_release_notes_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    RELEASE_NOTES_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or(ColoredMessage {
            text: Cow::Borrowed("Creating release notes..."),
            token: tokens::SUCCESS,
        })
}

pub fn get_user_message() -> ColoredMessage {
    let mut rng = rand::rng();
    USER_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or(ColoredMessage {
            text: Cow::Borrowed("What would you like to do?"),
            token: tokens::ACCENT_SECONDARY,
        })
}
