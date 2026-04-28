//! Changelog file utilities
//!
//! This module provides utilities for managing changelog files.
//! The changelog/release notes *types* are in the `types` module.

use crate::git::GitRepo;
use crate::log_debug;
use anyhow::{Context, Result};
use regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

/// Utilities for changelog file management
pub struct ChangelogGenerator;

impl ChangelogGenerator {
    /// Updates a changelog file with new content
    ///
    /// This function reads the existing changelog file (if it exists), preserves the header,
    /// and prepends the new changelog content while maintaining the file structure.
    ///
    /// # Arguments
    ///
    /// * `changelog_content` - The new changelog content to prepend
    /// * `changelog_path` - Path to the changelog file
    /// * `git_repo` - `GitRepo` instance to use for retrieving commit dates
    /// * `to_ref` - The "to" Git reference (commit/tag) to extract the date from
    /// * `version_name` - Optional custom version name to use instead of version from Git
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error
    #[allow(clippy::too_many_lines)]
    ///
    /// # Errors
    ///
    /// Returns an error when the changelog cannot be read, parsed, or written.
    pub fn update_changelog_file(
        changelog_content: &str,
        changelog_path: &str,
        git_repo: &Arc<GitRepo>,
        to_ref: &str,
        version_name: Option<String>,
    ) -> Result<()> {
        let path = Path::new(changelog_path);
        let commit_date = changelog_commit_date(git_repo, to_ref);
        let clean_content = clean_generated_changelog(changelog_content);
        let mut version_content = extract_version_content(&clean_content);

        apply_version_override(&mut version_content, version_name)?;
        ensure_version_date(&mut version_content, &commit_date);

        let updated_content =
            updated_changelog_content(path, changelog_path, &with_separator(&version_content))?;

        let mut file = fs::File::create(path)
            .with_context(|| format!("Failed to create changelog file: {changelog_path}"))?;

        file.write_all(updated_content.as_bytes())
            .with_context(|| format!("Failed to write to changelog file: {changelog_path}"))?;

        Ok(())
    }
}

fn changelog_commit_date(git_repo: &Arc<GitRepo>, to_ref: &str) -> String {
    match git_repo.get_commit_date(to_ref) {
        Ok(date) => {
            log_debug!("Got commit date for {}: {}", to_ref, date);
            date
        }
        Err(e) => {
            log_debug!("Failed to get commit date for {}: {}", to_ref, e);
            chrono::Local::now().format("%Y-%m-%d").to_string()
        }
    }
}

fn clean_generated_changelog(changelog_content: &str) -> String {
    let stripped_content = strip_ansi_codes(changelog_content);
    if stripped_content.starts_with("━") || stripped_content.starts_with('-') {
        stripped_content
            .find('\n')
            .map_or(stripped_content.clone(), |pos| {
                stripped_content[pos + 1..].to_string()
            })
    } else {
        stripped_content
    }
}

fn extract_version_content(clean_content: &str) -> String {
    clean_content
        .split_once("## [")
        .map_or(clean_content.to_string(), |(_, version)| {
            format!("## [{version}")
        })
}

fn apply_version_override(
    version_content: &mut String,
    version_name: Option<String>,
) -> Result<()> {
    let Some(version) = version_name else {
        return Ok(());
    };

    if !version_content.contains("## [") {
        log_debug!("Could not find version header to replace in changelog content");
        return Ok(());
    }

    let re = regex::Regex::new(r"## \[([^\]]+)\]")
        .context("Failed to compile changelog version regex")?;
    *version_content = re
        .replace(version_content, &format!("## [{version}]"))
        .to_string();
    log_debug!("Replaced version with user-provided version: {}", version);
    Ok(())
}

fn ensure_version_date(version_content: &mut String, commit_date: &str) {
    if version_content.contains(" - \n") {
        *version_content = version_content.replace(" - \n", &format!(" - {commit_date}\n"));
        log_debug!("Replaced empty date with commit date: {}", commit_date);
    } else if version_content.contains("] - ") && !version_content.contains("] - 20") {
        add_date_after_dash(version_content, commit_date);
    } else if !version_content.contains("] - ") {
        add_date_to_version_line(version_content, commit_date);
    }
}

fn add_date_after_dash(version_content: &mut String, commit_date: &str) {
    if let Some((prefix, rest)) = version_content.split_once("] - ") {
        *version_content = format!(
            "{prefix}] - {commit_date}\n{}",
            rest.trim_start_matches(['\n', ' '])
        );
        log_debug!("Added commit date after dash: {}", commit_date);
    }
}

fn add_date_to_version_line(version_content: &mut String, commit_date: &str) {
    let line_end = version_content.find('\n').unwrap_or(version_content.len());
    let version_line = &version_content[..line_end];

    if version_line.contains("## [")
        && let Some(bracket_pos) = version_line.rfind(']')
    {
        *version_content = format!(
            "{} - {}{}",
            &version_content[..=bracket_pos],
            commit_date,
            &version_content[bracket_pos + 1..]
        );
        log_debug!("Added date to version line: {}", commit_date);
    }
}

fn with_separator(version_content: &str) -> String {
    format!(
        "{version_content}\n<!-- -------------------------------------------------------------- -->\n\n"
    )
}

fn updated_changelog_content(
    path: &Path,
    changelog_path: &str,
    version_content: &str,
) -> Result<String> {
    let default_header = "# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n";

    if !path.exists() {
        return Ok(format!("{default_header}{version_content}"));
    }

    let existing_content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read changelog file: {changelog_path}"))?;

    Ok(merge_existing_changelog(
        &existing_content,
        default_header,
        version_content,
    ))
}

fn merge_existing_changelog(
    existing_content: &str,
    default_header: &str,
    version_content: &str,
) -> String {
    if !existing_content.contains("# Changelog") || !existing_content.contains("Keep a Changelog") {
        return format!("{default_header}{version_content}");
    }

    existing_content.split_once("## [").map_or_else(
        || format!("{existing_content}{version_content}"),
        |(header, existing_versions)| format!("{header}{version_content}## [{existing_versions}"),
    )
}

/// Strips ANSI color/style codes from a string
fn strip_ansi_codes(s: &str) -> String {
    let re = regex::Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})*)?[m|K]")
        .expect("Failed to compile ANSI escape code regex");
    re.replace_all(s, "").to_string()
}
