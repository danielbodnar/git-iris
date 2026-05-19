# UI Components

**Guide to reusable UI components in Iris Studio.**

## Philosophy

Studio components are **stateful widgets** that:

1. **Own their display state** (scroll position, selection, etc.)
2. **Provide pure render functions** (no side effects)
3. **Are reusable** across multiple modes
4. **Emit semantic updates** (no direct event handling)

## Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Component                              │
│                                                             │
│  ┌──────────────────┐                                       │
│  │ Component State  │ (scroll, selection, cursor, etc.)    │
│  └────────┬─────────┘                                       │
│           │                                                 │
│           ▼                                                 │
│  ┌──────────────────┐                                       │
│  │  Update Methods  │ (scroll_down, select_next, etc.)     │
│  └────────┬─────────┘                                       │
│           │                                                 │
│           ▼                                                 │
│  ┌──────────────────┐                                       │
│  │ Render Function  │ (draw to Ratatui frame)              │
│  │  render_xxx()    │                                       │
│  └──────────────────┘                                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Key insight:** Components manage **display state**, not **business logic**.

## Core Components

### FileTreeState

**Purpose:** Hierarchical file browser with git status.

**Location:** `src/studio/components/file_tree.rs`

#### State Structure

```rust
#[derive(Debug, Clone)]
pub struct FileTreeState {
    root: Vec<TreeNode>,           // Tree root nodes
    expanded: HashSet<PathBuf>,    // Expanded directory paths
    selected: usize,               // Selected index in the flat view
    scroll_offset: usize,          // Scroll offset
    flat_cache: Vec<FlatEntry>,    // Cached flat view (rebuilt on demand)
    cache_dirty: bool,
}
```

The selection is an **index into the flat view**, not a path. Path lookups go through `selected_path()`/`selected_entry()`.

#### Key Methods

```rust
impl FileTreeState {
    pub fn new() -> Self;

    /// Build a tree from a flat list of paths plus git status hints
    pub fn from_paths(paths: &[PathBuf], git_statuses: &[(PathBuf, FileGitStatus)]) -> Self;

    /// Get the rendered flat view (rebuilds cache if needed)
    pub fn flat_view(&mut self) -> &[FlatEntry];

    /// Get selected entry / path
    pub fn selected_entry(&mut self) -> Option<FlatEntry>;
    pub fn selected_path(&mut self) -> Option<PathBuf>;
    pub fn selected_index(&self) -> usize;

    /// Navigation
    pub fn select_prev(&mut self);
    pub fn select_next(&mut self);
    pub fn select_first(&mut self);
    pub fn select_last(&mut self);
    pub fn page_up(&mut self, page_size: usize);
    pub fn page_down(&mut self, page_size: usize);
    pub fn select_path(&mut self, path: &Path) -> bool;

    /// Expansion
    pub fn toggle_expand(&mut self);
    pub fn expand(&mut self);
    pub fn collapse(&mut self);
    pub fn expand_all(&mut self);
    pub fn collapse_all(&mut self);
    pub fn expand_to_depth(&mut self, max_depth: usize);

    /// Mouse support
    pub fn select_by_row(&mut self, row: usize) -> bool;
    pub fn handle_click(&mut self, row: usize) -> (bool, bool);
    pub fn is_row_selected(&self, row: usize) -> bool;

    /// Render-time viewport bookkeeping
    pub fn update_scroll(&mut self, visible_height: usize);
}
```

#### TreeNode + FlatEntry

```rust
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub git_status: FileGitStatus,
    pub depth: usize,
    pub children: Vec<TreeNode>,
}

/// A flattened view entry for rendering. The flat cache is the output of
/// walking the tree, respecting the `expanded` set.
pub struct FlatEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub git_status: FileGitStatus,
    pub depth: usize,
    pub is_expanded: bool,
    pub has_children: bool,
}
```

`from_paths` builds the tree by walking each path's components and calling the internal `insert_path` helper. After construction the first two levels are auto-expanded so something is visible immediately.

#### Git Status

```rust
pub enum FileGitStatus {
    Normal,
    Staged,      // ● (green)
    Modified,    // ○ (yellow)
    Untracked,   // ? (cyan)
    Deleted,     // ✕ (red)
    Renamed,     // → (green)
    Conflict,    // ! (red)
}
```

Each status has indicator character and color.

#### Rendering

```rust
pub fn render_file_tree(
    frame: &mut Frame,
    area: Rect,
    state: &mut FileTreeState,  // mutable: render may rebuild the flat cache
    title: &str,
    focused: bool,
);
```

**Visual:**

```
╭─ Files ────────────────────╮
│                            │
│ ● src/                     │  <- Staged directory
│   ● main.rs                │  <- Staged file (selected)
│   ○ lib.rs                 │  <- Modified file
│   ▸ components/            │  <- Collapsed directory
│ ? docs/                    │  <- Untracked directory
│   ? README.md              │
│                            │
╰────────────────────────────╯
```

**Features:**

- Tree structure with indentation
- Git status indicators
- Expand/collapse arrows (▾/▸)
- Selection highlight
- Scroll indicators
- Focus border styling

### CodeViewState

**Purpose:** Syntax-highlighted source code display.

**Location:** `src/studio/components/code_view.rs`

#### State Structure

```rust
#[derive(Debug, Clone, Default)]
pub struct CodeViewState {
    current_file: Option<PathBuf>,        // Path of the loaded file
    lines: Vec<String>,                   // File content as lines
    scroll_offset: usize,                 // Top visible line index
    selected_line: usize,                 // 1-indexed selected line (0 = none)
    selection: Option<(usize, usize)>,    // 1-indexed inclusive selection range
}
```

Syntax highlighting is *not* stored here — it's computed on the fly by `SyntaxHighlighter::for_path()` when the view renders, based on the file extension.

#### Key Methods

```rust
impl CodeViewState {
    pub fn new() -> Self;

    /// Load file content from disk (returns io::Result)
    pub fn load_file(&mut self, path: &Path) -> std::io::Result<()>;

    /// Accessors
    pub fn current_file(&self) -> Option<&Path>;
    pub fn lines(&self) -> &[String];
    pub fn line_count(&self) -> usize;
    pub fn is_loaded(&self) -> bool;
    pub fn scroll_offset(&self) -> usize;
    pub fn selected_line(&self) -> usize;
    pub fn selection(&self) -> Option<(usize, usize)>;

    /// Navigation (1-indexed for line numbers)
    pub fn scroll_up(&mut self, amount: usize);
    pub fn scroll_down(&mut self, amount: usize);
    pub fn scroll_to_line(&mut self, line: usize, visible_height: usize);
    pub fn move_up(&mut self, amount: usize, visible_height: usize);
    pub fn move_down(&mut self, amount: usize, visible_height: usize);
    pub fn goto_first(&mut self);
    pub fn goto_last(&mut self, visible_height: usize);

    /// Selection
    pub fn set_selected_line(&mut self, line: usize);
    pub fn set_selection(&mut self, start: usize, end: usize);
    pub fn clear_selection(&mut self);

    /// Mouse support
    pub fn select_by_row(&mut self, row: usize) -> bool;
}
```

#### Rendering

```rust
pub fn render_code_view(
    frame: &mut Frame,
    area: Rect,
    state: &CodeViewState,
    title: &str,
    focused: bool,
);
```

**Visual:**

```
╭─ src/main.rs ──────────────────────────────────────────────╮
│                                                            │
│   1  use std::io;                                          │
│   2                                                        │
│   3  fn main() {                                           │
│   4      println!("Hello, world!");                        │
│   5  }                                                     │
│                                                            │
│                                                     [1/142]│
╰────────────────────────────────────────────────────────────╯
```

**Features:**

- Line numbers
- Syntax highlighting (via tree-sitter or syntect)
- Scroll position indicator
- Highlighted line ranges
- Gutter for git blame info
- Focus border styling

### DiffViewState

**Purpose:** Unified/split diff rendering with hunks.

**Location:** `src/studio/components/diff_view.rs`

#### State Structure

```rust
#[derive(Debug, Clone)]
pub struct DiffViewState {
    diffs: Vec<FileDiff>,           // All file diffs
    selected_file: usize,           // Currently displayed file index
    scroll_offset: usize,           // Scroll offset within selected file
    selected_line: usize,           // Selected line within selected file
    cached_lines: Vec<DiffLine>,    // Pre-flattened lines for current file
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub path: PathBuf,
    pub old_path: Option<PathBuf>,   // For renames
    pub is_new: bool,
    pub is_deleted: bool,
    pub is_binary: bool,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub header: String,              // The "@@ -a,b +c,d @@" header
    pub lines: Vec<DiffLine>,
    pub old_start: usize, pub old_count: usize,
    pub new_start: usize, pub new_count: usize,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
    pub old_line_num: Option<usize>,
    pub new_line_num: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineType {
    Context,
    Added,
    Removed,
    HunkHeader,    // The "@@ ... @@" line
    FileHeader,    // The synthesized file-name header
    Empty,
}
```

There is no `status`/`stats`/`show_context` field, no `DiffLineKind` enum, no `NoNewline` variant. Per-file diff stats are derived: call `FileDiff::lines_changed() -> (added, removed)`.

#### Key Methods

```rust
impl DiffViewState {
    pub fn new() -> Self;
    pub fn set_diffs(&mut self, diffs: Vec<FileDiff>);

    /// Accessors
    pub fn current_diff(&self) -> Option<&FileDiff>;
    pub fn file_count(&self) -> usize;
    pub fn lines(&self) -> &[DiffLine];          // cached flat view for current file
    pub fn scroll_offset(&self) -> usize;
    pub fn selected_file_index(&self) -> usize;
    pub fn file_paths(&self) -> Vec<&Path>;

    /// File navigation
    pub fn next_file(&mut self);
    pub fn prev_file(&mut self);
    pub fn select_file(&mut self, index: usize);
    pub fn select_file_by_path(&mut self, path: &Path) -> bool;

    /// Scrolling
    pub fn scroll_up(&mut self, amount: usize);
    pub fn scroll_down(&mut self, amount: usize);
    pub fn scroll_to_top(&mut self);
    pub fn scroll_to_bottom(&mut self);

    /// Hunk navigation (jumps scroll_offset to the next/prev hunk header)
    pub fn next_hunk(&mut self);
    pub fn prev_hunk(&mut self);
}

/// Standalone parser (NOT a method on DiffViewState)
pub fn parse_diff(diff_text: &str) -> Vec<FileDiff>;
```

The flow for loading diffs is `let diffs = parse_diff(unified_diff_text); state.set_diffs(diffs);`.

#### Rendering

```rust
pub fn render_diff_view(
    frame: &mut Frame,
    area: Rect,
    state: &DiffViewState,
    title: &str,
    focused: bool,
);

/// Compact summary line for diff lists
pub fn render_diff_summary(diff: &FileDiff) -> Line<'static>;
```

**Visual:**

```
╭─ Diff (3 files, +42/-18) ──────────────────────────────────╮
│                                                            │
│ ● src/main.rs (+12/-5)                                     │
│ @@ -10,7 +10,8 @@ fn main() {                               │
│    use std::io;                                            │
│                                                            │
│ -  fn old_function() {                                     │
│ +  fn new_function() {                                     │
│ +      // Added feature                                    │
│        println!("Hello");                                  │
│    }                                                       │
│                                                            │
│ ○ src/lib.rs (+18/-8)                                      │
│ ? tests/test.rs (+12/-5)                                   │
│                                                            │
╰────────────────────────────────────────────────────────────╯
```

**Features:**

- File-level navigation (`next_file`/`prev_file`/`select_file_by_path`)
- Hunk navigation (`next_hunk`/`prev_hunk`) anchored on `DiffLineType::HunkHeader`
- Color-coded additions/deletions with old/new line numbers per side
- Per-file diff stats via `FileDiff::lines_changed() -> (added, removed)`
- New / deleted / renamed / binary file markers

### MessageEditorState

**Purpose:** Text editor for commit messages.

**Location:** `src/studio/components/message_editor.rs`

#### State Structure

```rust
pub struct MessageEditorState {
    /// Text area (from tui-textarea crate)
    textarea: TextArea<'static>,
    /// Generated messages from Iris
    generated_messages: Vec<GeneratedMessage>,
    /// Currently selected message index
    selected_message: usize,
    /// Edit mode (view vs edit)
    edit_mode: bool,
    /// Original message (for reset)
    original_message: String,
}
```

#### Key Methods

```rust
impl MessageEditorState {
    pub fn new() -> Self;

    /// Replace all generated messages
    pub fn set_messages(&mut self, messages: Vec<GeneratedMessage>);

    /// Append more generated messages without losing the existing ones.
    /// Returns the index of the first newly-added message.
    pub fn add_messages(&mut self, messages: Vec<GeneratedMessage>) -> usize;

    /// Variant navigation
    pub fn next_message(&mut self);
    pub fn prev_message(&mut self);
    pub fn message_count(&self) -> usize;
    pub fn selected_index(&self) -> usize;
    pub fn current_generated(&self) -> Option<&GeneratedMessage>;

    /// View / edit mode
    pub fn enter_edit_mode(&mut self);
    pub fn exit_edit_mode(&mut self);
    pub fn is_editing(&self) -> bool;

    /// Editing
    pub fn handle_key(&mut self, key: KeyEvent) -> bool;   // returns true if key was consumed
    pub fn get_message(&self) -> String;
    pub fn is_modified(&self) -> bool;
    pub fn reset(&mut self);
    pub fn clear(&mut self);

    /// Render helper
    pub fn textarea(&self) -> &TextArea<'static>;
}
```

The editor wraps `ratatui_textarea::TextArea`; `handle_key` (not `input`) is the entry point for key events while in edit mode, and it returns `true` once the key has been consumed.

#### Rendering

```rust
pub fn render_message_editor(
    frame: &mut Frame,
    area: Rect,
    state: &MessageEditorState,
    title: &str,
    focused: bool,
    generating: bool,
    status_message: Option<&str>,
);
```

`generating` toggles the braille spinner placeholder; `status_message` is the optional dynamic status string from the fast model that replaces the default "Iris is crafting your commit message…" hint.

**Visual (View Mode):**

```
╭─ Message (1/3) ────────────────────────────────────────────╮
│                                                            │
│ ✨ feat: Add user authentication                           │
│                                                            │
│ Implement JWT-based authentication with:                  │
│ - Login/logout endpoints                                   │
│ - Token refresh mechanism                                  │
│ - Role-based access control                                │
│                                                            │
│                                         [VIEW] [←/→ cycle] │
╰────────────────────────────────────────────────────────────╯
```

**Visual (Edit Mode):**

```
╭─ Message (1/3) [EDIT] ─────────────────────────────────────╮
│                                                            │
│ ✨ feat: Add user authentication█                          │
│                                                            │
│ Implement JWT-based authentication with:                  │
│ - Login/logout endpoints                                   │
│ - Token refresh mechanism                                  │
│ - Role-based access control                                │
│                                                            │
│                                  [ESC cancel] [Enter save] │
╰────────────────────────────────────────────────────────────╯
```

**Features:**

- Multi-line text editing
- Cursor positioning
- Message variant cycling
- View/edit mode toggle
- Reset to original
- Character count

## Component Patterns

### Pattern 1: Stateful Widget

Component owns display state:

```rust
pub struct MyComponentState {
    pub items: Vec<Item>,
    pub selected: usize,
    pub scroll_offset: usize,
}

impl MyComponentState {
    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
        self.scroll_to_selection();
    }
}
```

### Pattern 2: Pure Render

Render function has no side effects:

```rust
pub fn render_my_component(
    frame: &mut Frame,
    area: Rect,
    state: &MyComponentState,
    focused: bool,
) {
    // Only draws to frame, no state mutation
    let items: Vec<_> = state.items.iter()
        .map(|item| Line::from(item.name.clone()))
        .collect();

    let list = List::new(items)
        .highlight_style(theme::highlight());

    frame.render_stateful_widget(list, area, &mut state.selected);
}
```

### Pattern 3: Constructor Helpers

Most components provide a plain `new()` plus one or more domain-specific constructors:

```rust
impl FileTreeState {
    pub fn new() -> Self;
    pub fn from_paths(paths: &[PathBuf], git_statuses: &[(PathBuf, FileGitStatus)]) -> Self;
}

impl MessageEditorState {
    pub fn new() -> Self;
    // Then mutate: state.set_messages(...) or state.add_messages(...)
}
```

There's no chained `.with_X()` builder convention in Studio today — callers either start empty and mutate, or pass everything to a domain constructor.

### Pattern 4: Event Emission

Components don't handle events directly, they return what changed:

```rust
// BAD: Component handles events
impl MyComponent {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Enter {
            self.spawn_agent();  // Side effect!
        }
    }
}

// GOOD: Component updates state, caller decides action
impl MyComponent {
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Enter => Some(Action::Confirm),
            KeyCode::Up => { self.select_prev(); None }
            _ => None
        }
    }
}
```

## Component Composition

Components compose into mode layouts:

```
┌────────────────────────────────────────────────────────────┐
│                     Commit Mode                            │
├──────────────────┬─────────────────────┬───────────────────┤
│                  │                     │                   │
│  FileTreeState   │  MessageEditorState │   DiffViewState   │
│                  │                     │                   │
│  ╭─ Files ────╮  │  ╭─ Message ─────╮  │  ╭─ Diff ──────╮ │
│  │ ● main.rs  │  │  │ ✨ feat: ...   │  │  │ +  added    │ │
│  │ ○ lib.rs   │  │  │               │  │  │ -  removed  │ │
│  │ ? test.rs  │  │  │ Description   │  │  │    context  │ │
│  ╰────────────╯  │  ╰───────────────╯  │  ╰─────────────╯ │
│                  │                     │                   │
└──────────────────┴─────────────────────┴───────────────────┘
```

Each component is independent, mode state owns all component states.

## Component State Management

### Where State Lives

**Component state** (scroll, selection) lives in component struct.

**Business state** (generated messages, file content) lives in mode state.

**Example:**

```rust
pub struct CommitState {
    // Business state
    pub messages: Vec<GeneratedMessage>,
    pub current_index: usize,
    pub custom_instructions: String,
    pub selected_file_index: usize,
    pub editing_message: bool,
    pub generating: bool,
    pub use_gitmoji: bool,
    pub emoji_mode: EmojiMode,
    pub preset: String,
    pub show_all_files: bool,
    pub amend_mode: bool,
    pub original_message: Option<String>,

    // Component states
    pub message_editor: MessageEditorState,
    pub diff_view: DiffViewState,
    pub file_tree: FileTreeState,
}
```

Notice the struct is `CommitState`, not `CommitMode`. The same `*State` naming convention applies to every other mode (`ExploreState`, `ReviewState`, `PrState`, `ChangelogState`, `ReleaseNotesState`).

### State Updates

**Component state** updated directly:

```rust
state.modes.commit.file_tree.select_next();
state.modes.commit.diff_view.scroll_down(5);
```

**Business state** updated via reducer:

```rust
StudioEvent::AgentComplete { result: AgentResult::CommitMessages(messages), .. } => {
    let first_new_index = state.modes.commit.messages.len();
    state.modes.commit.messages.extend(messages.clone());
    state.modes.commit.current_index = first_new_index;
    state.modes.commit.message_editor.add_messages(messages);
}
```

## Advanced Components

### Syntax Highlighting

**Location:** `src/studio/components/syntax.rs`

```rust
pub struct SyntaxHighlighter {
    syntax: Option<&'static SyntaxReference>,  // None when no syntax matches
}

impl SyntaxHighlighter {
    /// Pick a syntax by file extension (e.g. "rs", "py")
    pub fn for_extension(ext: &str) -> Self;

    /// Convenience: derive the extension from a Path
    pub fn for_path(path: &Path) -> Self;

    pub fn is_available(&self) -> bool;

    /// Highlight one line of source. Falls back to a single plain-text span
    /// when no syntax was matched.
    pub fn highlight_line(&self, line: &str) -> Vec<(Style, String)>;
    pub fn highlight_lines(&self, lines: &[String]) -> Vec<Vec<(Style, String)>>;
}
```

`SYNTAX_SET` and `THEME_SET` are statics loaded once via `LazyLock` (defaults from syntect). `highlight_line` looks up `base16-ocean.dark` first, then any fallback theme, and remaps each syntect color into the SilkCircuit palette through `syntect_color_to_silkcircuit`. `CodeViewState` builds a fresh `SyntaxHighlighter::for_path` on every render — there's no caching on the state struct itself.

### Scrollbar

**Ratatui built-in:**

```rust
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};

let scrollbar = Scrollbar::default()
    .orientation(ScrollbarOrientation::VerticalRight)
    .begin_symbol(Some("↑"))
    .end_symbol(Some("↓"));

let mut scrollbar_state = ScrollbarState::new(total_lines)
    .position(scroll_offset);

frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
```

### Modal Overlays

`render/modals/` is a directory of 12 modal renderers, one per modal kind:

```
src/studio/render/modals/
  chat_modal.rs       confirm.rs           help.rs
  commit_count.rs     emoji_selector.rs    instructions.rs
  mod.rs              preset_selector.rs   ref_selector.rs
  search.rs           settings.rs          theme_selector.rs
```

Each modal sizes its own area inside `mod.rs` (which dispatches on `Modal` variant), clears the area with `Clear`, and renders its content with a bordered `Block`. There is no generic `render_modal_overlay` / `centered_rect` helper — modals tailor their geometry to their contents.

## Component Testing

Components are easy to test:

```rust
#[test]
fn test_file_tree_selection() {
    let paths = vec![
        PathBuf::from("a.rs"),
        PathBuf::from("b.rs"),
        PathBuf::from("c.rs"),
    ];
    let statuses = vec![
        (PathBuf::from("b.rs"), FileGitStatus::Modified),
        (PathBuf::from("c.rs"), FileGitStatus::Staged),
    ];

    let mut tree = FileTreeState::from_paths(&paths, &statuses);

    // Initial selection is the first flat entry
    assert_eq!(tree.selected_path(), Some(PathBuf::from("a.rs")));

    tree.select_next();
    assert_eq!(tree.selected_path(), Some(PathBuf::from("b.rs")));

    tree.select_next();
    assert_eq!(tree.selected_path(), Some(PathBuf::from("c.rs")));

    // Selection clamps at the end (does NOT wrap)
    tree.select_next();
    assert_eq!(tree.selected_path(), Some(PathBuf::from("c.rs")));
}

#[test]
fn test_diff_view_per_file_stats() {
    let diffs = parse_diff(SAMPLE_UNIFIED_DIFF);
    let mut diff_view = DiffViewState::new();
    diff_view.set_diffs(diffs);

    let current = diff_view.current_diff().expect("at least one file diff");
    let (added, removed) = current.lines_changed();
    assert!(added + removed > 0);
}
```

There is no `stats_summary()` API — diff stats are per-file via `FileDiff::lines_changed()`.

## Performance Considerations

**Lazy rendering:** Only iterate the visible window. Every render function in `components/` does this — for example, `render_code_view` skips `scroll_offset` lines and takes `visible_height` lines from `state.lines()` before applying syntax highlighting.

**Cache + dirty flag:** `FileTreeState` keeps a `flat_cache: Vec<FlatEntry>` plus a `cache_dirty: bool`. Anything that changes the visible shape of the tree (`expand`, `collapse`, `set_root`, `from_paths`) sets `cache_dirty = true`; the next call to `flat_view()` rebuilds the cache. `DiffViewState` uses the same pattern with `cached_lines` (refreshed in `update_cache()` whenever `set_diffs` or `*_file` is called).

```rust
fn rebuild_cache(&mut self) {
    self.flat_cache.clear();
    let root_clone = self.root.clone();
    for node in &root_clone {
        self.flatten_node(node);
    }
    self.cache_dirty = false;
}
```

**Batching:** Update once, render once.

```rust
// BAD: Multiple renders
state.diff_view.scroll_down(1);
frame.render(...);  // Render
state.diff_view.scroll_down(1);
frame.render(...);  // Render again

// GOOD: Batch updates
state.diff_view.scroll_down(2);
frame.render(...);  // Render once
```

## Styling with SilkCircuit Theme

All components use theme functions:

```rust
use crate::studio::theme;

// Text colors
let text = Span::styled("Hello", theme::text());
let dimmed = Span::styled("(optional)", theme::dimmed());
let keyword = Span::styled("fn", theme::keyword());

// Git status
let staged = Span::styled("●", theme::git_staged());
let modified = Span::styled("○", theme::git_modified());

// Highlights
let selected = Span::styled("Item", theme::highlight());
let focused = Block::default()
    .border_style(theme::focus_border());

// Notifications
let success = Span::styled("✓", theme::success());
let error = Span::styled("✗", theme::error());
let warning = Span::styled("⚠", theme::warning());
```

**Consistency:** All components use same color palette.

## Creating a New Component

### 1. Define State

```rust
pub struct MyComponentState {
    pub items: Vec<String>,
    pub selected: usize,
    pub scroll_offset: usize,
    pub viewport_height: usize,
}
```

### 2. Implement Methods

```rust
impl MyComponentState {
    pub fn new() -> Self { ... }

    pub fn select_next(&mut self) { ... }
    pub fn select_prev(&mut self) { ... }

    pub fn scroll_down(&mut self) { ... }
    pub fn scroll_up(&mut self) { ... }
}
```

### 3. Create Render Function

```rust
pub fn render_my_component(
    frame: &mut Frame,
    area: Rect,
    state: &MyComponentState,
    focused: bool,
) {
    let items: Vec<Line> = state.items.iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == state.selected {
                theme::highlight()
            } else {
                theme::text()
            };
            Line::from(Span::styled(item, style))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(if focused {
                theme::focus_border()
            } else {
                theme::dimmed()
            }));

    frame.render_widget(list, area);
}
```

### 4. Add to Mode State

```rust
pub struct MyState {
    pub my_component: MyComponentState,
}
```

### 5. Use in Render

```rust
pub fn render_my_mode_panel(
    frame: &mut Frame,
    areas: &LayoutAreas,
    state: &StudioState,
) {
    // areas.panels is a Vec<Rect> with one entry per panel (Left, Center, Right)
    render_my_component(
        frame,
        areas.panels[1], // Center
        &state.modes.my_mode.my_component,
        state.focused_panel == PanelId::Center,
    );
}
```

## Common Component Utilities

Shared helpers live in `src/studio/utils.rs` and are used across every component.

### Truncation

```rust
/// Truncate by character count, append "..." if cut. Use for log previews
/// and other text where unicode display width isn't critical.
pub fn truncate_chars(s: &str, max_chars: usize) -> String;

/// Truncate by unicode display width, append "…". Use for any TUI rendering
/// where CJK / emoji / combining characters matter.
pub fn truncate_width(s: &str, max_width: usize) -> String;
```

`truncate_width` is the workhorse — `file_tree`, `code_view`, `diff_view`, and `message_editor` all call it when they need to fit a label inside a column-bounded area. `truncate_chars` is re-exported from `state::chat` as `truncate_preview` for chat history previews.

### Tab + control-character handling

```rust
/// Expand tabs to spaces (next multiple of `tab_width`) and strip control
/// characters. Essential when displaying file content or diff lines in the
/// TUI: raw tabs corrupt alignment, control codes corrupt the entire screen.
pub fn expand_tabs(s: &str, tab_width: usize) -> String;
```

Both `CodeViewState` rendering and `DiffViewState` rendering pipe each line through `expand_tabs(line, 4)` before measuring width or applying highlights.

### Layout helpers

Mode-level layout lives in `src/studio/layout.rs` and is not really a "component utility" — `calculate_layout(area, mode)` returns `LayoutAreas` (header, tabs, content, panels, optional companion bar, status). Modal centering is handled inside `render/modals/` per modal, sized to the contents rather than a generic `centered_rect` helper.

## Summary

**Components are stateful widgets:**

- Own display state (scroll, selection)
- Provide pure render functions
- Reusable across modes
- No business logic, no side effects

**Key patterns:**

- State + Methods + Render
- Builder pattern for construction
- Event emission via return values
- Theme-consistent styling

**When creating components:**

- Keep state minimal (only display concerns)
- Make render pure (no mutations)
- Test state updates independently
- Use theme functions for consistency
