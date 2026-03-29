# Model Selection

Git-Iris uses a dual-model strategy: **primary models** for complex analysis and **fast models** for simple tasks.

## Model Strategy

| Model Type  | Used For                                       | Examples                                    |
| ----------- | ---------------------------------------------- | ------------------------------------------- |
| **Primary** | Commit messages, code reviews, PR descriptions | `claude-opus-4-6`, `gpt-5.4`, `gemini-3-pro-preview` |
| **Fast**    | Status updates, parsing, simple queries        | `claude-haiku-4-5-20251001`, `gpt-5.4-mini`, `gemini-2.5-flash` |

This dual-model approach optimizes for both quality and speed.

## Default Models by Provider

### OpenAI

```toml
[providers.openai]
model = "gpt-5.4"
fast_model = "gpt-5.4-mini"
```

| Model          | Use Case | Context | Notes                     |
| -------------- | -------- | ------- | ------------------------- |
| `gpt-5.4`      | Primary  | 128K    | Best for complex analysis |
| `gpt-5.4-mini` | Fast     | 128K    | Quick status updates      |

Git-Iris also applies workflow-specific GPT-5 reasoning defaults for OpenAI:

- Main agent tasks use `medium` reasoning for commit/review/PR quality
- Subagents and `parallel_analyze` use `low` reasoning to stay fast but thoughtful
- Status messages use `none` reasoning to keep waiting text snappy

If you want a different OpenAI reasoning level everywhere, set `additional_params.reasoning` for
that provider.

### Anthropic

```toml
[providers.anthropic]
model = "claude-opus-4-6"
fast_model = "claude-haiku-4-5-20251001"
```

| Model                        | Use Case | Context | Notes                       |
| ---------------------------- | -------- | ------- | --------------------------- |
| `claude-opus-4-6`            | Primary  | 200K    | Excellent for code analysis |
| `claude-haiku-4-5-20251001`  | Fast     | 200K    | Fastest response times      |

### Google

```toml
[providers.google]
model = "gemini-3-pro-preview"
fast_model = "gemini-2.5-flash"
```

| Model                  | Use Case | Context | Notes                  |
| ---------------------- | -------- | ------- | ---------------------- |
| `gemini-3-pro-preview` | Primary  | 1M      | Largest context window |
| `gemini-2.5-flash`     | Fast     | 1M      | Good for large diffs   |

## Configuring Models

### Via CLI

```bash
# Set primary model
git-iris config --provider anthropic --model claude-opus-4-6

# Set fast model
git-iris config --provider anthropic --fast-model claude-haiku-4-5-20251001
```

### Via Config File

```toml
[providers.anthropic]
api_key = "sk-ant-..."
model = "claude-opus-4-6"
fast_model = "claude-haiku-4-5-20251001"
```

## When to Use Which Model

### Primary Model Tasks

- **Commit message generation** — Needs context understanding
- **Code reviews** — Requires deep analysis
- **PR descriptions** — Synthesizes multiple changes
- **Changelogs** — Categorizes and summarizes
- **Release notes** — Produces polished documentation

### Fast Model Tasks

- **Status updates** — "Analyzing file 3 of 15..."
- **Progress parsing** — Extracting structured data
- **Tool responses** — Simple confirmations
- **Chat queries** — Quick interactions

## Model Selection Criteria

### Choose Primary Model Based On

| Priority    | Consideration                | Recommendation           |
| ----------- | ---------------------------- | ------------------------ |
| **Quality** | Need best analysis           | Claude Opus 4.6          |
| **Context** | Large changesets (>50 files) | Gemini 3 Pro (1M tokens) |
| **Speed**   | Fast turnaround              | GPT-5.4-mini or Gemini 2.5 Flash |
| **Cost**    | Budget constraints           | Use fast models more     |

### Optimize Fast Model For

- **Response time** — Haiku, GPT-5.4-mini, Gemini Flash
- **Context window** — All fast models support large context
- **Availability** — Check provider rate limits

## Custom Model Configuration

You can use any model supported by your provider:

```bash
# OpenAI custom model
git-iris config --provider openai --model gpt-5.4

# Anthropic custom model
git-iris config --provider anthropic --model claude-haiku-4-5-20251001

# Google custom model
git-iris config --provider google --model gemini-2.5-flash
```

## Model Fallback Behavior

If a configured model is unavailable, Git-Iris will:

1. Attempt to use the provider's default model
2. Report an error with the model name
3. Suggest checking provider documentation

## Context Window Management

Git-Iris automatically manages context to fit within model limits:

| Scenario                       | Strategy                   |
| ------------------------------ | -------------------------- |
| Small changeset (<10 files)    | Full context               |
| Medium changeset (10-20 files) | Relevance scoring          |
| Large changeset (20+ files)    | Parallel subagent analysis |

Override token limits per provider:

```bash
git-iris config --provider anthropic --token-limit 150000
```

## Model Performance Tips

### For Large Repositories

```toml
[providers.google]
model = "gemini-3-pro-preview"  # 1M context window
```

### For Speed-Critical Workflows

```toml
[providers.anthropic]
model = "claude-haiku-4-5-20251001"  # Fast even for primary tasks
fast_model = "claude-haiku-4-5-20251001"
```

### For Maximum Quality

```toml
[providers.anthropic]
model = "claude-opus-4-6"  # Best code understanding
```

## Monitoring Model Usage

Enable debug mode to see which model handles each task:

```bash
git-iris gen --debug
```

Output shows:

- Model name
- Token usage
- Tool calls
- Response time

## Cost Optimization

### Minimize Costs

```toml
# Use fast model for everything
[providers.openai]
model = "gpt-5.4-mini"
fast_model = "gpt-5.4-mini"
token_limit = 8000  # Lower limit
```

### Balance Quality and Cost

```toml
# Standard setup
[providers.anthropic]
model = "claude-opus-4-6"  # Quality for commits
fast_model = "claude-haiku-4-5-20251001"  # Speed for status
```

## Troubleshooting

| Issue             | Solution                                             |
| ----------------- | ---------------------------------------------------- |
| "Model not found" | Check provider documentation for available models    |
| Slow responses    | Switch to a faster model                             |
| Context exceeded  | Reduce `token_limit` or use model with larger window |
| Poor quality      | Use a more capable primary model                     |

## Model Comparison Table

| Model             | Provider  | Context | Speed  | Quality   | Cost   |
| ----------------- | --------- | ------- | ------ | --------- | ------ |
| claude-opus-4-6   | Anthropic | 200K    | Medium | Excellent | Medium |
| claude-haiku-4-5  | Anthropic | 200K    | Fast   | Good      | Low    |
| gpt-5.4           | OpenAI    | 128K    | Medium | Excellent | Medium |
| gpt-5.4-mini      | OpenAI    | 128K    | Fast   | Good      | Low    |
| gemini-3-pro-preview | Google | 1M      | Slow   | Excellent | High   |
| gemini-2.5-flash  | Google    | 1M      | Fast   | Good      | Low    |
