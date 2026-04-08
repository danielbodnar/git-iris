# 🔧 Git-Iris Configuration Guide

> **📚 For detailed configuration guides, examples, and advanced topics, see the [full documentation](/docs/configuration/).**

Git-Iris uses a TOML configuration file located at `~/.config/git-iris/config.toml`. This document provides a quick reference for all available configuration options.

## 📁 Configuration Structure

The configuration file is organized into these main sections:

1. **Global settings** — Apply to all operations
2. **Default provider** — Which LLM to use by default
3. **Provider-specific configurations** — API keys, models, and parameters per provider

## ⚙️ Configuration Options

### Global Settings

| Option               | Type    | Default     | Description                                     |
| -------------------- | ------- | ----------- | ----------------------------------------------- |
| `use_gitmoji`        | Boolean | `false`     | Enable Gitmoji in commit messages               |
| `instructions`       | String  | `""`        | Custom instructions included in all LLM prompts |
| `instruction_preset` | String  | `"default"` | Default preset for AI instructions              |

**Examples:**

```toml
use_gitmoji = true
instructions = """
Always mention the ticket number if applicable.
Focus on the impact of changes rather than implementation details.
"""
instruction_preset = "conventional"
```

### Default Provider

| Option             | Type   | Default    | Description                     |
| ------------------ | ------ | ---------- | ------------------------------- |
| `default_provider` | String | `"openai"` | The default LLM provider to use |

**Example:**

```toml
default_provider = "openai"
```

### Provider-Specific Configurations

Each provider has its own subtable under `[providers]` with these fields:

| Field               | Type    | Required | Description                                           |
| ------------------- | ------- | -------- | ----------------------------------------------------- |
| `api_key`           | String  | Yes      | The provider's API key                                |
| `model`             | String  | No       | Primary model for complex analysis tasks              |
| `fast_model`        | String  | No       | Fast model for simple tasks (status updates, parsing) |
| `additional_params` | Table   | No       | Additional provider-specific parameters               |
| `token_limit`       | Integer | No       | Token limit override                                  |

## 🤖 Supported Providers

Git-Iris supports three LLM providers:

| Provider      | Default Model        | Fast Model                | Context Window | API Key Env         |
| ------------- | -------------------- | ------------------------- | -------------- | ------------------- |
| **openai**    | gpt-5.4              | gpt-5.4-mini              | 128,000        | `OPENAI_API_KEY`    |
| **anthropic** | claude-opus-4-6      | claude-haiku-4-5-20251001 | 200,000        | `ANTHROPIC_API_KEY` |
| **google**    | gemini-3-pro-preview | gemini-2.5-flash          | 1,000,000      | `GOOGLE_API_KEY`    |

> **Note:** `claude` and `gemini` are still supported as legacy aliases for `anthropic` and `google`.

## 📝 Example Configuration File

```toml
# Global settings
use_gitmoji = true
default_provider = "openai"
instruction_preset = "conventional"

instructions = """
Always mention the ticket number if applicable.
Focus on the impact of changes rather than implementation details.
"""

# OpenAI configuration
[providers.openai]
api_key = "sk-your-openai-api-key"
model = "gpt-5.4"
fast_model = "gpt-5.4-mini"
additional_params = { temperature = "0.7", max_tokens = "4096" }
token_limit = 8000

# Anthropic configuration
[providers.anthropic]
api_key = "sk-ant-your-anthropic-api-key"
model = "claude-opus-4-6"
fast_model = "claude-haiku-4-5-20251001"
additional_params = { temperature = "0.8" }
token_limit = 200000

# Google configuration
[providers.google]
api_key = "your-google-api-key"
model = "gemini-3-pro-preview"
fast_model = "gemini-2.5-flash"
additional_params = { temperature = "0.7" }
token_limit = 1048576
```

## 🖥️ CLI Configuration Commands

### Global Configuration

```bash
# Set provider and API key
git-iris config --provider openai --api-key YOUR_API_KEY

# Set models
git-iris config --provider anthropic --model claude-opus-4-6
git-iris config --provider anthropic --fast-model claude-haiku-4-5-20251001

# Set token limit
git-iris config --provider openai --token-limit 8000

# Set additional parameters
git-iris config --provider openai --param temperature=0.7 --param max_tokens=4096

# Enable Gitmoji
git-iris config --gitmoji

# Set custom instructions
git-iris config --instructions "Your custom instructions here"

# Set default preset
git-iris config --preset conventional
```

### Project Configuration

Project settings are stored in `.irisconfig` in your repository root:

```bash
# Set project-specific provider
git-iris project-config --provider google

# Set project-specific model
git-iris project-config --model gemini-3-pro-preview

# Set project-specific preset
git-iris project-config --preset security

# View current project configuration
git-iris project-config --print
```

> **Security:** Project configuration files do not store API keys, only models, presets, and custom instructions.

## 🔧 Environment Variables

You can also configure Git-Iris using environment variables:

| Variable            | Description                 |
| ------------------- | --------------------------- |
| `OPENAI_API_KEY`    | OpenAI API key              |
| `ANTHROPIC_API_KEY` | Anthropic API key           |
| `GOOGLE_API_KEY`    | Google API key              |
| `RUST_LOG`          | Logging level for debugging |

**Example (Docker/CI):**

```bash
docker run --rm -v "$(pwd):/git-repo" \
  -e OPENAI_API_KEY="$OPENAI_API_KEY" \
  hyperb1iss/git-iris gen --provider openai --print
```

## 🎛️ Instruction Presets

Git-Iris includes built-in instruction presets for different styles:

**General Presets:**

- `default` — Standard professional style
- `conventional` — Conventional Commits specification
- `detailed` — More context and explanation
- `concise` — Short and to-the-point
- `cosmic` — Mystical, space-themed language ✨

**Review-Specific Presets:**

- `security` — Focus on security vulnerabilities
- `performance` — Analyze performance optimizations
- `architecture` — Evaluate design patterns
- `testing` — Focus on test coverage
- `maintainability` — Long-term maintenance
- `conventions` — Coding standards

```bash
# List all available presets
git-iris list-presets
```

## ⚡ Token Optimization

Git-Iris automatically optimizes token usage to maximize context while staying within provider limits. The optimization strategy adapts based on:

- **Changeset size**: Small changes get full context; large changes use relevance scoring
- **File count**: 20+ files triggers parallel subagent analysis
- **Provider limits**: Respects each provider's context window

You can override limits per provider:

```bash
git-iris config --provider openai --token-limit 4000
```

## 🔒 Security Notes

- **Keep API keys secret** — Never share your configuration file containing API keys
- **File permissions** — Ensure `~/.config/git-iris/config.toml` is readable only by you
- **Environment variables** — Consider using env vars for API keys in shared environments
- **Project configs** — `.irisconfig` files don't store API keys for team safety

## 🐛 Troubleshooting

| Issue                     | Solution                                                    |
| ------------------------- | ----------------------------------------------------------- |
| **Authentication failed** | Verify API key is correct and has required permissions      |
| **Model not found**       | Check you're using a supported model for your provider      |
| **Token limit exceeded**  | Reduce `token_limit` or use a smaller changeset             |
| **Slow responses**        | Try a faster model with `--fast-model`                      |
| **Debug issues**          | Enable logging with `-l` or use `--debug` for agent details |

**Enable debug logging:**

```bash
git-iris gen --log --log-file debug.log
git-iris gen --debug  # Gorgeous color-coded agent execution
```

For further assistance, please refer to the [Git-Iris documentation](https://github.com/hyperb1iss/git-iris/wiki) or [open an issue](https://github.com/hyperb1iss/git-iris/issues).

---

## 📖 Full Configuration Documentation

For comprehensive configuration guides, advanced topics, and examples:

- **[Configuration Overview](/docs/configuration/)** — Complete configuration guide
- **[Provider Setup](/docs/configuration/providers.md)** — Detailed provider configuration
- **[Instruction Presets](/docs/configuration/presets.md)** — Custom instruction presets
- **[Project Configuration](/docs/configuration/project.md)** — Project-specific settings
