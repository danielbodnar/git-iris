//! Iris Companion - Ambient awareness for Git workflows
//!
//! Provides session tracking, branch memory, and live file watching
//! to transform Studio into an always-aware development companion.

mod branch_memory;
mod session;
mod storage;
mod watcher;

pub use branch_memory::{BranchMemory, FileFocus};
pub use session::{FileActivity, SessionState};
pub use storage::CompanionStorage;
pub use watcher::{CompanionEvent, FileWatcherService};

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Main companion service that coordinates all subsystems
pub struct CompanionService {
    /// Repository path being watched
    repo_path: PathBuf,
    /// Current session state
    session: Arc<parking_lot::RwLock<SessionState>>,
    /// Storage backend for persistence
    storage: CompanionStorage,
    /// File watcher service (optional - may fail to start)
    watcher: Option<FileWatcherService>,
    /// Channel for receiving companion events
    event_rx: mpsc::UnboundedReceiver<CompanionEvent>,
    /// Channel sender (held to keep channel alive)
    _event_tx: mpsc::UnboundedSender<CompanionEvent>,
}

impl CompanionService {
    /// Create a new companion service for the given repository
    ///
    /// # Errors
    ///
    /// Returns an error when companion storage cannot be initialized.
    pub fn new(repo_path: PathBuf, branch: &str) -> Result<Self> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let storage = CompanionStorage::new(&repo_path)?;
        let session = Arc::new(parking_lot::RwLock::new(load_session(
            &storage, &repo_path, branch,
        )));
        let watcher = start_file_watcher(&repo_path, event_tx.clone());

        Ok(Self {
            repo_path,
            session,
            storage,
            watcher,
            event_rx,
            _event_tx: event_tx,
        })
    }

    /// Get the current session state
    #[must_use]
    pub fn session(&self) -> &Arc<parking_lot::RwLock<SessionState>> {
        &self.session
    }

    /// Load branch memory for the given branch
    ///
    /// # Errors
    ///
    /// Returns an error when the branch memory cannot be read or parsed.
    pub fn load_branch_memory(&self, branch: &str) -> Result<Option<BranchMemory>> {
        self.storage.load_branch_memory(branch)
    }

    /// Save branch memory
    ///
    /// # Errors
    ///
    /// Returns an error when the branch memory cannot be serialized or written.
    pub fn save_branch_memory(&self, memory: &BranchMemory) -> Result<()> {
        self.storage.save_branch_memory(memory)
    }

    /// Save current session state
    ///
    /// # Errors
    ///
    /// Returns an error when the session cannot be serialized or written.
    pub fn save_session(&self) -> Result<()> {
        let session = self.session.read();
        self.storage.save_session(&session)
    }

    /// Record a file touch (opened/modified)
    pub fn touch_file(&self, path: PathBuf) {
        let mut session = self.session.write();
        session.touch_file(path);
    }

    /// Record a commit was made
    pub fn record_commit(&self, hash: String) {
        let mut session = self.session.write();
        session.record_commit(hash);
    }

    /// Try to receive the next companion event (non-blocking)
    pub fn try_recv_event(&mut self) -> Option<CompanionEvent> {
        self.event_rx.try_recv().ok()
    }

    /// Check if file watcher is active
    #[must_use]
    pub fn has_watcher(&self) -> bool {
        self.watcher.is_some()
    }

    /// Get repository path
    #[must_use]
    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }
}

fn load_session(storage: &CompanionStorage, repo_path: &Path, branch: &str) -> SessionState {
    match storage.load_session() {
        Ok(Some(mut session)) if session.branch == branch => {
            repo_path.clone_into(&mut session.repo_path);
            session
        }
        Ok(Some(session)) => {
            tracing::info!(
                "Ignoring session data for branch {} while starting on {}",
                session.branch,
                branch
            );
            SessionState::new(repo_path.to_path_buf(), branch.to_owned())
        }
        Ok(None) => SessionState::new(repo_path.to_path_buf(), branch.to_owned()),
        Err(e) => {
            tracing::warn!("Failed to load companion session; starting fresh: {}", e);
            SessionState::new(repo_path.to_path_buf(), branch.to_owned())
        }
    }
}

fn start_file_watcher(
    repo_path: &Path,
    event_tx: mpsc::UnboundedSender<CompanionEvent>,
) -> Option<FileWatcherService> {
    match FileWatcherService::new(repo_path, event_tx) {
        Ok(watcher) => {
            tracing::info!("Companion file watcher started");
            Some(watcher)
        }
        Err(e) => {
            tracing::warn!(
                "Failed to start file watcher: {}. Companion will run without live updates.",
                e
            );
            None
        }
    }
}

impl Drop for CompanionService {
    fn drop(&mut self) {
        // Try to save session on shutdown
        if let Err(e) = self.save_session() {
            tracing::warn!("Failed to save session on shutdown: {}", e);
        }
    }
}

#[cfg(test)]
mod tests;
