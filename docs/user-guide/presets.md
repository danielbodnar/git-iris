# Instruction Presets

Presets allow you to customize Iris's behavior and output style across all commands. They provide pre-configured instructions for common use cases, from professional and technical to creative and playful.

## Quick Example

```bash
# Use concise preset for commit
git-iris gen --preset concise

# Detailed code review
git-iris review --preset detailed

# Technical PR description
git-iris pr --from main --to feature-branch --preset technical

# User-focused changelog
git-iris changelog --from v1.0.0 --preset user-focused
```

## Using Presets

### Command Line

```bash
# Any command supports --preset
git-iris <command> --preset <preset-name>

# Examples
git-iris gen --preset technical
git-iris review --preset detailed
git-iris pr --preset concise
git-iris changelog --preset user-focused
```

### List Available Presets

```bash
git-iris list-presets
```

Output shows preset name, emoji, and description for each available preset.

## Available Presets

### Professional Presets

| Preset      | Emoji | Description                 | Best For                      |
| ----------- | ----- | --------------------------- | ----------------------------- |
| `default`   | 📝    | Standard professional style | General use                   |
| `concise`   | 🎯    | Short and to-the-point      | Quick commits, patch releases |
| `detailed`  | 🔍    | Comprehensive explanations  | Major features, complex PRs   |
| `technical` | ⚙️    | Focus on technical details  | API changes, architecture     |
| `formal`    | 🎩    | Highly professional tone    | Enterprise, official releases |

**Examples:**

```bash
# Concise commit message
git-iris gen --preset concise
# Output: "fix: Resolve auth token expiry bug"

# Detailed review
git-iris review --preset detailed
# Output: Comprehensive analysis with explanations and context

# Technical PR
git-iris pr --preset technical --from main --to refactor-branch
# Output: Focus on implementation details, patterns, performance
```

### User-Focused Presets

| Preset         | Emoji | Description                 | Best For                          |
| -------------- | ----- | --------------------------- | --------------------------------- |
| `user-focused` | 👥    | Emphasize user impact       | Changelogs, release notes         |
| `explanatory`  | 💡    | Focus on the "why"          | Complex changes, breaking changes |
| `conventional` | 📏    | Strict Conventional Commits | Automated workflows, consistency  |

**Examples:**

```bash
# User-focused changelog
git-iris changelog --from v2.0.0 --preset user-focused
# Output: Emphasizes benefits and user-visible improvements

# Explanatory commit
git-iris gen --preset explanatory
# Output: Includes rationale and context for changes
```

### Creative Presets

| Preset        | Emoji | Description            | Best For                            |
| ------------- | ----- | ---------------------- | ----------------------------------- |
| `storyteller` | 📚    | Narrative-style output | Project updates, team communication |
| `emoji-lover` | 😍    | Enhanced with emojis   | Visual communication, fun projects  |
| `chill`       | 😎    | Professional but fun   | Casual teams, side projects         |

**Examples:**

```bash
# Storytelling release notes
git-iris release-notes --from v1.0.0 --preset storyteller
# Output: Narrative arc connecting features and improvements

# Emoji-rich commit
git-iris gen --preset emoji-lover
# Output: Abundant emoji to enhance visual communication
```

### Analytical Presets

| Preset            | Emoji | Description             | Best For                              |
| ----------------- | ----- | ----------------------- | ------------------------------------- |
| `comparative`     | ⚖️    | Highlight differences   | Migration guides, refactoring         |
| `future-oriented` | 🔮    | Focus on implications   | Roadmap items, architectural changes  |
| `academic`        | 🎓    | Scholarly analysis      | Research projects, technical papers   |
| `hater`           | 💢    | Hyper-critical feedback | Code quality audits, security reviews |

**Examples:**

```bash
# Comparative review
git-iris review --preset comparative --from old-approach --to new-approach
# Output: Highlights differences and trade-offs

# Critical security review
git-iris review --preset hater --instructions "Security audit"
# Output: Brutally honest, focuses on flaws and risks
```

### Fun Presets

| Preset               | Emoji | Description              | Best For                            |
| -------------------- | ----- | ------------------------ | ----------------------------------- |
| `cosmic`             | 🔮    | Mystical cosmic energy   | Fun projects, creative teams        |
| `time-traveler`      | ⏳    | Narrate across time      | Version comparisons, legacy updates |
| `chef-special`       | 👩‍🍳    | Culinary metaphors       | Creative documentation, team fun    |
| `superhero-saga`     | 🦸    | Comic book style         | Gamification, engaging updates      |
| `nature-documentary` | 🌿    | David Attenborough style | Observational analysis, ecosystems  |

**Examples:**

```bash
# Cosmic commit message
git-iris gen --preset cosmic
# Output: "✨ The cosmos aligned as authentication energies merged..."

# Nature documentary review
git-iris review --preset nature-documentary
# Output: "Here we observe the authentication module in its natural habitat..."
```

### Review-Specific Presets

These presets are scoped to `git-iris review` and tune Iris toward a specific lens. `git-iris list-presets` prints them under a dedicated "Review-specific Presets" header.

| Preset            | Emoji | Description                                       | Best For                                |
| ----------------- | ----- | ------------------------------------------------- | --------------------------------------- |
| `security`        | 🔒    | Vulnerabilities, auth, data exposure, hardening    | Security audits, sensitive changes      |
| `performance`     | ⚡    | Bottlenecks, hot paths, resource usage             | Hot-path PRs, perf regressions          |
| `architecture`    | 🏗️    | SOLID, coupling, separation of concerns            | Refactors, design-heavy PRs             |
| `testing`         | 🧪    | Coverage gaps, edge cases, test strategy           | PRs landing without tests, test rewrites |
| `maintainability` | 🔧    | Readability, naming, long-term maintenance         | Large changes, onboarding-sensitive code |
| `conventions`     | 📏    | Language/project style and consistency             | Codebase consistency checks             |

**Examples:**

```bash
# Security-focused review
git-iris review --preset security

# Performance audit
git-iris review --preset performance --from main --to hot-path-fix

# Architecture pass on a refactor
git-iris review --preset architecture --from main --to refactor-branch
```

These are not valid for `git-iris gen` — Iris will warn if you pass a review-only preset to a commit command.

## Custom Instructions

Combine presets with custom instructions for fine-tuned control:

```bash
# Preset + custom focus
git-iris gen --preset detailed --instructions "Emphasize performance impacts"

# Technical review with security focus
git-iris review --preset technical --instructions "Deep dive on security vulnerabilities"

# User-focused changelog with migration notes
git-iris changelog --from v1.0.0 --preset user-focused \
  --instructions "Include clear migration steps for breaking changes"
```

## Custom Instructions Only

Skip presets and use only custom instructions:

```bash
# No preset, just custom instructions
git-iris gen --instructions "Focus on API changes and backward compatibility"

git-iris review --instructions "Analyze only security implications"

git-iris pr --instructions "Write for non-technical stakeholders"
```

## Preset vs Custom Instructions

| Aspect          | Presets               | Custom Instructions     |
| --------------- | --------------------- | ----------------------- |
| **Ease of Use** | One flag, consistent  | Must write each time    |
| **Consistency** | Same style every time | Varies by wording       |
| **Flexibility** | Pre-defined options   | Unlimited possibilities |
| **Combinable**  | Yes, with `-i` flag   | Yes, with `--preset`    |

**Best Practice:** Use presets for general style, custom instructions for specific focus.

## Configuration

### Set Default Preset

Edit `~/.config/git-iris/config.toml`:

```toml
[general]
default_preset = "technical"
```

### Project-Specific Presets

Create `.irisconfig` in your repository:

```toml
[general]
default_preset = "conventional"
```

Useful for enforcing team-wide consistency.

## Common Workflows

### Commit Messages

```bash
# Quick fixes
git-iris gen --preset concise

# Feature implementations
git-iris gen --preset detailed

# Breaking changes
git-iris gen --preset explanatory

# Conventional commits for automation
git-iris gen --preset conventional
```

### Code Reviews

```bash
# Standard review
git-iris review --preset default

# Deep technical analysis
git-iris review --preset technical

# Security audit
git-iris review --preset hater --instructions "Focus on vulnerabilities"

# Quick check
git-iris review --preset concise
```

### Pull Requests

```bash
# Standard PR
git-iris pr --preset default

# Complex feature
git-iris pr --preset detailed

# User-facing changes
git-iris pr --preset user-focused

# Technical refactoring
git-iris pr --preset technical --instructions "Emphasize architecture improvements"
```

### Changelogs & Release Notes

```bash
# User-friendly changelog
git-iris changelog --from v1.0.0 --preset user-focused

# Technical changelog
git-iris changelog --from v1.0.0 --preset technical

# Detailed release notes
git-iris release-notes --from v2.0.0 --preset detailed

# Storytelling release
git-iris release-notes --from v2.0.0 --preset storyteller
```

## Tips

**For Team Consistency:**

- Set a default preset in `.irisconfig`
- Document which presets to use for different scenarios
- Use `conventional` preset for automated workflows

**For Flexibility:**

- Combine presets with custom instructions
- Use different presets for different commands
- Experiment to find what works for your workflow

**For Fun:**

- Try creative presets for side projects
- Use `nature-documentary` for architecture reviews
- `cosmic` preset for Friday deployments

**For Professionalism:**

- Stick to `default`, `technical`, or `formal` for work
- Use `user-focused` for external communication
- `detailed` for important documentation

## Examples

```bash
# Professional commit
git-iris gen --preset formal

# Fun commit for personal project
git-iris gen --preset cosmic

# Critical code review
git-iris review --preset hater

# User-friendly release notes
git-iris release-notes --from v2.0.0 --preset user-focused

# Technical changelog with custom focus
git-iris changelog --from v1.0.0 --preset technical \
  --instructions "Include API breaking changes section"

# Detailed PR with migration guide
git-iris pr --from v1 --to v2 --preset detailed \
  --instructions "Provide comprehensive migration guide"

# Quick concise review
git-iris review --preset concise --print
```

## Creating Workflows

### Git Aliases with Presets

Add to `~/.gitconfig`:

```ini
[alias]
    # Commit shortcuts
    cgen = !git-iris gen --preset concise
    cdet = !git-iris gen --preset detailed
    ctech = !git-iris gen --preset technical

    # Review shortcuts
    rquick = !git-iris review --preset concise
    rdeep = !git-iris review --preset detailed --instructions "Security and performance"

    # PR shortcuts
    pr-user = !f() { git-iris pr --from ${1:-main} --to ${2:-HEAD} --preset user-focused; }; f
    pr-tech = !f() { git-iris pr --from ${1:-main} --to ${2:-HEAD} --preset technical; }; f
```

Usage:

```bash
git cgen          # Concise commit
git rdeep         # Deep review
git pr-user main  # User-focused PR
```

## Preset Reference

For the complete, up-to-date list of presets:

```bash
git-iris list-presets
```

Each preset is designed for specific scenarios. Experiment to find what works best for your workflow!
