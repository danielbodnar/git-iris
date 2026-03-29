# Configuration Overview

Git-Iris uses a layered configuration system that combines personal settings, project-specific settings, and runtime overrides.

## Configuration Hierarchy

1. **Personal Config** (`~/.config/git-iris/config.toml`) — Your global defaults
2. **Project Config** (`.irisconfig` in repo root) — Team-shared settings
3. **Environment Variables** — Runtime overrides
4. **CLI Flags** — Command-specific overrides

Settings are merged in this order (later takes precedence), except **API keys are never loaded from project config** for security.

## Quick Start

```bash
# Set up your provider
git-iris config --provider anthropic --api-key YOUR_API_KEY

# Set preferred models
git-iris config --model claude-sonnet-4-5-20250929
git-iris config --fast-model claude-haiku-4-5-20251001

# Enable gitmoji
git-iris config --use-gitmoji true
```

## Configuration Files

### Personal Config Location

**macOS/Linux:**

```
~/.config/git-iris/config.toml
```

**Windows:**

```
%APPDATA%\git-iris\config.toml
```

### Project Config Location

```
.irisconfig  (in repository root)
```

## Configuration Sections

| Section       | Description                                                  | Scope                 |
| ------------- | ------------------------------------------------------------ | --------------------- |
| **Global**    | `use_gitmoji`, `instructions`, `instruction_preset`, `theme` | All operations        |
| **Provider**  | `default_provider`                                           | Which LLM to use      |
| **Providers** | `api_key`, `model`, `fast_model`, `token_limit`              | Per-provider settings |

## Basic Configuration Structure

```toml
# Global settings
use_gitmoji = true
instruction_preset = "conventional"
theme = "silkcircuit-neon"

# Default provider
default_provider = "anthropic"

# Provider configurations
[providers.anthropic]
api_key = "sk-ant-..."
model = "claude-sonnet-4-5-20250929"
fast_model = "claude-haiku-4-5-20251001"

[providers.openai]
api_key = "sk-..."
model = "gpt-5.4"
fast_model = "gpt-5.4-mini"
```

## Global Settings

| Setting              | Type    | Default     | Description                                   |
| -------------------- | ------- | ----------- | --------------------------------------------- |
| `use_gitmoji`        | Boolean | `true`      | Enable emoji prefixes in commit messages      |
| `instructions`       | String  | `""`        | Custom instructions for all LLM operations    |
| `instruction_preset` | String  | `"default"` | Built-in instruction preset name              |
| `theme`              | String  | `""`        | Theme name (empty = default SilkCircuit Neon) |
| `default_provider`   | String  | `"openai"`  | Default LLM provider                          |

## Next Steps

- **[Providers](providers.md)** — Configure OpenAI, Anthropic, or Google
- **[Models](models.md)** — Choose the right model for your needs
- **[Project Config](project-config.md)** — Share settings with your team
- **[Environment Variables](environment.md)** — Runtime configuration
