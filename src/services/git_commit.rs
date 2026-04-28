//! Git commit service
//!
//! Focused service for git commit operations. This extracts the commit-specific
//! functionality from the monolithic `IrisCommitService`.

use anyhow::Result;
use std::sync::Arc;

use crate::git::{CommitResult, GitRepo};
use crate::log_debug;

/// Service for performing git commit operations
///
/// This service handles:
/// - Creating commits with optional hook verification
/// - Pre-commit hook execution
/// - Remote repository detection
///
/// It does NOT handle:
/// - LLM operations (handled by `IrisAgentService`)
/// - Context gathering (handled by agents)
/// - Message generation (handled by agents)
pub struct GitCommitService {
    repo: Arc<GitRepo>,
    verify: bool,
}

impl GitCommitService {
    /// Create a new `GitCommitService`
    ///
    /// # Arguments
    /// * `repo` - The git repository to operate on
    /// * `_use_gitmoji` - Retained for API compatibility; commit messages are
    ///   stored exactly as provided
    /// * `verify` - Whether to run pre/post-commit hooks
    #[must_use]
    pub fn new(repo: Arc<GitRepo>, _use_gitmoji: bool, verify: bool) -> Self {
        Self { repo, verify }
    }

    /// Create from an existing `GitRepo` (convenience constructor)
    #[must_use]
    pub fn from_repo(repo: GitRepo, use_gitmoji: bool, verify: bool) -> Self {
        Self::new(Arc::new(repo), use_gitmoji, verify)
    }

    /// Check if the repository is a remote repository
    #[must_use]
    pub fn is_remote(&self) -> bool {
        self.repo.is_remote()
    }

    /// Execute the pre-commit hook if verification is enabled
    ///
    /// Returns Ok(()) if:
    /// - verify is false (hooks disabled)
    /// - repository is remote (hooks don't apply)
    /// - pre-commit hook succeeds
    ///
    /// # Errors
    ///
    /// Returns an error when hook verification is enabled and the pre-commit hook fails.
    pub fn pre_commit(&self) -> Result<()> {
        if self.is_remote() {
            log_debug!("Skipping pre-commit hook for remote repository");
            return Ok(());
        }

        if self.verify {
            self.repo.execute_hook("pre-commit")
        } else {
            Ok(())
        }
    }

    /// Perform a commit with the given message
    ///
    /// This method:
    /// 1. Validates the repository is not remote
    /// 2. Uses the exact message provided
    /// 3. Runs pre-commit hook (if verify is enabled)
    /// 4. Creates the commit
    /// 5. Runs post-commit hook (if verify is enabled)
    ///
    /// # Arguments
    /// * `message` - The commit message to use
    ///
    /// # Returns
    /// The result of the commit operation
    ///
    /// # Errors
    ///
    /// Returns an error when the repository is remote, hooks fail, or Git cannot create the commit.
    pub fn perform_commit(&self, message: &str) -> Result<CommitResult> {
        self.perform_local_change(
            message,
            "commit",
            "Cannot commit to a remote repository",
            GitRepo::commit,
        )
    }

    /// Amend the previous commit with staged changes and a new message
    ///
    /// This method:
    /// 1. Validates the repository is not remote
    /// 2. Uses the exact message provided
    /// 3. Runs pre-commit hook (if verify is enabled)
    /// 4. Amends the commit (replaces HEAD)
    /// 5. Runs post-commit hook (if verify is enabled)
    ///
    /// # Arguments
    /// * `message` - The new commit message
    ///
    /// # Returns
    /// The result of the amend operation
    ///
    /// # Errors
    ///
    /// Returns an error when the repository is remote, hooks fail, or Git cannot amend the commit.
    pub fn perform_amend(&self, message: &str) -> Result<CommitResult> {
        self.perform_local_change(
            message,
            "amend",
            "Cannot amend a commit in a remote repository",
            GitRepo::amend_commit,
        )
    }

    fn perform_local_change(
        &self,
        message: &str,
        action: &str,
        remote_error: &str,
        operation: fn(&GitRepo, &str) -> Result<CommitResult>,
    ) -> Result<CommitResult> {
        if self.is_remote() {
            return Err(anyhow::anyhow!("{remote_error}"));
        }

        log_debug!("Performing {} with message: {}", action, message);

        if !self.verify {
            log_debug!("Skipping pre-commit hook (verify=false)");
            return operation(&self.repo, message);
        }

        self.run_pre_commit_hook()?;
        self.finish_local_change(message, action, operation)
    }

    fn run_pre_commit_hook(&self) -> Result<()> {
        log_debug!("Executing pre-commit hook");
        self.repo
            .execute_hook("pre-commit")
            .inspect(|()| {
                log_debug!("Pre-commit hook executed successfully");
            })
            .inspect_err(|e| {
                log_debug!("Pre-commit hook failed: {}", e);
            })
    }

    fn finish_local_change(
        &self,
        message: &str,
        action: &str,
        operation: fn(&GitRepo, &str) -> Result<CommitResult>,
    ) -> Result<CommitResult> {
        match operation(&self.repo, message) {
            Ok(result) => {
                self.run_post_commit_hook();
                log_debug!("{} performed successfully", capitalized_action(action));
                Ok(result)
            }
            Err(e) => {
                log_debug!("{} failed: {}", capitalized_action(action), e);
                Err(e)
            }
        }
    }

    fn run_post_commit_hook(&self) {
        log_debug!("Executing post-commit hook");
        if let Err(e) = self.repo.execute_hook("post-commit") {
            log_debug!("Post-commit hook failed: {}", e);
        }
    }

    /// Get the message of the HEAD commit
    ///
    /// Useful for amend operations to provide original context
    ///
    /// # Errors
    ///
    /// Returns an error when the HEAD commit cannot be read.
    pub fn get_head_commit_message(&self) -> Result<String> {
        self.repo.get_head_commit_message()
    }

    /// Get a reference to the underlying repository
    #[must_use]
    pub fn repo(&self) -> &GitRepo {
        &self.repo
    }
}

fn capitalized_action(action: &str) -> String {
    let mut chars = action.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_git_commit_service_construction() {
        // This test just verifies the API compiles correctly
        // Real tests would need a mock GitRepo
    }
}
