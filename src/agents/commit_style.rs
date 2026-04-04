use crate::context::RecentCommit;
use regex::Regex;
use std::sync::LazyLock;

const MIN_CONVENTIONAL_SAMPLES: usize = 4;
const MIN_CONVENTIONAL_MATCHES: usize = 3;
const MIN_CONVENTIONAL_CONFIDENCE_NUMERATOR: usize = 3;
const MIN_CONVENTIONAL_CONFIDENCE_DENOMINATOR: usize = 5;

static CONVENTIONAL_COMMIT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-z][a-z0-9-]*(\([^)]+\))?!?: [^\s].+$")
        .expect("conventional commit regex must compile")
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConventionalCommitStyle {
    examples: Vec<String>,
}

impl ConventionalCommitStyle {
    #[must_use]
    pub(crate) fn examples(&self) -> &[String] {
        &self.examples
    }
}

#[must_use]
pub(crate) fn detect_conventional_commit_style(
    commits: &[RecentCommit],
) -> Option<ConventionalCommitStyle> {
    let subjects: Vec<&str> = commits
        .iter()
        .map(|commit| first_subject_line(&commit.message))
        .filter(|subject| !should_ignore_subject(subject))
        .collect();

    if subjects.len() < MIN_CONVENTIONAL_SAMPLES {
        return None;
    }

    let matching_subjects: Vec<&str> = subjects
        .iter()
        .copied()
        .filter(|subject| CONVENTIONAL_COMMIT_RE.is_match(subject))
        .collect();

    if matching_subjects.len() < MIN_CONVENTIONAL_MATCHES
        || matching_subjects.len() * MIN_CONVENTIONAL_CONFIDENCE_DENOMINATOR
            < subjects.len() * MIN_CONVENTIONAL_CONFIDENCE_NUMERATOR
    {
        return None;
    }

    Some(ConventionalCommitStyle {
        examples: matching_subjects
            .into_iter()
            .take(3)
            .map(ToOwned::to_owned)
            .collect(),
    })
}

fn first_subject_line(message: &str) -> &str {
    message.lines().next().unwrap_or_default().trim()
}

fn should_ignore_subject(subject: &str) -> bool {
    let normalized = subject.trim().to_ascii_lowercase();

    normalized.is_empty()
        || normalized.starts_with("merge ")
        || normalized.starts_with("fixup!")
        || normalized.starts_with("squash!")
}
