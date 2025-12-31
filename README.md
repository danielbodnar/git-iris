# 🔮 Git-Iris: Your Agentic Git Companion

<div align="center">

[![CI/CD](https://img.shields.io/github/actions/workflow/status/hyperb1iss/git-iris/cicd.yml?style=for-the-badge&logo=github-actions&logoColor=white&color=4C566A)](https://github.com/hyperb1iss/git-iris/actions)
[![Docker](https://img.shields.io/docker/pulls/hyperb1iss/git-iris?style=for-the-badge&logo=docker&logoColor=white&color=2496ED)](https://hub.docker.com/r/hyperb1iss/git-iris)
[![License](https://img.shields.io/badge/License-Apache%202.0-5E81AC?style=for-the-badge&logo=apache&logoColor=white&color=3B6EA8)](https://opensource.org/licenses/Apache-2.0)
[![GitHub Release](https://img.shields.io/github/release/hyperb1iss/git-iris.svg?style=for-the-badge&logo=github&logoColor=white&color=9D6DB3)][releases]
[![Crates.io](https://img.shields.io/crates/v/git-iris.svg?style=for-the-badge&logo=rust&logoColor=white&color=D35D47)][crates]
[![GitHub Action](https://img.shields.io/badge/GitHub_Action-Available-5E81AC?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/marketplace/actions/git-iris)
[![Rust](https://img.shields.io/badge/rust-stable-EBCB8B?style=for-the-badge&logo=rust&logoColor=white&color=EFBB4D)](https://www.rust-lang.org/)
[![ko-fi](https://img.shields.io/badge/Ko--fi-Support%20Me-A3BE8C?style=for-the-badge&logo=ko-fi&logoColor=white&color=82B062)](https://ko-fi.com/hyperb1iss)

✨ _An intelligent agent that understands your code and crafts perfect Git artifacts_ ✨

📖 [Documentation](https://hyperb1iss.github.io/git-iris/) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Studio](#-iris-studio) • [GitHub Action](#-github-action) • [Contributing](#-contributing)

</div>

<div align="center">
  <img src="https://raw.githubusercontent.com/hyperb1iss/git-iris/main/docs/images/git-iris-screenshot-1.png" alt="Commit Mode" width="24%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/git-iris/main/docs/images/git-iris-screenshot-2.png" alt="Code Review" width="24%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/git-iris/main/docs/images/git-iris-screenshot-3.png" alt="Changelog" width="24%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/git-iris/main/docs/images/git-iris-screenshot-4.png" alt="Chat with Iris" width="24%">
</div>

<p align="center"><em>Iris crafting commit messages, reviewing code, generating changelogs, and chatting about your changes</em></p>

## 💜 Overview

Git-Iris is powered by **Iris**, an intelligent agent that actively explores your codebase to understand what you're building. Rather than dumping context and hoping for the best, Iris uses tools to gather precisely the information she needs—analyzing diffs, exploring file relationships, and building understanding iteratively.

### 🪄 What Iris Can Do

| Capability             | Description                                                                   |
| ---------------------- | ----------------------------------------------------------------------------- |
| ✍️ **Commit Messages** | Context-aware messages that capture the essence of your changes               |
| 🔬 **Code Reviews**    | Multi-dimensional analysis covering security, performance, and best practices |
| 📜 **Pull Requests**   | Comprehensive PR descriptions for branches or individual commits              |
| 🗂️ **Changelogs**      | Keep a Changelog format with intelligent categorization                       |
| 🎊 **Release Notes**   | User-focused documentation highlighting impact and benefits                   |
| 🔭 **Semantic Blame**  | Ask "why does this code exist?" and get real answers                          |

### 🌌 Iris Studio

**Studio** is a stunning terminal interface built with the **SilkCircuit Neon** design language that brings everything together. Press `/` to chat with Iris, ask her to refine your commit message or explain changes—she can update content directly through intelligent tool calls!

## 📦 Installation

```bash
cargo install git-iris
```

Or via Docker:

```bash
docker pull hyperb1iss/git-iris:latest
```

Then configure your AI provider:

```bash
git-iris config --provider anthropic --api-key YOUR_API_KEY
```

Supports **OpenAI**, **Anthropic**, and **Google**. See the [Configuration Guide](https://hyperb1iss.github.io/git-iris/getting-started/configuration) for details.

## 🚀 Quick Start

### 🌌 Launch Studio

```bash
git-iris
```

That's it. Studio auto-detects your context and suggests the right mode.

### 💫 Generate Commit Messages

```bash
git add .
git-iris gen              # Interactive mode
git-iris gen --print      # Just print the message
git-iris gen -a           # Auto-commit with generated message
```

### 🔬 Review Code

```bash
git-iris review                              # Review staged changes
git-iris review --from main --to feature     # Compare branches
git-iris review --commit abc1234             # Review specific commit
```

### 🗂️ Generate Changelogs & Release Notes

```bash
git-iris changelog --from v1.0.0 --update    # Update CHANGELOG.md
git-iris release-notes --from v1.0.0         # Generate release notes
```

### 📜 Pull Request Descriptions

```bash
git-iris pr --from main --to feature-branch
```

## 🤖 GitHub Action

Automate release notes and changelogs in your CI/CD:

```yaml
- name: Generate release notes
  uses: hyperb1iss/git-iris@v1
  with:
    from: v1.0.0
    to: v1.1.0
    api-key: ${{ secrets.OPENAI_API_KEY }}
    output-file: RELEASE_NOTES.md
```

See the [GitHub Action documentation](https://hyperb1iss.github.io/git-iris/reference/cli#github-action) for all options.

## 📚 Documentation

Full documentation is available at **[hyperb1iss.github.io/git-iris](https://hyperb1iss.github.io/git-iris/)**

- 🚀 [Getting Started](https://hyperb1iss.github.io/git-iris/getting-started/) — Installation, configuration, quick start
- 🌌 [Studio Guide](https://hyperb1iss.github.io/git-iris/studio/) — Master the TUI, modes, and chat
- 📖 [CLI Reference](https://hyperb1iss.github.io/git-iris/reference/cli) — All commands and options
- 🏗️ [Architecture](https://hyperb1iss.github.io/git-iris/architecture/) — How Iris works under the hood

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## ⚖️ License

Distributed under the Apache 2.0 License. See `LICENSE` for more information.

---

<div align="center">

📚 [Documentation](https://hyperb1iss.github.io/git-iris/) · 🐛 [Report Bug](https://github.com/hyperb1iss/git-iris/issues) · 💡 [Request Feature](https://github.com/hyperb1iss/git-iris/issues)

</div>

<div align="center">

Created by [Stefanie Jane 🌠](https://github.com/hyperb1iss)

If you find Git-Iris useful, [buy me a Monster Ultra Violet](https://ko-fi.com/hyperb1iss)! ⚡️

</div>

[crates]: https://crates.io/crates/git-iris
[releases]: https://github.com/hyperb1iss/git-iris/releases
