# Adding Studio Modes

Studio modes are interactive TUI interfaces for specific workflows. Each mode combines state management, input handling, and rendering to create a focused user experience. This guide shows you how to add a new mode to Iris Studio.

## What is a Studio Mode?

A mode is a complete user interface for a specific task:

- **Explore Mode**: Navigate codebase with semantic understanding
- **Commit Mode**: Generate and edit commit messages
- **Review Mode**: AI-powered code reviews
- **PR Mode**: Pull request descriptions
- **Changelog Mode**: Structured changelog generation

Each mode has:

1. **State** — Data specific to this mode
2. **Handler** — Input processing logic
3. **Renderer** — UI drawing code

## Architecture: Reducer-Centric Event Flow

Studio uses a predictable reducer-centered state management pattern:

```
Input Event
    ↓
Handler (maps input → StudioEvent)
    ↓
Reducer (central state/event layer)
    ↓
Side Effects (spawn agent, load data, etc.)
    ↓
State Updated
    ↓
Renderer (draw UI from state)
```

**Key principle**: The reducer remains the shared event-processing core, but Studio is not a fully
pure reducer architecture end-to-end. Handlers and `StudioApp` still apply some immediate UI and
coordination updates directly, while reducer-driven flows return explicit side effects as data.

## Step-by-Step: Adding a New Mode

### Example: Feature Summary Mode

::: tip Teaching Example
This section walks through creating a hypothetical "Feature Summary" mode. **This mode does not exist in the current codebase** — it's a complete example to illustrate the pattern. Follow along to understand how modes work, then apply the same structure to your own mode.
:::

Let's create a mode that displays AI-generated feature summaries.

### Step 1: Add Mode Variant

Edit `src/studio/state/mod.rs`:

```rust
/// Available modes in Iris Studio
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mode {
    #[default]
    Explore,
    Commit,
    Review,
    PR,
    Changelog,
    ReleaseNotes,
    FeatureSummary,  // Add your mode
}

impl Mode {
    pub fn display_name(&self) -> &'static str {
        match self {
            // ... existing modes ...
            Mode::FeatureSummary => "Feature Summary",
        }
    }

    pub fn shortcut(&self) -> char {
        match self {
            // ... existing modes ...
            Mode::FeatureSummary => 'F',
        }
    }

    pub fn is_available(&self) -> bool {
        matches!(
            self,
            Mode::Explore
                | Mode::Commit
                | Mode::Review
                | Mode::PR
                | Mode::Changelog
                | Mode::ReleaseNotes
                | Mode::FeatureSummary  // Mark as available
        )
    }

    pub fn all() -> &'static [Mode] {
        &[
            Mode::Explore,
            Mode::Commit,
            Mode::Review,
            Mode::PR,
            Mode::Changelog,
            Mode::ReleaseNotes,
            Mode::FeatureSummary,  // Add to list
        ]
    }
}
```

### Step 2: Create State Struct

Edit `src/studio/state/modes.rs`. Mode state structs follow the `*State` naming convention (not `*Mode`) — the shipped structs are `ExploreState`, `CommitState`, `ReviewState`, `PrState`, `ChangelogState`, and `ReleaseNotesState`:

```rust
/// Feature Summary mode state
#[derive(Debug, Clone, Default)]
pub struct FeatureSummaryState {
    /// Base branch to compare against
    pub from_ref: String,
    /// Feature branch to summarize
    pub to_ref: String,
    /// Generated summary content
    pub summary_content: String,
    /// Whether we're currently generating
    pub generating: bool,
    /// Scroll offset for summary view
    pub scroll_offset: usize,
    /// Panel state for file list (if showing files)
    pub file_list: Vec<String>,
    pub file_list_selected: usize,
}

impl FeatureSummaryState {
    pub fn new() -> Self {
        Self {
            from_ref: "<default-branch>".to_string(),
            to_ref: "HEAD".to_string(),
            ..Default::default()
        }
    }

    /// Scroll summary down
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_add(lines);
    }

    /// Scroll summary up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Select next file in list
    pub fn select_next_file(&mut self) {
        if !self.file_list.is_empty() {
            self.file_list_selected = (self.file_list_selected + 1) % self.file_list.len();
        }
    }

    /// Select previous file in list
    pub fn select_prev_file(&mut self) {
        if !self.file_list.is_empty() && self.file_list_selected > 0 {
            self.file_list_selected -= 1;
        } else if !self.file_list.is_empty() {
            self.file_list_selected = self.file_list.len() - 1;
        }
    }
}
```

Add to `ModeStates`. The shipped fields are `explore`, `commit`, `review`, `pr`, `changelog`, and `release_notes`, all of the `*State` types above:

```rust
/// Container for all mode states
#[derive(Debug, Default)]
pub struct ModeStates {
    pub explore: ExploreState,
    pub commit: CommitState,
    pub review: ReviewState,
    pub pr: PrState,
    pub changelog: ChangelogState,
    pub release_notes: ReleaseNotesState,
    pub feature_summary: FeatureSummaryState,  // Add here
}
```

### Step 3: Create Input Handler

Create `src/studio/handlers/feature_summary.rs`:

```rust
//! Feature Summary mode key handling

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::studio::events::{AgentTask, SideEffect};
use crate::studio::state::{Modal, PanelId, RefSelectorTarget, StudioState};

use super::copy_to_clipboard;

/// Handle key events in Feature Summary mode
pub fn handle_feature_summary_key(state: &mut StudioState, key: KeyEvent) -> Vec<SideEffect> {
    match state.focused_panel {
        PanelId::Left => handle_files_key(state, key),
        PanelId::Center => handle_summary_key(state, key),
        PanelId::Right => handle_diff_key(state, key),
    }
}

fn handle_files_key(state: &mut StudioState, key: KeyEvent) -> Vec<SideEffect> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.modes.feature_summary.select_next_file();
            state.mark_dirty();
            vec![]
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.modes.feature_summary.select_prev_file();
            state.mark_dirty();
            vec![]
        }
        KeyCode::Enter => {
            // Switch to diff view for selected file
            state.focused_panel = PanelId::Right;
            state.mark_dirty();
            vec![]
        }
        _ => vec![],
    }
}

fn handle_summary_key(state: &mut StudioState, key: KeyEvent) -> Vec<SideEffect> {
    match key.code {
        // Scrolling
        KeyCode::Char('j') | KeyCode::Down => {
            state.modes.feature_summary.scroll_down(1);
            state.mark_dirty();
            vec![]
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.modes.feature_summary.scroll_up(1);
            state.mark_dirty();
            vec![]
        }
        KeyCode::PageDown | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.modes.feature_summary.scroll_down(20);
            state.mark_dirty();
            vec![]
        }
        KeyCode::PageUp | KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.modes.feature_summary.scroll_up(20);
            state.mark_dirty();
            vec![]
        }

        // Generate summary
        KeyCode::Char('r') => {
            state.set_iris_thinking("Generating feature summary...");
            state.modes.feature_summary.generating = true;
            vec![spawn_feature_summary_task(state)]
        }

        // Select base branch
        KeyCode::Char('b') => {
            let refs = state.get_branch_refs();
            state.modal = Some(Modal::RefSelector {
                input: String::new(),
                refs,
                selected: 0,
                target: RefSelectorTarget::FeatureSummaryFrom,
            });
            state.mark_dirty();
            vec![]
        }

        // Select feature branch
        KeyCode::Char('f') => {
            let refs = state.get_branch_refs();
            state.modal = Some(Modal::RefSelector {
                input: String::new(),
                refs,
                selected: 0,
                target: RefSelectorTarget::FeatureSummaryTo,
            });
            state.mark_dirty();
            vec![]
        }

        // Copy to clipboard
        KeyCode::Char('y') => {
            let content = &state.modes.feature_summary.summary_content;
            if !content.is_empty() {
                copy_to_clipboard(state, content, "Feature summary");
            }
            vec![]
        }

        _ => vec![],
    }
}

fn handle_diff_key(state: &mut StudioState, key: KeyEvent) -> Vec<SideEffect> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            // Scroll diff view
            state.mark_dirty();
            vec![]
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.mark_dirty();
            vec![]
        }
        _ => vec![],
    }
}

/// Spawn task to generate feature summary
///
/// `SideEffect::SpawnAgent` carries a typed `AgentTask` variant — not a boxed
/// async future. Extend the `AgentTask` enum in `src/studio/events.rs` with a
/// `FeatureSummary { from_ref, to_ref }` variant, then return it from here. The
/// actual call to `IrisAgentService` happens inside `src/studio/app/agent_tasks.rs`
/// when the app drains `SpawnAgent` effects.
fn spawn_feature_summary_task(state: &StudioState) -> SideEffect {
    SideEffect::SpawnAgent {
        task: AgentTask::FeatureSummary {
            from_ref: state.modes.feature_summary.from_ref.clone(),
            to_ref: state.modes.feature_summary.to_ref.clone(),
        },
    }
}
```

When the app executes the new `AgentTask::FeatureSummary` variant, it constructs the service and calls `execute_task` (or `execute_task_with_prompt`). The real signatures live in `src/agents/setup.rs`:

```rust
// IrisAgentService::new takes four args and is NOT fallible.
pub fn new(config: Config, provider: String, model: String, fast_model: String) -> Self { ... }

// Task execution uses execute_task with a structured TaskContext...
pub async fn execute_task(
    &self,
    capability: &str,
    context: TaskContext,
) -> Result<StructuredResponse> { ... }

// ...or execute_task_with_prompt for a pre-built prompt string.
pub async fn execute_task_with_prompt(
    &self,
    capability: &str,
    task_prompt: &str,
) -> Result<StructuredResponse> { ... }
```

There is no `execute_capability(name, &[(key, value)])` method.

Add to `src/studio/handlers/mod.rs`:

```rust
pub mod feature_summary;
pub use feature_summary::handle_feature_summary_key;
```

Update the mode dispatch in `handle_key_event` (in `src/studio/handlers/mod.rs`, around line 45) — `handlers/global.rs` does not exist; cross-mode dispatch lives in `mod.rs`:

```rust
match state.active_mode {
    Mode::Explore => handle_explore_key(state, key),
    Mode::Commit => handle_commit_key(state, key),
    Mode::Review => handle_review_key(state, key),
    Mode::PR => handle_pr_key(state, key),
    Mode::Changelog => handle_changelog_key(state, key),
    Mode::ReleaseNotes => handle_release_notes_key(state, key),
    Mode::FeatureSummary => handle_feature_summary_key(state, key),
}
```

You also need to register the mode's keyboard shortcut in `handle_global_key` (same file, around line 59). Each shipped mode has a hard-coded `matches_shift_char` check; add one for your mode:

```rust
if matches_shift_char(&key, 'f') {
    return Some(switch_mode(state, Mode::FeatureSummary));
}
```

### Step 4: Create Renderer

Create `src/studio/render/feature_summary.rs`:

```rust
//! Feature Summary mode rendering

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::studio::state::{PanelId, StudioState};
use crate::studio::theme;

/// Render a panel in Feature Summary mode
pub fn render_feature_summary_panel(
    state: &mut StudioState,
    frame: &mut Frame,
    area: Rect,
    panel_id: PanelId,
) {
    let is_focused = panel_id == state.focused_panel;
    let theme = theme::current();

    match panel_id {
        PanelId::Left => {
            // File list panel
            render_file_list(state, frame, area, is_focused, &theme);
        }
        PanelId::Center => {
            // Summary content panel
            render_summary(state, frame, area, is_focused, &theme);
        }
        PanelId::Right => {
            // Diff view panel (optional)
            render_diff(state, frame, area, is_focused, &theme);
        }
    }
}

fn render_file_list(
    state: &StudioState,
    frame: &mut Frame,
    area: Rect,
    is_focused: bool,
    theme: &theme::Theme,
) {
    let files = &state.modes.feature_summary.file_list;
    let selected = state.modes.feature_summary.file_list_selected;

    let items: Vec<ListItem> = files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if i == selected {
                Style::default()
                    .fg(theme.colors.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.colors.text)
            };

            let marker = if i == selected { "▸" } else { " " };
            ListItem::new(Line::from(vec![
                Span::styled(marker, style),
                Span::raw(" "),
                Span::styled(file, style),
            ]))
        })
        .collect();

    let border_style = if is_focused {
        Style::default().fg(theme.colors.accent)
    } else {
        Style::default().fg(theme.colors.border)
    };

    let title = format!("Files · {}", files.len());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title);

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn render_summary(
    state: &StudioState,
    frame: &mut Frame,
    area: Rect,
    is_focused: bool,
    theme: &theme::Theme,
) {
    let content = &state.modes.feature_summary.summary_content;
    let scroll = state.modes.feature_summary.scroll_offset;

    let border_style = if is_focused {
        Style::default().fg(theme.colors.accent)
    } else {
        Style::default().fg(theme.colors.border)
    };

    // Build title with refs
    let from = &state.modes.feature_summary.from_ref;
    let to = &state.modes.feature_summary.to_ref;
    let title = format!("Summary · {} → {}", from, to);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Render content with scrolling
    if content.is_empty() {
        let placeholder = if state.modes.feature_summary.generating {
            "Generating feature summary..."
        } else {
            "Press 'r' to generate feature summary\n\
             Press 'b' to select base branch\n\
             Press 'f' to select feature branch"
        };

        let para = Paragraph::new(placeholder)
            .style(Style::default().fg(theme.colors.text_dim))
            .wrap(Wrap { trim: false });

        frame.render_widget(para, inner);
    } else {
        // Render markdown content (simplified - use proper markdown rendering in real impl)
        let lines: Vec<Line> = content
            .lines()
            .skip(scroll)
            .take(inner.height as usize)
            .map(|line| Line::from(line))
            .collect();

        let para = Paragraph::new(lines)
            .style(Style::default().fg(theme.colors.text))
            .wrap(Wrap { trim: false });

        frame.render_widget(para, inner);
    }
}

fn render_diff(
    state: &StudioState,
    frame: &mut Frame,
    area: Rect,
    is_focused: bool,
    theme: &theme::Theme,
) {
    let border_style = if is_focused {
        Style::default().fg(theme.colors.accent)
    } else {
        Style::default().fg(theme.colors.border)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title("Diff");

    // Render diff for selected file (simplified)
    let placeholder = Paragraph::new("Diff view")
        .block(block)
        .style(Style::default().fg(theme.colors.text_dim));

    frame.render_widget(placeholder, area);
}
```

Register the renderer module in `src/studio/render/mod.rs`:

```rust
pub mod feature_summary;
pub use feature_summary::render_feature_summary_panel;
```

The per-mode renderer dispatch does **not** live in `render/mod.rs` — that file only contains module declarations and re-exports. The actual `match state.active_mode { ... }` happens in `render_panel_content` inside `src/studio/app/mod.rs` (around line 2093). Add your arm there:

```rust
fn render_panel_content(&mut self, frame: &mut Frame, area: Rect, panel_id: PanelId) {
    match self.state.active_mode {
        Mode::Explore => render_explore_panel(&mut self.state, frame, area, panel_id),
        Mode::Commit => render_commit_panel(&mut self.state, frame, area, panel_id),
        Mode::Review => render_review_panel(&mut self.state, frame, area, panel_id),
        Mode::PR => render_pr_panel(&mut self.state, frame, area, panel_id),
        Mode::Changelog => render_changelog_panel(&mut self.state, frame, area, panel_id),
        Mode::ReleaseNotes => {
            render_release_notes_panel(&mut self.state, frame, area, panel_id);
        }
        Mode::FeatureSummary => {
            render_feature_summary_panel(&mut self.state, frame, area, panel_id);
        }
    }
}
```

The shipped renderers all receive a single `panel_id` and decide internally which panel to draw, so panel area iteration is already handled by `render_panels` further up in `app/mod.rs`.

### Step 5: Add Side Effects

The reducer lives in `src/studio/reducer/` as a directory module (`mod.rs` plus `agent.rs`, `content.rs`, `git.rs`, `modal.rs`, `navigation.rs`, `settings.rs`, `ui.rs`). It reduces `StudioEvent` values, mutates state, and returns `SideEffect` values for the app to execute — it does not match on side effects itself.

If your mode needs anything beyond `SideEffect::SpawnAgent { task: AgentTask::FeatureSummary { ... } }` (added in Step 3), add the new variant to the `SideEffect` enum in `src/studio/events.rs`:

```rust
#[derive(Debug, Clone)]
pub enum SideEffect {
    // ... existing effects ...

    /// Pre-load feature-summary inputs before the agent runs
    PrepareFeatureSummary {
        from_ref: String,
        to_ref: String,
    },
}
```

The app's effect dispatcher (not the reducer) is responsible for actually executing the effect — wire your new variant into the app loop alongside `SpawnAgent`, `LoadData`, etc.

### Step 6: Update Focus Defaults

Edit `src/studio/state/mod.rs` in the `switch_mode` method:

```rust
pub fn switch_mode(&mut self, new_mode: Mode) {
    // ... existing code ...

    // Set default focus based on mode
    self.focused_panel = match new_mode {
        Mode::Commit => PanelId::Center,
        Mode::Review | Mode::PR | Mode::Changelog | Mode::ReleaseNotes => PanelId::Center,
        Mode::FeatureSummary => PanelId::Center,  // Add here
        Mode::Explore => PanelId::Left,
    };
    self.dirty = true;
}
```

### Step 7: Test Your Mode

```bash
just build
just studio
```

Make sure you've also added the `matches_shift_char(&key, 'f')` line to `handle_global_key` (Step 3) — mode shortcuts are dispatched explicitly per mode, not auto-derived from `Mode::shortcut()`.

In Studio:

- Press `Shift+F` to switch to Feature Summary mode
- Test navigation with `j`/`k`, `Tab`
- Test generating with `r`
- Test branch selection with `b` and `f`

## Component Reuse

Studio provides reusable components in `src/studio/components/`:

### File Tree

```rust
use crate::studio::components::render_file_tree;

render_file_tree(
    frame,
    area,
    &mut state.modes.my_mode.file_tree,
    "Files",
    is_focused,
);
```

### Diff View

```rust
use crate::studio::components::render_diff_view;

render_diff_view(
    frame,
    area,
    &state.modes.my_mode.diff_view,
    "Changes",
    is_focused,
);
```

### Message Editor

```rust
use crate::studio::components::render_message_editor;

render_message_editor(
    frame,
    area,
    &state.modes.my_mode.message_editor,
    "Message",
    is_focused,
    generating,
);
```

### Code View

```rust
use crate::studio::components::render_code_view;

render_code_view(
    frame,
    area,
    &content,
    Some(&language),
    scroll_offset,
    is_focused,
);
```

## Best Practices

### State Design

**Keep state minimal:**

```rust
pub struct MyState {
    pub essential_data: String,
    pub scroll_offset: usize,
    // Don't store derived data - compute on render
}
```

**Use clear field names:**

```rust
pub struct MyState {
    pub from_ref: String,      // Good - clear purpose
    pub to_ref: String,         // Good
    pub data: String,           // Bad - vague
    pub temp: Vec<String>,      // Bad - unclear
}
```

### Handler Design

**Return side effects, don't execute:**

```rust
// Good
KeyCode::Char('r') => {
    state.modes.my_mode.generating = true;
    vec![SideEffect::SpawnAgent { task: ... }]
}

// Bad - executes directly
KeyCode::Char('r') => {
    tokio::spawn(async { ... });  // Don't do this!
    vec![]
}
```

**Keep handlers focused:**

```rust
// Good - separate concerns
fn handle_file_list_key(...) -> Vec<SideEffect> { ... }
fn handle_content_key(...) -> Vec<SideEffect> { ... }

pub fn handle_my_mode_key(...) -> Vec<SideEffect> {
    match state.focused_panel {
        PanelId::Left => handle_file_list_key(state, key),
        PanelId::Center => handle_content_key(state, key),
        PanelId::Right => handle_diff_key(state, key),
    }
}
```

### Renderer Design

**Compute dimensions from available space:**

```rust
fn render_my_panel(frame: &mut Frame, area: Rect, ...) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Header
            Constraint::Min(10),        // Content
            Constraint::Length(1),      // Footer
        ])
        .split(area);

    render_header(frame, chunks[0], ...);
    render_content(frame, chunks[1], ...);
    render_footer(frame, chunks[2], ...);
}
```

**Use theme colors:**

```rust
use crate::studio::theme;

let theme = theme::current();

let text_style = Style::default().fg(theme.colors.text);
let accent_style = Style::default().fg(theme.colors.accent);
let dim_style = Style::default().fg(theme.colors.text_dim);
```

**Handle empty states:**

```rust
if content.is_empty() {
    let placeholder = if generating {
        "Generating..."
    } else {
        "Press 'r' to generate"
    };

    render_placeholder(frame, area, placeholder);
} else {
    render_content(frame, area, content);
}
```

## Panel Layout Patterns

### Three-Panel Layout (Files | Content | Detail)

Used by Commit, Review modes:

```rust
let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(20),  // File list
        Constraint::Percentage(50),  // Main content
        Constraint::Percentage(30),  // Details/diff
    ])
    .split(area);
```

### Two-Panel Layout (List | Content)

Used by Explore mode:

```rust
let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(30),  // Navigation
        Constraint::Percentage(70),  // Content
    ])
    .split(area);
```

### Single-Panel Layout (Full Content)

For focused workflows:

```rust
// Use entire area for content
render_content(frame, area, ...);
```

## Keyboard Navigation Standards

Follow Studio conventions:

| Key       | Action                |
| --------- | --------------------- |
| `j`/`k`   | Navigate up/down      |
| `h`/`l`   | Navigate left/right   |
| `g`/`G`   | Jump to top/bottom    |
| `Tab`     | Cycle panels          |
| `Ctrl+D`  | Page down             |
| `Ctrl+U`  | Page up               |
| `r`       | Regenerate/refresh    |
| `e`       | Edit                  |
| `y`       | Copy to clipboard     |
| `/`       | Open chat             |
| `?`       | Show help             |
| `Esc`     | Close modal/cancel    |
| `Shift+C` | Switch to Commit mode |
| `Shift+R` | Switch to Review mode |

**Mode-specific keys** (like `b` for "select base branch") are fine, but document them in help.

## Event Flow Example

**User presses `r` to regenerate:**

1. Handler receives `KeyCode::Char('r')`
2. Handler updates state: `state.modes.my_mode.generating = true`
3. Handler returns `SideEffect::SpawnAgent { task }`
4. Reducer processes effect, spawns async task
5. Task completes, sends result via channel
6. App loop receives result, dispatches `StudioEvent::AgentComplete`
7. Reducer updates state: `state.modes.my_mode.content = result`
8. Next render cycle draws updated content

## Real-World Examples

Study these complete mode implementations:

### Commit Mode

- **State**: `src/studio/state/modes.rs` → `CommitState`
- **Handler**: `src/studio/handlers/commit.rs`
- **Renderer**: `src/studio/render/commit.rs`

**Learn from**: Message editing, emoji selection, staged file handling

### Review Mode

- **State**: `src/studio/state/modes.rs` → `ReviewState`
- **Handler**: `src/studio/handlers/review.rs`
- **Renderer**: `src/studio/render/review.rs`

**Learn from**: Ref selection, markdown rendering, scrolling

### PR Mode

- **State**: `src/studio/state/modes.rs` → `PrState`
- **Handler**: `src/studio/handlers/pr.rs`
- **Renderer**: `src/studio/render/pr.rs`

**Learn from**: Branch comparison, commit history display

## Testing Your Mode

### Manual Testing Checklist

- [ ] Mode switches correctly from other modes
- [ ] Default panel focus is correct
- [ ] All keybindings work as expected
- [ ] Panel navigation with Tab works
- [ ] Scrolling works (if applicable)
- [ ] Content generates correctly
- [ ] Copy to clipboard works
- [ ] Modal interactions work (ref selector, etc.)
- [ ] Theme colors apply correctly
- [ ] Empty states display properly
- [ ] Error states handled gracefully

### Debug Your Mode

```bash
# Run with verbose logging
just run-debug -- studio

# Check for panics
just studio 2> errors.log
```

## Next Steps

- **Add capabilities** that your mode uses → [Adding Capabilities](./capabilities.md)
- **Create tools** to gather mode-specific data → [Adding Tools](./tools.md)
- **Contribute** your mode back → [Contributing](./contributing.md)
