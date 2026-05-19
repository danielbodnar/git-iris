# Semantic Token Reference

Complete reference of the semantic tokens git-iris reads from the active theme. The 26 tokens listed under "opaline's 26-token contract" are what every well-rounded theme defines; git-iris also registers a handful of additional tokens at runtime (`git.*`, `diff.*`, `mode.*`, `code.hash`, `code.path`) with sensible defaults derived from the contract tokens — those are optional overrides, not required inputs.

opaline performs no required-token validation. A token that isn't defined silently resolves to `OpalineColor::FALLBACK` (a neutral gray, `#808080`). Themes never fail to load over missing tokens — they only fail on malformed TOML, bad hex colors, unresolved or circular references, or empty gradients.

## Token Naming Convention

Tokens use dot notation to create hierarchical namespaces:

```toml
"text.primary"       # Namespace: text, Property: primary
"bg.highlight"       # Namespace: bg, Property: highlight
"git.staged"         # Namespace: git, Property: staged
```

This structure makes it easy to understand token purpose and modify related colors together.

## Text Hierarchy

Controls text rendering throughout the UI.

| Token            | Usage                                  | Example                      |
| ---------------- | -------------------------------------- | ---------------------------- |
| `text.primary`   | Primary text, highest contrast         | File names, headings         |
| `text.secondary` | Secondary text, medium contrast        | Descriptions, metadata       |
| `text.muted`     | Tertiary text, lower contrast          | Labels, hints                |
| `text.dim`       | Lowest priority text, minimal contrast | Disabled items, placeholders |

**Example:**

```toml
[tokens]
"text.primary" = "#f8f8f2"
"text.secondary" = "#bcbcca"
"text.muted" = "#82879f"
"text.dim" = "#6e7daf"
```

**Visual hierarchy:**

```
Primary:   ████████ 100% contrast
Secondary: ██████░░  75% contrast
Muted:     ████░░░░  50% contrast
Dim:       ██░░░░░░  25% contrast
```

## Background Surfaces

Defines background layers and elevation.

| Token          | Contract | Usage                       | Example                    |
| -------------- | -------- | --------------------------- | -------------------------- |
| `bg.base`      | ✓        | Main application background | Canvas, root window        |
| `bg.panel`     | ✓        | Panel/section backgrounds   | Sidebar, main content area |
| `bg.code`      | ✓        | Code block backgrounds      | Diff view, file contents   |
| `bg.highlight` | ✓        | Highlighted/hovered items   | Cursor line, row hover     |
| `bg.selection` | ✓        | Selection background        | Active selection, focused row |
| `bg.elevated`  | extra    | Elevated surfaces (modals, tooltips) | Used by SilkCircuit builtins |
| `bg.active`    | extra    | Active/selected state       | Used by SilkCircuit builtins |

The five tokens marked **✓** are part of opaline's standard 26-token contract. `bg.elevated` and `bg.active` are extras the SilkCircuit builtins define — they're not part of the standard contract, so other themes may omit them and Studio still renders correctly.

**Example:**

```toml
[tokens]
"bg.base" = "#121218"        # Darkest
"bg.panel" = "#181820"       # Slightly lighter
"bg.code" = "#1e1e28"        # Code context
"bg.highlight" = "#37324b"   # Hover state
"bg.selection" = "#3c3c50"   # Selection
# SilkCircuit-specific extras
"bg.elevated" = "#37324b"    # Floating elements
"bg.active" = "#3c2d55"      # Active selection
```

**Elevation model:**

```
Base      Panel     Code      Highlight Elevated  Active    Selection
████░░    █████░    ██████    ███████░  ███████░  ████████  █████████
```

## Accent Colors

Brand colors for emphasis and interaction.

| Token              | Usage                               | Example                |
| ------------------ | ----------------------------------- | ---------------------- |
| `accent.primary`   | Primary brand color, main actions   | Active mode, keywords  |
| `accent.secondary` | Secondary brand color, interactions | Links, hover states    |
| `accent.tertiary`  | Tertiary brand color, decorative    | Icons, badges          |
| `accent.deep`      | Deeper variant of primary           | Shadows, depth effects |

**Example:**

```toml
[tokens]
"accent.primary" = "#e135ff"     # Electric Purple
"accent.secondary" = "#80ffea"   # Neon Cyan
"accent.tertiary" = "#ff6ac1"    # Coral
"accent.deep" = "#bd93f9"        # Deep Purple
```

## Semantic Status Colors

Universal status indicators.

| Token     | Usage                            | Example                        |
| --------- | -------------------------------- | ------------------------------ |
| `success` | Positive states, confirmations   | Staged files, success messages |
| `error`   | Negative states, errors          | Deleted files, error messages  |
| `warning` | Caution states, attention needed | Modified files, warnings       |
| `info`    | Informational, neutral           | Hints, info messages           |

**Example:**

```toml
[tokens]
success = "#50fa7b"   # Green
error = "#ff6363"     # Red
warning = "#f1fa8c"   # Yellow
info = "#80ffea"      # Cyan
```

## Git Status Colors

Git file state indicators. These are git-iris extras with derived defaults — define them in your TOML only if you want to override the defaults shown.

| Token           | Default fallback | Usage                            | Git Status     |
| --------------- | ---------------- | -------------------------------- | -------------- |
| `git.staged`    | `success`        | Staged changes (ready to commit) | `A ` added     |
| `git.modified`  | `warning`        | Modified but unstaged            | ` M` modified  |
| `git.untracked` | `text.muted`     | Untracked files                  | `??` untracked |
| `git.deleted`   | `error`          | Deleted files                    | ` D` deleted   |

**Example:**

```toml
[tokens]
"git.staged" = "#50fa7b"     # Green (ready)
"git.modified" = "#f1fa8c"   # Yellow (changed)
"git.untracked" = "#6e7daf"  # Gray (new)
"git.deleted" = "#ff6363"    # Red (removed)
```

**File tree rendering:**

```
src/
  main.rs         (staged)    █ Green
  config.rs       (modified)  █ Yellow
  temp.txt        (untracked) █ Gray
  old_code.rs     (deleted)   █ Red
```

## Diff Colors

Unified diff view syntax highlighting. Like the git status tokens, these are git-iris extras with derived defaults.

| Token          | Default fallback | Usage                      | Diff Line Prefix |
| -------------- | ---------------- | -------------------------- | ---------------- |
| `diff.added`   | `success`        | Added lines                | `+`              |
| `diff.removed` | `error`          | Removed lines              | `-`              |
| `diff.hunk`    | `info`           | Hunk headers (`@@ ... @@`) | `@@`             |
| `diff.context` | `text.dim`       | Unchanged context lines    | ` ` (space)      |

**Example:**

```toml
[tokens]
"diff.added" = "#50fa7b"     # Green
"diff.removed" = "#ff6363"   # Red
"diff.hunk" = "#80ffea"      # Cyan
"diff.context" = "#6e7daf"   # Gray
```

**Diff rendering:**

```diff
@@ -12,6 +12,8 @@                    (diff.hunk)
 fn main() {                          (diff.context)
-    println!("old");                  (diff.removed)
+    println!("new");                  (diff.added)
+    println!("another");              (diff.added)
 }                                     (diff.context)
```

## UI Elements

Interface components and interactions.

| Token              | Usage                  | Example                |
| ------------------ | ---------------------- | ---------------------- |
| `border.focused`   | Focused panel border   | Active panel outline   |
| `border.unfocused` | Unfocused panel border | Inactive panel outline |

**Example:**

```toml
[tokens]
"border.focused" = "#80ffea"    # Bright cyan (attention)
"border.unfocused" = "#82879f"  # Gray (subtle)
```

**Border states:**

```
┌─ Focused Panel ──────┐   ┌─ Unfocused Panel ───┐
│ (border.focused)     │   │ (border.unfocused)  │
│ Bright, attention-   │   │ Subtle, recedes to  │
│ grabbing             │   │ background          │
└──────────────────────┘   └─────────────────────┘
```

## Code Syntax

Syntax highlighting tokens for source code. The seven `code.keyword` / `code.function` / `code.string` / `code.number` / `code.comment` / `code.type` / `code.line_number` tokens are part of opaline's contract. `code.hash` and `code.path` are git-iris extras with derived defaults (commit hashes default to `accent.tertiary`, file paths to `accent.secondary`).

| Token              | Contract | Usage                     | Example                    |
| ------------------ | -------- | ------------------------- | -------------------------- |
| `code.keyword`     | ✓        | Programming keywords      | `fn`, `let`, `if`          |
| `code.function`    | ✓        | Function/method names     | `calculate()`, `get_value` |
| `code.string`      | ✓        | String literals           | `"hello"`, `'world'`       |
| `code.number`      | ✓        | Numeric literals          | `42`, `3.14`, `0xFF`       |
| `code.comment`     | ✓        | Code comments             | `// comment`, `/* ... */`  |
| `code.type`        | ✓        | Type names, classes       | `String`, `Option<T>`      |
| `code.line_number` | ✓        | Line numbers in code view | `1`, `2`, `3`              |
| `code.hash`        | extra    | Commit hashes, checksums  | `a3f2c9b`                  |
| `code.path`        | extra    | File paths, URLs          | `src/main.rs`              |

**Example:**

```toml
[tokens]
"code.hash" = "#ff6ac1"      # Coral
"code.path" = "#80ffea"      # Cyan
"code.keyword" = "#e135ff"   # Purple
"code.function" = "#80ffea"  # Cyan
"code.string" = "#ff99ff"    # Pink
"code.number" = "#ff6ac1"    # Coral
"code.comment" = "#6e7daf"   # Gray
"code.type" = "#f1fa8c"      # Yellow
"code.line_number" = "#6e7daf" # Gray
```

**Syntax highlighting:**

```rust
1  fn calculate(x: i32) -> i32 {  // Calculate result
   ██ ████████ ███ ███   ███         ████████████████
   │  │        │   │     │           └─ code.comment
   │  │        │   │     └─ code.keyword (return type)
   │  │        │   └─ code.type
   │  │        └─ code.type
   │  └─ code.function
   └─ code.line_number

2      let value = "test";
       ███ █████   ██████
       │   │       └─ code.string
       │   └─ identifier
       └─ code.keyword
```

## Mode Tabs

Navigation tab states. These are git-iris extras with derived defaults.

| Token           | Default fallback   | Usage                             | Example          |
| --------------- | ------------------ | --------------------------------- | ---------------- |
| `mode.active`   | `accent.primary`   | Currently active mode             | Selected tab     |
| `mode.inactive` | `text.muted`       | Inactive modes                    | Unselected tabs  |
| `mode.hover`    | `accent.secondary` | Hovered mode (future enhancement) | Tab under cursor |

**Example:**

```toml
[tokens]
"mode.active" = "#e135ff"    # Purple (bold)
"mode.inactive" = "#6e7daf"  # Gray (dim)
"mode.hover" = "#80ffea"     # Cyan (highlight)
```

**Tab bar:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 EXPLORE   COMMIT   REVIEW   PR   CHAT
 ███████   ██████   ██████   ██   ████
 (active)  (inactive)...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Chat

Conversational UI colors. These are theme-defined extras — Studio renders chat in sensible accent colors even if a theme omits them.

| Token       | Usage               | Example                |
| ----------- | ------------------- | ---------------------- |
| `chat.user` | User messages       | Your questions to Iris |
| `chat.iris` | Iris agent messages | Iris responses         |

**Example:**

```toml
[tokens]
"chat.user" = "#80ffea"    # Cyan (you)
"chat.iris" = "#e135ff"    # Purple (AI)
```

**Chat rendering:**

```
You:  What changed in this commit?
████  (chat.user)

Iris: This commit adds theme support...
████  (chat.iris)
```

## Load-Time Errors and Fallbacks

opaline does **not** validate that any particular token is defined. A theme file with zero tokens loads without error; missing tokens silently resolve to `OpalineColor::FALLBACK` (`#808080`) and missing styles return `OpalineStyle::default()`.

The errors you can hit at load time are:

```
TOML parse error in <path>: <details>           # malformed TOML, unknown keys
invalid color for token '<name>': <details>     # bad hex literal (`#rrggbb` only)
unresolved token '<name>' references '<ref>'    # palette/token name not found
circular token reference '<name>': a → b → a    # cycle in token-to-token chain
gradient must have at least one color stop      # empty gradient array
```

That's the entire error surface. If your theme parses cleanly, it loads — gaps just appear as gray.

## Complete Token Checklist

opaline's **26-token contract** is what every well-rounded theme should cover. Git-iris layers **9 optional overrides** on top with sensible defaults derived from the contract.

### opaline contract (26 tokens)

**Text hierarchy (4)**

- [ ] `text.primary`
- [ ] `text.secondary`
- [ ] `text.muted`
- [ ] `text.dim`

**Backgrounds (5)**

- [ ] `bg.base`
- [ ] `bg.panel`
- [ ] `bg.code`
- [ ] `bg.highlight`
- [ ] `bg.selection`

**Accents (4)**

- [ ] `accent.primary`
- [ ] `accent.secondary`
- [ ] `accent.tertiary`
- [ ] `accent.deep`

**Semantic status (4)**

- [ ] `success`
- [ ] `error`
- [ ] `warning`
- [ ] `info`

**Borders (2)**

- [ ] `border.focused`
- [ ] `border.unfocused`

**Code syntax (7)**

- [ ] `code.keyword`
- [ ] `code.function`
- [ ] `code.string`
- [ ] `code.number`
- [ ] `code.comment`
- [ ] `code.type`
- [ ] `code.line_number`

### Optional overrides (derived defaults)

Git-iris registers these as derived defaults at theme load — you can override any of them in your TOML, or omit them and they'll inherit from the contract tokens shown in parentheses.

**Git status** — derive from semantic colors

- `git.staged` (defaults to `success`)
- `git.modified` (defaults to `warning`)
- `git.untracked` (defaults to `text.muted`)
- `git.deleted` (defaults to `error`)

**Diff** — derive from semantic colors

- `diff.added` (defaults to `success`)
- `diff.removed` (defaults to `error`)
- `diff.hunk` (defaults to `info`)
- `diff.context` (defaults to `text.dim`)

**Mode tabs** — derive from accents

- `mode.active` (defaults to `accent.primary`)
- `mode.inactive` (defaults to `text.muted`)
- `mode.hover` (defaults to `accent.secondary`)

**Code extras** — derive from accents

- `code.hash` (defaults to `accent.tertiary`)
- `code.path` (defaults to `accent.secondary`)

Other tokens you'll see in some themes (`bg.elevated`, `bg.active`, `chat.user`, `chat.iris`) are extras some builtins define but aren't part of opaline's standard contract.

## Token Evolution

The token contract is governed by the opaline crate version, not by git-iris itself.

| opaline version | Contract size  | Notes                                       |
| --------------- | -------------- | ------------------------------------------- |
| 0.4.x           | 26 tokens      | Current version; see `opaline::names::tokens` |

Check opaline's release notes when bumping versions to spot any contract changes. Existing themes keep working through gaps — the only risk is unexpected gray fallbacks when a theme doesn't define a newly added token.

## Usage in Code

Tokens are accessed through the active theme. Conversion to ratatui types uses the `From`/`Into` adapter — there's no `to_ratatui()` method.

```rust
use git_iris::theme;
use opaline::names::tokens;
use ratatui::style::{Color, Style};

// Get current theme
let theme = theme::current();

// Look up a color by token name
let color: Color = theme.color(tokens::ACCENT_PRIMARY).into();

// Look up a style and convert to a ratatui Style
let style: Style = theme.style("keyword").into();
```

In practice git-iris wraps these accessors in `src/studio/theme.rs` so call sites read like `theme::keyword()` and get a `ratatui::style::Style` directly.

## Token Naming Philosophy

Token names follow these principles:

1. **Semantic over visual** — `accent.primary` not `purple`
2. **Hierarchical** — Use dots for namespacing
3. **Consistent** — Same pattern across categories
4. **Self-documenting** — Name reveals purpose

**Good token names:**

- `text.primary` — Clear hierarchy and purpose
- `git.staged` — Obvious semantic meaning
- `diff.hunk` — Specific, unambiguous

**Poor token names:**

- `color1` — No semantic meaning
- `purple_text` — Too specific, not flexible
- `important` — Vague, subjective

---

**Next Steps:**

- [Learn about styles](./styles.md)
- [Create your own theme](./creating.md)
- [View theme gallery](./gallery.md)
