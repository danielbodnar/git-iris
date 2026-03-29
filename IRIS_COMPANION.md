# Iris Companion Mode

> Iris as an ambient, always-present git companion — not just a tool you invoke, but an intelligence that watches, learns, and assists throughout your development flow.

## Vision

**From:** "Invoke Iris when you need something"
**To:** "Iris is always watching, learning, and ready"

Iris becomes a persistent daemon that understands your working context, notices patterns, offers proactive insights, and is ready to assist at any moment — all while staying out of your way until needed.

---

## Core Feature Areas

### 1. Live Awareness Engine

The foundation: Iris maintains real-time awareness of your repository state.

| Feature | Description |
|---------|-------------|
| **File Watcher** | Real-time detection of file saves via `notify` crate |
| **Working Tree Evolution** | Tracks what you've been editing over time, not just current state |
| **Session Context** | "You've been in this file for 40 minutes" |
| **Git Event Hooks** | Reacts to commits, rebases, merges, checkouts, stashes |
| **Change Velocity** | Understands your editing pace and patterns |

**Example Insights:**
- "You've touched 12 files in the last hour"
- "Heavy activity in `src/auth/` today"
- "This is your third time editing this function this session"

---

### 2. Proactive Commit Intelligence

Iris learns your commit patterns and suggests natural breakpoints.

| Feature | Description |
|---------|-------------|
| **Commit Nudges** | "You've edited 6 files over 2 hours — ready to commit?" |
| **Concern Detection** | "These staged changes look like 2 separate concerns" |
| **Test Awareness** | "You modified tests but not implementation — intentional?" |
| **Size Warnings** | "This file grew 200 lines — consider splitting?" |
| **Pattern Learning** | Learns your natural commit cadence |

**Example Scenarios:**
- After sustained editing with a natural pause: "Natural breakpoint? These changes tell a complete story."
- When staging: "This looks like Feature A + Bugfix B mixed together"
- Before leaving: "You have uncommitted changes spanning 3 features"

---

### 3. Smart Staging Assistant

AI-powered staging that understands semantic relationships between changes.

| Feature | Description |
|---------|-------------|
| **Atomic Boundary Detection** | Suggests which files belong together |
| **Semantic Grouping** | "These 3 files are all auth-related — stage together?" |
| **Mixed Change Warnings** | Warns when staging unrelated changes |
| **Visual Groupings** | Studio shows suggested commit boundaries |
| **One-Key Accept** | Quick accept suggested grouping |

**Grouping Signals:**
- Import/dependency relationships
- Test + implementation pairs
- Config + code that uses it
- Related by recent edit proximity
- Common semantic domain (auth, api, ui, etc.)

---

### 4. Branch Context Memory

Per-branch persistent memory — Iris remembers what you were doing.

| Feature | Description |
|---------|-------------|
| **Branch Welcome** | "Welcome back to `feature-oauth` — you were implementing token refresh" |
| **TODO Tracking** | Tracks TODOs added/removed on this branch |
| **WIP Detection** | "You have 3 uncommitted WIP sessions on other branches" |
| **Stash Context** | Auto-labels stashes with meaningful context |
| **Last Position** | Remembers which files/lines you were focused on |
| **Branch Sessions** | Switching branches starts a fresh live session snapshot |
| **Branch Sessions** | Switching branches starts a fresh live session snapshot |

**Example Flow:**
```
$ git checkout feature-auth

Iris: Welcome back to feature-auth
      Last session: debugging token expiration in auth.rs:142
      You noted: "edge case when user is nil"
      3 uncommitted changes from 2 days ago
```

---

### 5. Upstream Intelligence

Stay aware of what's happening in the remote without constant manual checking.

| Feature | Description |
|---------|-------------|
| **Remote Tracking** | "main has 8 new commits since you branched" |
| **PR Branch Watch** | "Someone pushed to your PR branch" |
| **Conflict Prediction** | "Conflict likely: remote changed `auth.rs` which you're editing" |
| **Change Summaries** | Summarize incoming changes in plain English |
| **Rebase Suggestions** | "Good time to rebase — no conflicts detected" |

**Example Notifications:**
- Subtle: "↓ main +5 commits (no conflicts)"
- Warning: "⚠ Upstream modified `config.rs` — you have local changes"
- Urgent: "🔴 Your PR has merge conflicts"

---

### 6. Pre-Commit Guardian

Passive review that catches issues before they become commits.

| Category | Detections |
|----------|------------|
| **Debug Artifacts** | Console.log, print statements, debug flags |
| **Secrets** | API keys, tokens, passwords, connection strings |
| **Incomplete Work** | TODO/FIXME without issue refs, unfinished comments |
| **Code Quality** | Unhandled errors, unused imports, obvious bugs |
| **Style Issues** | Formatting inconsistencies, naming violations |

**Configurable Modes:**
- **Silent** — Only surface critical issues (secrets, obvious bugs)
- **Subtle** — Gentle reminders, non-blocking
- **Thorough** — Full review before every commit

**Example:**
```
Pre-commit scan:
  ⚠ Debug log in src/api.rs:45
  🔴 Possible API key in src/config.rs:12
  💭 TODO without issue ref in src/auth.rs:88

Proceed anyway? [y/N/review]
```

---

### 7. Code Evolution Insights

Long-term awareness of how your codebase evolves.

| Feature | Description |
|---------|-------------|
| **Churn Analysis** | "This function has been modified 8 times in 2 weeks" |
| **Complexity Trends** | "Cyclomatic complexity trending up in this module" |
| **Hot Files** | Identify files that might need refactoring attention |
| **Pattern Detection** | "You often forget to update README when touching CLI args" |
| **Technical Debt Signals** | Growing file sizes, increasing dependencies |

**Example Insights:**
- "🔥 `auth.rs` is a hot file — 12 changes this month"
- "📈 `handlers/` complexity up 23% since last release"
- "💡 You usually update docs when changing API — forgot this time?"

---

### 8. Session Persistence

Context survives across restarts — pick up exactly where you left off.

| Feature | Description |
|---------|-------------|
| **Session Resume** | "Last session: debugging token expiration" |
| **Workspace Notes** | Your notes persist: "edge case when user is nil" |
| **Edit History** | What files you focused on, in what order |
| **Insight History** | What Iris told you, what you acknowledged |
| **Cross-Session Patterns** | Learn from your behavior over time |

Live session data is branch-scoped. When you switch branches, Iris preserves the previous branch memory separately and starts a fresh session snapshot for the new branch.

Live session data is branch-scoped. When you switch branches, Iris preserves the previous branch memory separately and starts a fresh session snapshot for the new branch.

**Persistence Format:**
```
~/.iris/sessions/
  └── {repo-hash}/
      ├── session.json      # Current session state
      ├── branches/         # Per-branch memory
      ├── insights.log      # Insight history
      └── workspace.md      # Persistent notes
```

---

### 9. Quick Action Hotkeys

Instant actions for common operations — zero friction.

| Key | Action | Description |
|-----|--------|-------------|
| `c` | Quick Commit | Commit staged with auto-generated message |
| `C` | Commit + Edit | Generate message, open for editing |
| `a` | Amend | Amend last commit with staged changes |
| `s` | Smart Stash | Stash with auto-generated context label |
| `S` | Stash Pop | Pop most recent stash |
| `p` | Push | Push with safety checks |
| `P` | Force Push | Push with lease (safe force) |
| `r` | Regenerate | Re-run current generation (commit, PR, etc.) |
| `?` | Suggest | "What should I do next?" |
| `/` | Chat | Open chat with Iris |

**Safety Checks on Push:**
- "You're pushing 3 commits, one has a TODO — continue?"
- "This will push to main — are you sure?"
- "Remote has changes you haven't pulled"

---

### 10. Notification System

Configurable alerts that stay out of your way until important.

| Level | Behavior |
|-------|----------|
| **Silent** | No notifications, check manually |
| **Subtle** | Status line only, no interruptions |
| **Normal** | Status line + occasional desktop notifications |
| **Chatty** | All insights surfaced proactively |

**Notification Categories:**
- 🔴 **Critical**: Secrets detected, merge conflicts, CI failure
- 🟡 **Warning**: Upstream changes, uncommitted work, long session
- 🔵 **Info**: Commit suggestions, pattern observations
- ⚪ **Subtle**: Background status updates

**Desktop Integration:**
- macOS: Native notifications via `notify-rust`
- Linux: D-Bus notifications
- Optional: Terminal bell for critical only

---

### 11. Learning & Adaptation

Iris gets smarter about *you* over time.

| Learning Area | Examples |
|---------------|----------|
| **Commit Style** | Message length, emoji usage, conventional commits |
| **Work Patterns** | When you commit, how long you edit, break patterns |
| **Code Preferences** | File organization, naming conventions |
| **Review Habits** | What warnings you dismiss vs. address |
| **Project Conventions** | Per-repo learned patterns |

**Adaptation Examples:**
- "You usually write longer messages for API changes"
- "You prefer to commit tests separately"
- "You always update CHANGELOG for features, not fixes"
- Adjusts suggestion timing to your rhythm

---

## Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                       Iris Daemon                               │
├───────────────┬───────────────┬───────────────┬────────────────┤
│  File Watch   │  Git Events   │   Upstream    │    Session     │
│    Engine     │    Handler    │    Poller     │    Memory      │
│   (notify)    │   (hooks)     │   (fetch)     │   (sled/json)  │
├───────────────┴───────────────┴───────────────┴────────────────┤
│                    Context Aggregator                           │
│          Builds unified view of "what's happening"              │
│   - Current changes    - Session timeline    - Branch state    │
│   - Edit velocity      - Upstream delta      - Stash stack     │
├─────────────────────────────────────────────────────────────────┤
│                     Insight Engine                              │
│        Rules + LLM hybrid for proactive suggestions             │
│   - Pattern matching (fast, local)                              │
│   - LLM analysis (deeper understanding)                         │
│   - Learning model (adapts over time)                           │
├─────────────────────────────────────────────────────────────────┤
│                    Presentation Layer                           │
│   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│   │ Studio TUI  │  │  CLI Output │  │   Desktop   │            │
│   │  (primary)  │  │  (minimal)  │  │   Notifs    │            │
│   └─────────────┘  └─────────────┘  └─────────────┘            │
└─────────────────────────────────────────────────────────────────┘
```

### New State Concepts

```rust
/// What's happened this session
struct SessionState {
    started_at: DateTime<Utc>,
    files_touched: HashMap<PathBuf, FileActivity>,
    commits_made: Vec<CommitRef>,
    insights_shown: Vec<InsightId>,
    notes: Vec<WorkspaceNote>,
}

/// Per-branch persistent memory
struct BranchMemory {
    branch_name: String,
    last_visited: DateTime<Utc>,
    last_focus: Option<FileFocus>,  // file:line you were on
    notes: Vec<String>,
    uncommitted_sessions: u32,
}

/// Evolution of changes over time
struct WorkingTreeTimeline {
    snapshots: VecDeque<TreeSnapshot>,
    change_velocity: ChangeVelocity,
    hot_files: Vec<HotFile>,
}

/// Pending proactive suggestions
struct InsightQueue {
    pending: VecDeque<Insight>,
    dismissed: HashSet<InsightId>,
    snoozed: HashMap<InsightId, DateTime<Utc>>,
}
```

### Event Flow

```
File Save ─────┐
               │
Git Commit ────┼──▶ Event Bus ──▶ Context Aggregator ──▶ Insight Engine
               │                         │                     │
Timer Tick ────┤                         │                     │
               │                         ▼                     ▼
Upstream Poll ─┘                   Session State          Insight Queue
                                         │                     │
                                         └─────────┬───────────┘
                                                   │
                                                   ▼
                                            Presentation
                                         (TUI / Notifications)
```

---

## Implementation Phases

### Phase 1: Foundation
- [ ] Daemon mode infrastructure (start/stop/status)
- [ ] File watcher integration (`notify` crate)
- [ ] Session state persistence
- [ ] Basic event bus

### Phase 2: Awareness
- [ ] Git event detection (commits, checkouts, etc.)
- [ ] Working tree timeline tracking
- [ ] Branch memory system
- [ ] Upstream polling

### Phase 3: Intelligence
- [ ] Insight engine framework
- [ ] Commit boundary detection
- [ ] Pre-commit guardian rules
- [ ] Pattern learning basics

### Phase 4: Integration
- [ ] Studio TUI daemon mode
- [ ] Desktop notifications
- [ ] Quick action hotkeys
- [ ] Configuration system

### Phase 5: Learning
- [ ] Commit style learning
- [ ] Work pattern adaptation
- [ ] Per-project conventions
- [ ] Insight effectiveness tracking

---

## User Experience

### Starting Iris Companion

```bash
# Start daemon in current repo
$ iris watch

# Start with Studio TUI
$ iris studio --watch

# Check daemon status
$ iris status

# Stop daemon
$ iris stop
```

### Studio Integration

Companion mode enhances Studio with:
- **Status Bar**: Live awareness indicators
- **Insight Panel**: Proactive suggestions sidebar
- **Timeline View**: Session activity visualization
- **Branch Drawer**: Quick branch context switching

### Example Session

```
$ iris studio --watch

┌─ Iris Studio ─────────────────────────────────────────────────┐
│                                                                │
│  📍 feature-oauth (3 ahead, 2 behind main)                    │
│                                                                │
│  ┌─ Changes ──────────────────────────────────────────────┐   │
│  │  M src/auth/token.rs        [staged]                   │   │
│  │  M src/auth/refresh.rs      [staged]                   │   │
│  │  M src/config.rs            [unstaged]                 │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                │
│  ┌─ Insights ─────────────────────────────────────────────┐   │
│  │  💡 Staged changes look like a complete feature        │   │
│  │     Ready to commit?                          [c]ommit │   │
│  │                                                        │   │
│  │  ⚠️  config.rs has unrelated changes                   │   │
│  │     Consider separate commit                  [s]tage  │   │
│  │                                                        │   │
│  │  ↓  main has 2 new commits (no conflicts)              │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                │
│  Session: 47m │ Files: 8 │ Last commit: 23m ago               │
└────────────────────────────────────────────────────────────────┘
```

---

## Configuration

```toml
# ~/.config/git-iris/companion.toml

[watch]
enabled = true
poll_interval = "30s"
upstream_check_interval = "5m"

[notifications]
level = "normal"  # silent | subtle | normal | chatty
desktop = true
sound = false

[insights]
commit_nudge_after = "30m"
concern_detection = true
pre_commit_scan = true
pre_commit_level = "subtle"  # silent | subtle | thorough

[learning]
enabled = true
commit_style = true
work_patterns = true

[guardian]
detect_secrets = true
detect_debug = true
detect_todos = true
block_secrets = true  # Hard block on detected secrets
```

---

## Open Questions

1. **Daemon vs. Integrated**: Separate daemon process or integrated into Studio?
2. **Resource Usage**: How aggressive should file watching be?
3. **Privacy**: What data stays local vs. could be synced?
4. **Multi-Repo**: Support watching multiple repos simultaneously?
5. **IDE Integration**: LSP-style protocol for editor plugins?

---

## Success Metrics

- **Time to Commit**: Faster path from change to committed code
- **Commit Quality**: Better atomic commits, fewer "oops" follow-ups
- **Context Retention**: Less mental overhead switching branches
- **Issue Prevention**: Secrets/debug code caught before commit
- **Developer Satisfaction**: Feels like a helpful companion, not a nag
