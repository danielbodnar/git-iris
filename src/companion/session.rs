//! Session state tracking for Iris Companion
//!
//! Tracks files touched, time elapsed, and commits made during a session.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Activity tracking for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileActivity {
    /// Path to the file
    pub path: PathBuf,
    /// When this file was first touched in the session
    pub first_touched: DateTime<Utc>,
    /// When this file was last touched
    pub last_touched: DateTime<Utc>,
    /// Number of times this file was touched
    pub touch_count: u32,
}

impl FileActivity {
    /// Create a new file activity record
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            path,
            first_touched: now,
            last_touched: now,
            touch_count: 1,
        }
    }

    /// Record another touch
    pub fn touch(&mut self) {
        self.last_touched = Utc::now();
        self.touch_count += 1;
    }
}

/// Session state for the current Studio session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    /// Unique session identifier
    pub session_id: Uuid,
    /// Repository path
    pub repo_path: PathBuf,
    /// Current branch name
    pub branch: String,
    /// When the session started
    pub started_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Most recent commit timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_commit_at: Option<DateTime<Utc>>,
    /// Files touched during this session
    pub files_touched: HashMap<PathBuf, FileActivity>,
    /// Commits made during this session (hashes)
    pub commits_made: Vec<String>,
}

impl SessionState {
    /// Create a new session
    #[must_use]
    pub fn new(repo_path: PathBuf, branch: String) -> Self {
        let now = Utc::now();
        Self {
            session_id: Uuid::new_v4(),
            repo_path,
            branch,
            started_at: now,
            last_activity: now,
            last_commit_at: None,
            files_touched: HashMap::new(),
            commits_made: Vec::new(),
        }
    }

    /// Record a file touch
    pub fn touch_file(&mut self, path: PathBuf) {
        self.last_activity = Utc::now();
        let normalized_path = self.normalize_path(path);
        self.files_touched
            .entry(normalized_path.clone())
            .and_modify(FileActivity::touch)
            .or_insert_with(|| FileActivity::new(normalized_path));
    }

    /// Record a commit
    pub fn record_commit(&mut self, hash: String) {
        let now = Utc::now();
        self.last_activity = now;
        self.last_commit_at = Some(now);
        self.commits_made.push(hash);
    }

    /// Get session duration
    #[must_use]
    pub fn duration(&self) -> chrono::Duration {
        Utc::now() - self.started_at
    }

    /// Get number of files touched
    #[must_use]
    pub fn files_count(&self) -> usize {
        self.files_touched.len()
    }

    /// Get files ordered by most recently touched
    #[must_use]
    pub fn recent_files(&self) -> Vec<&FileActivity> {
        let mut files: Vec<_> = self.files_touched.values().collect();
        files.sort_by_key(|f| std::cmp::Reverse(f.last_touched));
        files
    }

    /// Get time since last commit (if any)
    #[must_use]
    pub fn time_since_last_commit(&self) -> Option<chrono::Duration> {
        self.last_commit_at
            .map(|last_commit_at| Utc::now() - last_commit_at)
    }

    /// Update branch and start a fresh session for the new branch
    pub fn set_branch(&mut self, branch: String) {
        self.session_id = Uuid::new_v4();
        self.started_at = Utc::now();
        self.last_activity = self.started_at;
        self.last_commit_at = None;
        self.files_touched.clear();
        self.commits_made.clear();
        self.branch = branch;
    }

    /// Normalize a path so the same file is tracked consistently
    fn normalize_path(&self, path: PathBuf) -> PathBuf {
        if path.is_absolute()
            && let Ok(relative) = path.strip_prefix(&self.repo_path)
        {
            return relative.to_path_buf();
        }

        path
    }
}
