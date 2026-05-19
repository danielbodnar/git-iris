# Environment Variables

Configure Git-Iris using environment variables for CI/CD, Docker, or temporary overrides.

## API Key Variables

Each provider has a corresponding environment variable:

| Provider  | Environment Variable | Example      |
| --------- | -------------------- | ------------ |
| OpenAI    | `OPENAI_API_KEY`     | `sk-...`     |
| Anthropic | `ANTHROPIC_API_KEY`  | `sk-ant-...` |
| Google    | `GOOGLE_API_KEY`     | `AIza...`    |

### Usage

```bash
# Set API key
export ANTHROPIC_API_KEY="sk-ant-..."

# Git-Iris will use it automatically
git-iris gen
```

## Docker Example

```bash
docker run --rm \
  -v "$(pwd):/git-repo" \
  -e OPENAI_API_KEY="$OPENAI_API_KEY" \
  hyperb1iss/git-iris gen --provider openai --print
```

## Configuration Priority

Environment variables take precedence over config files:

```
Config File < Environment Variable < CLI Flag
```

### Example

**Config file:**

```toml
[providers.anthropic]
api_key = "sk-ant-config-key"
```

**Environment:**

```bash
export ANTHROPIC_API_KEY="sk-ant-env-key"
```

**Result:** Git-Iris uses `sk-ant-env-key`

## Common Patterns

### Local Development

```bash
# .envrc (direnv)
export ANTHROPIC_API_KEY="sk-ant-..."
export OPENAI_API_KEY="sk-..."
```

### CI/CD

```yaml
# GitHub Actions
env:
  ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}

steps:
  - name: Generate commit message
    run: |
      git add .
      git-iris gen --print > commit_msg.txt
```

### Docker Compose

```yaml
# docker-compose.yml
services:
  git-iris:
    image: hyperb1iss/git-iris
    environment:
      OPENAI_API_KEY: ${OPENAI_API_KEY}
    volumes:
      - .:/git-repo
```

## Shell Configuration

### Bash/Zsh

```bash
# ~/.bashrc or ~/.zshrc
export ANTHROPIC_API_KEY="sk-ant-..."
export OPENAI_API_KEY="sk-..."
export GOOGLE_API_KEY="..."
```

### Fish

```fish
# ~/.config/fish/config.fish
set -gx ANTHROPIC_API_KEY "sk-ant-..."
set -gx OPENAI_API_KEY "sk-..."
set -gx GOOGLE_API_KEY "..."
```

## Temporary Overrides

### One-Time Command

```bash
# Use different API key just for this command
ANTHROPIC_API_KEY="sk-ant-temp-key" git-iris gen
```

### Session Override

```bash
# Override for this shell session
export ANTHROPIC_API_KEY="sk-ant-session-key"
git-iris gen
git-iris review
# ... more commands
```

## Security Best Practices

### Never Commit Environment Files

```bash
# .gitignore
.env
.envrc
*.env.local
```

### Use Secret Management

```bash
# Load from 1Password
export ANTHROPIC_API_KEY="$(op read op://vault/item/field)"

# Load from AWS Secrets Manager
export ANTHROPIC_API_KEY="$(aws secretsmanager get-secret-value \
  --secret-id git-iris/anthropic-key --query SecretString --output text)"
```

### Restrict File Permissions

```bash
chmod 600 .env
chmod 600 .envrc
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Git-Iris Commit Check

on: [pull_request]

jobs:
  check-commit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Git-Iris
        run: |
          curl -fsSL https://raw.githubusercontent.com/hyperb1iss/git-iris/main/install.sh \
            | IRIS_INSTALL_DIR=/usr/local/bin sh

      - name: Generate commit message
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: |
          git-iris gen --print
```

### GitLab CI

```yaml
# .gitlab-ci.yml
commit-check:
  stage: test
  image: hyperb1iss/git-iris
  variables:
    OPENAI_API_KEY: ${OPENAI_API_KEY}
  script:
    - git-iris gen --provider openai --print
  only:
    - merge_requests
```

### Jenkins

```groovy
pipeline {
  agent any

  environment {
    ANTHROPIC_API_KEY = credentials('anthropic-api-key')
  }

  stages {
    stage('Generate Commit') {
      steps {
        sh 'git-iris gen --print'
      }
    }
  }
}
```

## Validation

### Check API Key Loading

```bash
# Enable debug logging
git-iris gen --log --log-file debug.log

# Check which API key source is used
cat debug.log | grep "API key"
```

### Test Environment Setup

```bash
# This should work if OPENAI_API_KEY is set
git-iris gen --provider openai --print

# If it fails, check:
echo $OPENAI_API_KEY  # Should output key (or empty if not set)
```

## Debugging

### API Key Not Found

```
Error: API key required for openai.
Set OPENAI_API_KEY or configure in ~/.config/git-iris/config.toml
```

**Solution:**

```bash
export OPENAI_API_KEY="sk-proj-..."
```

### Wrong Provider Selected

```bash
# Check current provider
git-iris config --print | grep default_provider

# Override for this command
git-iris gen --provider openai --print
```

### Environment Variable Not Loading

```bash
# Verify it's exported
env | grep API_KEY

# If not, export it
export OPENAI_API_KEY="sk-proj-..."

# Test immediately
git-iris gen --print
```

## Reference Table

### All Environment Variables

| Variable            | Purpose                  | Example         |
| ------------------- | ------------------------ | --------------- |
| `OPENAI_API_KEY`    | OpenAI authentication    | `sk-proj-...`   |
| `ANTHROPIC_API_KEY` | Anthropic authentication | `sk-ant-...`    |
| `GOOGLE_API_KEY`    | Google authentication    | `AIza...`       |
| `RUST_LOG`          | Logging level            | `debug`, `info` |

### Example Complete Setup

```bash
# Personal development
export OPENAI_API_KEY="sk-proj-personal-dev-key"
export ANTHROPIC_API_KEY="sk-ant-personal-dev-key"

# Debug logging
export RUST_LOG="debug"
```

## Platform-Specific Notes

### Windows (PowerShell)

```powershell
# Set environment variable
$env:ANTHROPIC_API_KEY = "sk-ant-..."

# Permanent (user scope)
[Environment]::SetEnvironmentVariable("ANTHROPIC_API_KEY", "sk-ant-...", "User")
```

### Windows (CMD)

```cmd
set ANTHROPIC_API_KEY=sk-ant-...
```

### macOS/Linux

```bash
# Temporary
export ANTHROPIC_API_KEY="sk-ant-..."

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export ANTHROPIC_API_KEY="sk-ant-..."' >> ~/.bashrc
```
