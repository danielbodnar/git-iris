# SilkCircuit Design Language

**Electric meets elegant.**

Git-Iris ships with the SilkCircuit design language as its default look, layered on top of the [opaline](https://crates.io/crates/opaline) theme engine. Every color, gradient, and style is defined in TOML, so you can swap in any opaline-compatible theme or author your own without touching Rust.

## Philosophy

SilkCircuit was designed around three core principles:

1. **Electric Energy** — Bold, saturated colors that command attention
2. **Elegant Structure** — Semantic tokens that maintain visual hierarchy
3. **Easy Customization** — Simple TOML configuration, no code required

The result is a visual language that feels futuristic and refined, built for developers who spend hours in the terminal.

## Core Color Palette

The original SilkCircuit Neon theme defines six core brand colors:

| Color           | Hex       | RGB               | Usage                          |
| --------------- | --------- | ----------------- | ------------------------------ |
| Electric Purple | `#e135ff` | `(225, 53, 255)`  | Primary accents, active modes  |
| Neon Cyan       | `#80ffea` | `(128, 255, 234)` | Paths, interactions, focus     |
| Coral           | `#ff6ac1` | `(255, 106, 193)` | Hashes, numbers, constants     |
| Electric Yellow | `#f1fa8c` | `(241, 250, 140)` | Warnings, timestamps           |
| Success Green   | `#50fa7b` | `(80, 250, 123)`  | Success states, staged changes |
| Error Red       | `#ff6363` | `(255, 99, 99)`   | Errors, danger, deleted files  |

## Theme Catalog

Git-Iris exposes **39 builtin themes** that ship inside opaline 0.4 — the five SilkCircuit variants plus a curated set of community classics (Dracula, Nord, Tokyo Night, Catppuccin, Gruvbox, Ayu, Rose Pine, Kanagawa, Everforest, Flexoki, GitHub, Monokai Pro, One Dark, One Light, Palenight, Solarized, Night Owl, Light Owl).

The five SilkCircuit variants:

| Theme                   | Variant | Description                               |
| ----------------------- | ------- | ----------------------------------------- |
| **SilkCircuit Neon**    | Dark    | Electric purple and neon cyan (default)   |
| **SilkCircuit Soft**    | Dark    | Muted elegance with desaturated colors    |
| **SilkCircuit Glow**    | Dark    | Maximum neon intensity against pure black |
| **SilkCircuit Vibrant** | Dark    | High saturation with rich purple tones    |
| **SilkCircuit Dawn**    | Light   | Purple accents on soft lavender-white     |

Run `git-iris themes` to list every builtin alongside any user themes you have installed. The Gallery has visual previews and a tour of the non-SilkCircuit families.

## Token-Based Architecture

Unlike traditional themes that hardcode colors, SilkCircuit uses a **semantic token system**:

```toml
[palette]
purple_500 = "#e135ff"      # Raw color primitive

[tokens]
"accent.primary" = "purple_500"  # Semantic token → palette
"mode.active" = "accent.primary" # Token → token reference

[styles]
mode_active = { fg = "mode.active", bold = true }  # Composed style
```

This three-layer architecture enables:

- **Consistency** — Change one palette color, update everywhere
- **Flexibility** — Override specific tokens without rebuilding
- **Clarity** — Semantic names reveal intent (`accent.primary` vs `#e135ff`)

## Quick Start

### Switching Themes

```bash
# List available themes
git-iris themes

# Use a different theme for this session
git-iris studio --theme silkcircuit-soft

# Set persistently in config.toml: theme = "silkcircuit-glow"
```

### Creating Custom Themes

Create `~/.config/git-iris/themes/my-theme.toml`:

```toml
[meta]
name = "My Custom Theme"
author = "Your Name"
variant = "dark"

[palette]
primary = "#ff00ff"
secondary = "#00ffff"

[tokens]
"accent.primary" = "primary"
"accent.secondary" = "secondary"
```

See [Creating Custom Themes](./creating.md) for a complete guide.

## Architecture Overview

```mermaid
flowchart TB
    subgraph Theme["Theme System"]
        palette[Palette]
        tokens[Tokens]
        styles[Styles]
        gradients[Gradients]

        palette --> tokens
        tokens --> styles
        tokens --> gradients
    end
```

| Layer         | Purpose                        | Example                                            |
| ------------- | ------------------------------ | -------------------------------------------------- |
| **Palette**   | Raw color primitives           | `purple_500 = "#e135ff"`                           |
| **Tokens**    | Semantic color assignments     | `accent.primary = "purple_500"`                    |
| **Styles**    | Composed styles with modifiers | `keyword = { fg = "accent.primary", bold = true }` |
| **Gradients** | Multi-stop color transitions   | `primary = ["purple_500", "cyan_400"]`             |

## Documentation Structure

- **[Theme Gallery](./gallery.md)** — Visual showcase of all builtin themes
- **[Creating Custom Themes](./creating.md)** — Step-by-step theme creation guide
- **[Semantic Token Reference](./tokens.md)** — Complete token listing
- **[Styles & Gradients](./styles.md)** — Advanced styling techniques

## Design Inspiration

SilkCircuit draws inspiration from:

- **Synthwave aesthetics** — Neon colors and retro-future vibes
- **Cyberpunk UI** — High contrast, electric gradients
- **Modern terminal themes** — Dracula, Tokyo Night, Nord
- **Material Design** — Semantic color systems and elevation

The result is a unique visual identity that stands out in the crowded landscape of terminal themes while remaining functional and readable for extended coding sessions.

## Technical Implementation

The theme system is built on the [opaline](https://crates.io/crates/opaline) crate (v0.4):

- **TOML configuration** — Simple, human-readable format
- **26 named semantic tokens** — opaline's standard contract for text, backgrounds, accents, status colors, borders, and code syntax
- **Color interpolation** — Smooth multi-stop gradients
- **Runtime switching** — Change themes without restarting
- **Graceful fallbacks** — Missing tokens resolve to `OpalineColor::FALLBACK` (a neutral gray); missing styles return `OpalineStyle::default()`. There is no separate "validation" step

Git-Iris layers a handful of extra runtime-registered tokens on top of opaline's contract (`git.*`, `diff.*`, `mode.*`, `code.hash`, `code.path`) so file-tree, diff, and tab styling have semantic names too. These are derived from opaline's standard tokens by default but theme TOMLs can override them.

Theme files load from `~/.config/opaline/themes/` and `~/.config/git-iris/themes/`. The only load-time errors come from TOML parsing failures, invalid hex colors, unresolved or circular token references, and empty gradients.

---

**Ready to explore?** Check out the [Theme Gallery](./gallery.md) or dive into [creating your own theme](./creating.md).
