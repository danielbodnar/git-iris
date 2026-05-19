# Creating Custom Themes

This guide walks you through creating custom themes for Git-Iris, from basic color changes to advanced gradient definitions.

## Quick Start

### 1. Create Your Theme File

Git-Iris uses the [opaline](https://crates.io/crates/opaline) theme engine and scans two user directories for themes:

- `~/.config/opaline/themes/` — shared with any other opaline-powered app
- `~/.config/git-iris/themes/` — git-iris-specific

When the same theme id appears in both directories, the **later directory wins** (git-iris-specific overrides the shared opaline directory). File-backed themes always override builtins of the same id.

```bash
mkdir -p ~/.config/git-iris/themes
touch ~/.config/git-iris/themes/my-theme.toml
```

### 2. Define Metadata

Every theme starts with metadata:

```toml
[meta]
name = "My Custom Theme"
author = "Your Name"
variant = "dark"  # or "light"
version = "1.0"
description = "A brief description of your theme"
```

**Fields:**

- `name` — Display name shown in the theme selector (effectively required — opaline parses without it but the selector will show an empty string)
- `author`, `version`, `description` — Optional metadata
- `variant` — `dark` (default) or `light`

::: warning Unknown keys are rejected
opaline parses theme TOML with `#[serde(deny_unknown_fields)]` on `[meta]`, on each `[styles.*]` entry, and at the top level. A misspelled key or an unrecognized property anywhere in your file causes a hard parse error at load time. Stick to the documented fields.
:::

### 3. Define Your Palette

The palette contains raw color primitives referenced by tokens:

```toml
[palette]
# Brand colors
primary = "#ff00ff"
secondary = "#00ffff"
accent = "#ff6ac1"

# Backgrounds
bg_dark = "#1a1a2e"
bg_light = "#25254a"

# Text colors
text_bright = "#ffffff"
text_dim = "#666699"
```

**Color formats:**

- Hex RGB: `"#ff00ff"` (must be the full 7-character `#rrggbb` form — opaline does not accept 3-digit shorthand)
- Lowercase recommended for consistency

**Naming conventions:**

- Use semantic names: `purple_500`, `cyan_400`
- Indicate intensity: `bg_dark`, `bg_light`
- Follow numeric scales: `gray_50` to `gray_950`

### 4. Define Semantic Tokens

Tokens map palette colors to semantic meanings:

```toml
[tokens]
# Text hierarchy
"text.primary" = "text_bright"
"text.secondary" = "text_dim"

# Backgrounds
"bg.base" = "bg_dark"
"bg.panel" = "bg_light"

# Accents
"accent.primary" = "primary"
"accent.secondary" = "secondary"

# Git status
"git.staged" = "#50fa7b"
"git.modified" = "#f1fa8c"
```

**Token value formats:**

- Palette references: `"primary"` → looks up `[palette]` key
- Token-to-token references: `"accent.primary"` → resolves through another token (cycles are detected and reported)
- Direct hex colors: `"#ff00ff"` → inline color definition

You only have to define the tokens you care about. opaline performs no required-token validation: any token a UI element asks for that isn't defined falls back silently to `OpalineColor::FALLBACK` (a neutral gray). Missing styles return `OpalineStyle::default()`. See the [Token Reference](./tokens.md) for the full 26-token contract and the optional git-iris extras.

### 5. Add Your Theme

Once saved, your theme is automatically available:

```bash
# List themes (yours will appear alongside builtins)
git-iris themes

# Preview in Studio
git-iris studio --theme my-theme
```

To set a theme persistently, edit `~/.config/git-iris/config.toml`:

```toml
theme = "my-theme"
```

Leaving `theme = ""` keeps the default (`silkcircuit-neon`). You can override the choice per-project by placing the same field in a `.irisconfig` file at the repo root.

## Complete Theme Template

Here's a complete theme template that covers opaline's 26-token contract and the optional git-iris overrides. You can omit any token — missing tokens fall back to gray, but covering the full set produces a polished UI everywhere.

```toml
[meta]
name = "My Theme"
author = "Your Name"
variant = "dark"

# ═══════════════════════════════════════════════════════════════════════════════
# Palette — Raw color primitives
# ═══════════════════════════════════════════════════════════════════════════════

[palette]
# Core colors
purple = "#a855f7"
cyan = "#06b6d4"
pink = "#ec4899"
green = "#10b981"
red = "#ef4444"
yellow = "#f59e0b"

# Backgrounds
bg_base = "#0f172a"
bg_panel = "#1e293b"
bg_code = "#1e293b"
bg_highlight = "#334155"

# Text
text_primary = "#f8fafc"
text_secondary = "#cbd5e1"
text_muted = "#94a3b8"

# ═══════════════════════════════════════════════════════════════════════════════
# Tokens — opaline's 26-token contract
# ═══════════════════════════════════════════════════════════════════════════════

[tokens]
# Text hierarchy
"text.primary" = "text_primary"
"text.secondary" = "text_secondary"
"text.muted" = "text_muted"
"text.dim" = "text_muted"

# Backgrounds
"bg.base" = "bg_base"
"bg.panel" = "bg_panel"
"bg.code" = "bg_code"
"bg.highlight" = "bg_highlight"
"bg.selection" = "bg_highlight"

# Accent colors
"accent.primary" = "purple"
"accent.secondary" = "cyan"
"accent.tertiary" = "pink"
"accent.deep" = "purple"

# Semantic status
success = "green"
error = "red"
warning = "yellow"
info = "cyan"

# Borders
"border.focused" = "cyan"
"border.unfocused" = "text_muted"

# Code syntax
"code.keyword" = "purple"
"code.function" = "cyan"
"code.string" = "green"
"code.number" = "pink"
"code.comment" = "text_muted"
"code.type" = "yellow"
"code.line_number" = "text_muted"

# ─────────────────────────────────────────────────────────────────────────────
# Optional git-iris overrides — derived from the contract above if omitted
# ─────────────────────────────────────────────────────────────────────────────

# Git status (default: success / warning / text.muted / error)
"git.staged" = "green"
"git.modified" = "yellow"
"git.untracked" = "text_muted"
"git.deleted" = "red"

# Diff colors (default: success / error / info / text.dim)
"diff.added" = "green"
"diff.removed" = "red"
"diff.hunk" = "cyan"
"diff.context" = "text_muted"

# Mode tabs (default: accent.primary / text.muted / accent.secondary)
"mode.active" = "purple"
"mode.inactive" = "text_muted"
"mode.hover" = "cyan"

# Commit hashes and file paths in CLI output
# (default: accent.tertiary / accent.secondary)
"code.hash" = "pink"
"code.path" = "cyan"

# ═══════════════════════════════════════════════════════════════════════════════
# Styles — Composed styles with modifiers (optional)
# ═══════════════════════════════════════════════════════════════════════════════

[styles]
keyword = { fg = "accent.primary", bold = true }
file_path = { fg = "code.path" }
selected = { fg = "accent.secondary", bg = "bg.highlight" }

# ═══════════════════════════════════════════════════════════════════════════════
# Gradients — Color transitions (optional)
# ═══════════════════════════════════════════════════════════════════════════════

[gradients]
primary = ["purple", "cyan"]
warm = ["pink", "yellow"]
```

## Advanced Techniques

### Token Chaining

Tokens can reference other tokens for consistency:

```toml
[palette]
purple_500 = "#e135ff"

[tokens]
"accent.primary" = "purple_500"
"mode.active" = "accent.primary"      # References accent.primary
"border.focused" = "mode.active"      # References mode.active
```

All three tokens resolve to `#e135ff`, but you can change the entire chain by updating `purple_500`.

### Custom Styles

Define composed styles with foreground, background, and modifiers:

```toml
[styles]
# Bold keyword
keyword = { fg = "accent.primary", bold = true }

# Highlighted selection
selected = { fg = "accent.secondary", bg = "bg.highlight" }

# Dimmed text
muted = { fg = "text.muted", dim = true }

# Italic comments
comment = { fg = "code.comment", italic = true }

# Underlined links
link = { fg = "accent.secondary", underline = true }

# Complex combination
error_highlight = { fg = "error", bg = "bg.highlight", bold = true }
```

**Available modifiers:**

opaline supports all nine ratatui text modifiers as boolean style properties:

- `bold` — Bold text
- `italic` — Italic text
- `underline` — Underlined text
- `dim` — Dimmed/faint text
- `slow_blink` — Slow blink (terminal-dependent)
- `rapid_blink` — Rapid blink (terminal-dependent, often disabled)
- `reversed` — Swap foreground and background
- `hidden` — Hidden / invisible text (still occupies space)
- `crossed_out` — Strikethrough

Whether a modifier actually renders depends on your terminal emulator. Bold, italic, underline, and dim are widely supported; the blinks and crossed-out are not.

### Multi-Stop Gradients

Create smooth color transitions with multiple stops:

```toml
[gradients]
# Two-stop gradient (simple)
primary = ["purple_500", "cyan_400"]

# Three-stop gradient (middle accent)
warm = ["coral_400", "yellow_400", "green_400"]

# Five-stop gradient (complex)
rainbow = ["#ff0000", "#ff7f00", "#ffff00", "#00ff00", "#0000ff"]

# Aurora gradient (signature SilkCircuit sweep)
aurora = ["purple_500", "#f31bff", "#ff00ff", "#bf80f4", "cyan_400"]
```

Gradients interpolate smoothly between stops. Access with:

```rust
// In Rust code
let color = theme.gradient("aurora", 0.5);  // Midpoint color

// Generate N evenly-spaced colors
let colors = theme.get_gradient("aurora").unwrap().generate(10);
```

### Light Theme Considerations

When creating light themes:

```toml
[meta]
variant = "light"

[palette]
# Darker accent colors for contrast
purple = "#7e2bd5"
cyan = "#007f8e"

# Light backgrounds
bg_base = "#faf8ff"
bg_panel = "#f1ecff"
bg_code = "#efeaff"

# Dark text
text_primary = "#2b2540"
text_secondary = "#3d3558"
text_muted = "#5a4d78"
```

**Tips:**

- Use darker, more saturated accent colors
- Ensure sufficient contrast (WCAG AA: 4.5:1 minimum)
- Test with syntax highlighting
- Consider ambient light conditions

## Best Practices

### Color Psychology

Choose colors that convey the right meaning:

| Semantic | Traditional Color | Reason                       |
| -------- | ----------------- | ---------------------------- |
| Success  | Green             | Universal positive indicator |
| Error    | Red               | Universal danger/stop signal |
| Warning  | Yellow/Orange     | Caution without severity     |
| Info     | Cyan/Blue         | Neutral, informative         |
| Modified | Yellow            | Changed, needs attention     |
| Staged   | Green             | Ready for commit (positive)  |
| Deleted  | Red               | Removal (negative)           |

### Accessibility

Ensure your theme is accessible:

1. **Contrast ratios**
   - Normal text: 4.5:1 minimum (WCAG AA)
   - Large text: 3:1 minimum
   - Use tools like [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)

2. **Color blindness**
   - Don't rely solely on color to convey information
   - Test with simulators (Coblis, ColorOracle)
   - Consider red-green color blindness (most common)

3. **Brightness**
   - Avoid pure black (`#000000`) backgrounds
   - Avoid pure white (`#ffffff`) on dark themes
   - Use slightly off-black/off-white for reduced eye strain

### Consistency

Maintain visual hierarchy:

```toml
# Text hierarchy (decreasing contrast)
"text.primary" = "gray_50"    # Highest contrast
"text.secondary" = "gray_200"  # Medium contrast
"text.muted" = "gray_400"      # Low contrast
"text.dim" = "gray_500"        # Lowest contrast
```

### Performance

- The 39 opaline builtin themes are `include_str!`'d at compile time and load with zero I/O
- User themes in `~/.config/opaline/themes/` and `~/.config/git-iris/themes/` are read from disk at runtime, but only once per theme switch
- Resolution runs once per load; access via `theme::current().color()` and `.style()` is a HashMap lookup
- Gradients interpolate on-demand; pre-generate with `Gradient::generate(n)` if you call inside tight loops

## Testing Your Theme

### Visual Testing

1. **Preview in Studio**

   ```bash
   git-iris studio --theme my-theme
   ```

2. **Cycle through modes**
   - Explore mode: Test file tree, code view
   - Commit mode: Test diff colors, git status
   - Review mode: Test syntax highlighting
   - PR mode: Test markdown rendering

3. **Test edge cases**
   - Long commit messages
   - Large diffs
   - Empty states
   - Error messages

### Load-Time Errors

opaline doesn't enforce a required-token list — a theme with zero tokens loads without complaint and just renders everything in the fallback gray. The errors you will see come from genuinely malformed TOML:

```
TOML parse error in /path/to/theme.toml: ...
invalid color for token 'accent.primary': invalid hex color length 4 (expected 7, e.g. #rrggbb)
unresolved token 'accent.primary' references 'nonexistent_color'
circular token reference 'a': a → b → a
gradient must have at least one color stop
```

**Common issues:**

- Unknown keys (caught by `#[serde(deny_unknown_fields)]`)
- Malformed hex colors (must be `#rrggbb`, exactly 7 chars)
- References to palette names or tokens that don't exist
- Cycles in token-to-token references
- Empty gradient arrays

### Iterative Refinement

1. **Start simple** — Copy a builtin theme and modify colors
2. **Test frequently** — Preview after each major change
3. **Compare themes** — Switch between yours and builtins
4. **Get feedback** — Share with others for fresh perspectives
5. **Refine gradually** — Small tweaks compound over time

## Sharing Your Theme

### Export Your Theme

```bash
# Your theme is already in a shareable location
cat ~/.config/git-iris/themes/my-theme.toml
```

### Contribute to opaline

Git-Iris doesn't carry its own theme builtin directory — the 39 builtin themes live in the [opaline](https://crates.io/crates/opaline) crate and are discovered automatically from `opaline-<version>/src/builtins/*.toml` at compile time. To get your theme into the builtin set:

1. Open a pull request against the opaline repository
2. Place the `.toml` file in `src/builtins/`
3. opaline's build script auto-discovers the file — no manifest edit needed
4. Include screenshots demonstrating the theme in real apps

If you just want to ship a theme for git-iris users without going through opaline, distribute it as a `.toml` file users can drop into `~/.config/git-iris/themes/`.

### Community Themes

Share your themes:

- GitHub Gists
- Git-Iris discussions
- Reddit r/unixporn
- Terminal theme repositories

## Examples

### Monochrome Theme

```toml
[meta]
name = "Grayscale"
variant = "dark"

[palette]
gray_50 = "#f5f5f5"
gray_400 = "#9ca3af"
gray_700 = "#374151"
gray_900 = "#111827"

[tokens]
"text.primary" = "gray_50"
"text.muted" = "gray_400"
"bg.base" = "gray_900"
"bg.panel" = "gray_700"
"accent.primary" = "gray_50"
"accent.secondary" = "gray_400"
# ... (all other tokens use grayscale)
```

### High Contrast Theme

```toml
[meta]
name = "Maximum Contrast"
variant = "dark"

[palette]
white = "#ffffff"
black = "#000000"
pure_cyan = "#00ffff"
pure_magenta = "#ff00ff"

[tokens]
"text.primary" = "white"
"bg.base" = "black"
"accent.primary" = "pure_magenta"
"accent.secondary" = "pure_cyan"
# ... (pure colors only)
```

### Pastel Theme

```toml
[meta]
name = "Soft Pastels"
variant = "light"

[palette]
pastel_purple = "#dcc9ff"
pastel_pink = "#ffd9e8"
pastel_blue = "#c9f0ff"
pastel_green = "#d4f4dd"
cream = "#fffef9"

[tokens]
"bg.base" = "cream"
"accent.primary" = "pastel_purple"
"accent.secondary" = "pastel_blue"
# ... (soft, muted colors)
```

## Troubleshooting

### Theme Not Appearing

```bash
# Check theme file location
ls ~/.config/git-iris/themes/

# Verify file has .toml extension
mv my-theme.txt my-theme.toml

# Check for TOML syntax errors
git-iris studio --theme my-theme
```

### Colors Look Wrong

- Verify terminal true color support: `echo $COLORTERM` should be `truecolor`
- Check terminal emulator settings
- Test in different terminals (iTerm2, Alacritty, WezTerm)
- Verify hex colors are valid RGB

### Tokens Showing as Gray

A token that isn't defined resolves to `OpalineColor::FALLBACK` — a neutral gray. If you see unexpected gray patches, check the [Token Reference](./tokens.md) and add the missing tokens. opaline never errors on a missing token, so this is a silent issue you have to spot visually.

---

**Next Steps:**

- [Explore semantic tokens](./tokens.md)
- [Learn about styles and gradients](./styles.md)
- [View theme gallery](./gallery.md)
