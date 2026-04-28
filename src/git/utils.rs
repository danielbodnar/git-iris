use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use crate::log_debug;

static EXCLUDE_PATH_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"(^|/)\.git(/|$)",
        r"(^|/)\.svn(/|$)",
        r"(^|/)\.hg(/|$)",
        r"(^|/)\.DS_Store$",
        r"(^|/)node_modules(/|$)",
        r"(^|/)target(/|$)",
        r"(^|/)build(/|$)",
        r"(^|/)dist(/|$)",
        r"(^|/)\.vscode(/|$)",
        r"(^|/)\.idea(/|$)",
        r"(^|/)\.vs(/|$)",
    ]
    .into_iter()
    .map(|pattern| Regex::new(pattern).expect("exclude path regex should compile"))
    .collect()
});

static EXCLUDE_FILE_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"package-lock\.json$",
        r"\.lock$",
        r"\.log$",
        r"\.tmp$",
        r"\.temp$",
        r"\.swp$",
        r"\.min\.js$",
    ]
    .into_iter()
    .map(|pattern| Regex::new(pattern).expect("exclude file regex should compile"))
    .collect()
});

/// Checks if the current directory is inside a Git work tree.
///
/// # Returns
///
/// A Result containing a boolean indicating if inside a work tree or an error.
///
/// # Errors
///
/// Returns an error only if the Git command cannot be spawned. Git reporting a
/// non-repository directory is normalized to `Ok(false)`.
pub fn is_inside_work_tree() -> Result<bool> {
    let status = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(exit) => Ok(exit.success()),
        Err(_) => Ok(false),
    }
}

/// Determines if the given diff represents a binary file.
#[must_use]
pub fn is_binary_diff(diff: &str) -> bool {
    diff.contains("Binary files")
        || diff.contains("GIT binary patch")
        || diff.contains("[Binary file changed]")
}

/// Executes a git command and returns the output as a string
///
/// # Arguments
///
/// * `args` - The arguments to pass to git
///
/// # Returns
///
/// A Result containing the output as a String or an error.
///
/// # Errors
///
/// Returns an error when the Git command fails or emits invalid UTF-8 output.
pub fn run_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Git command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout =
        String::from_utf8(output.stdout).context("Invalid UTF-8 output from git command")?;

    Ok(stdout.trim().to_string())
}

/// Checks if a file should be excluded from analysis.
///
/// Excludes common directories and files that don't contribute meaningfully
/// to commit context (build artifacts, lock files, IDE configs, etc.)
#[must_use]
pub fn should_exclude_file(path: &str) -> bool {
    log_debug!("Checking if file should be excluded: {}", path);
    let path = Path::new(path);
    let excluded = path_matches(path) || file_name_matches(path);

    if excluded {
        log_debug!("File excluded: {}", path.display());
    } else {
        log_debug!("File not excluded: {}", path.display());
    }

    excluded
}

fn path_matches(path: &Path) -> bool {
    path.to_str()
        .is_some_and(|path| EXCLUDE_PATH_PATTERNS.iter().any(|re| re.is_match(path)))
}

fn file_name_matches(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| EXCLUDE_FILE_PATTERNS.iter().any(|re| re.is_match(name)))
}
