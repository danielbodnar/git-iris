## [2.0.6] - 2026-03-02

This release migrates the **theme engine to the standalone Opaline library** (removing ~3,600 lines of code), adds **Google/Gemini support for parallel subagents**, and improves **API key security** with validation and sanitized error messages.

### Added

- ✨ Add **Google/Gemini provider support** to `parallel_analyze` subagents alongside OpenAI and Anthropic (14c27a9)
- ✨ Expand OpenAI API key validation to accept both `sk-` and `sk-proj-` prefixes for project-scoped keys
- ✨ Add `api_key_if_set()` helper to `ProviderConfig` for cleaner empty-string handling
- ✨ Add content update tools (`UpdateCommitTool`, `UpdatePRTool`, `UpdateReviewTool`) to agent builders for chat mode (f241f14)

### Changed

- ♻️ Replace custom theme engine with **Opaline library** (`opaline = "0.2.0"`), removing entire `src/theme/` directory (~4,262 lines) (7f364d1)
- ♻️ Replace hardcoded theme token strings with `opaline::names` constants for compile-time validation (4eeb8c3)
- ⬆️ Upgrade dependencies: `ratatui` 0.29→0.30, `crossterm` 0.28→0.29, `tui-textarea`→`ratatui-textarea` 0.8.0
- ♻️ Replace `DynClientBuilder` with explicit provider dispatch using `DynAgent` enum for rig-core 0.27+ compatibility (dfbd9c5)
- ♻️ Switch Studio generation tasks from streaming to non-streaming execution for clean structured output parsing (228afda)
- ♻️ Migrate CI/CD workflows to shared-workflows repository, reducing workflow files from 887→654 lines (1ff091d)
- ⬆️ Bump GitHub Actions: `actions/checkout` v4→v6, `actions/setup-node` v4→v6, `docker/build-push-action` v5→v6
- ♻️ Switch to **OIDC trusted publishing** for crates.io via `rust-lang/crates-io-auth-action`
- ♻️ Apply Clippy suggestions: `sort_by_key`, `is_ok_and()`, `checked_div()` for idiomatic Rust patterns (d66dd2f)
- ♻️ Replace `unwrap()` with `expect()` in tests for clearer failure diagnostics (5d5f81e)

### Fixed

- 🐛 Remove silent fallback to OpenAI when configured provider fails in `parallel_analyze`—now returns clear error (713a9fe)
- 🐛 Fix state management: mark state dirty when opening settings modal, load file tree when switching to Explore mode (5dd47cd)
- 🐛 Fix theme tests to handle parallel execution safely by removing global state assertions (be64764)
- 🐛 Replace panicking `unwrap`/`expect` calls with proper error handling in `ParallelAnalyze`, `FileWatcherService`, and `GitRepo` (ee4a7df)

### Security

- 🔒️ **Prevent API key exposure in error messages**: sanitize client creation errors to avoid leaking key material (aeefd91)
- 🔒️ Add `validate_api_key_format()` for format validation (prefix and length checks) to catch misconfigurations early (d2b02e6)
- 🔒️ Make `resolve_api_key()` and `ApiKeySource` public for consistent resolution across codebase
- 🔒️ Update test assertions to verify error messages don't contain key fragments

### Removed

- 🔥 Remove entire `src/theme/` directory (36 files): `color.rs`, `gradient.rs`, `schema.rs`, `loader.rs`, `resolver.rs`, adapters, and 13 builtin theme TOML files
- 🔥 Remove unused dependencies: `once_cell`, `tiktoken-rs`, `tokio-retry`

### Metrics

- Total Commits: 14
- Files Changed: 61
- Insertions: +611
- Deletions: -4,987

## [2.0.4] - 2026-01-09

This release improves **API key management** with config-based credential resolution, adds **Google/Gemini support** for parallel analysis subagents, and strengthens error handling across provider operations.

### Added

- ✨ Add Google/Gemini provider support for `parallel_analyze` subagents, enabling concurrent analysis with all three providers (14c27a9)
- ✨ Add `api_key_if_set()` helper to `ProviderConfig` for cleaner empty-string handling when extracting keys
- ✨ Add `ApiKeySource` enum to track API key resolution path (config → environment → client default) for debugging
- ✨ Add `validate_api_key_format()` method to `Provider` for early detection of misconfigurations (prefix and length checks)
- ✨ Expand OpenAI API key validation to accept both `sk-` and `sk-proj-` prefixes for project-scoped keys

### Changed

- 🔧 Pass API keys from config to provider builders instead of requiring environment variables for all clients (25e293b)
- 🔧 Thread `api_key` through all agent builder paths including main agents, subagents, and debug agents
- 🔧 Update `StatusMessageGenerator` to accept and use API keys from config
- 🔧 Provider builder functions (`openai_builder`, `anthropic_builder`, `gemini_builder`) now accept optional API key parameters with environment fallback
- 🔧 Make `resolve_api_key()` and `ApiKeySource` public in provider module for reuse across codebase

### Fixed

- 🐛 Remove silent fallback to OpenAI in `parallel_analyze` when requested provider fails—now returns clear error message (713a9fe)
- 🐛 Handle errors in LLM builder functions with context-aware error returns using `anyhow` instead of panics (4019c3b)

### Security

- 🔒 Prevent API key exposure in error messages by sanitizing client creation errors (aeefd91)
- 🔒 Remove key prefix display from validation errors, using generic "unexpected prefix" message
- 🔒 Update test assertions to verify error messages don't contain key fragments

### Metrics

- Total Commits: 6
- Files Changed: 7
- Insertions: +548
- Deletions: -53

## [2.0.3] - 2025-12-31

Patch release fixing GitHub Action installation failures caused by asset naming mismatches.

### Fixed

- 🐛 Fix binary download and installation in GitHub Action by adding wildcard to release download pattern and dynamically detecting downloaded filename (028a6c6)

### Metrics

- Total Commits: 1
- Files Changed: 1
- Insertions: +6
- Deletions: -4

## [2.0.2] - 2025-12-31

This patch release improves **CI/CD reliability** with better version parsing and adds **AUR package publishing** for Arch Linux users.

### Added

- ✨ Add automated **AUR package publishing** to CI/CD pipeline with dynamically calculated SHA256 checksums for x86_64 and aarch64 binaries (01f7ce78)
- 📝 Add comprehensive **GitHub Action documentation** page with quick start examples, input/output reference tables, and provider-specific configurations (b8349aa9)

### Changed

- 🔧 Update GitHub Action metadata to "Git-Iris Action" with clearer description: "AI agent that crafts perfect Git artifacts" (b2fd2023)
- 📝 Update README GitHub Action link to point to new dedicated documentation page

### Fixed

- 🐛 Fix version detection to filter semver tags only using explicit `v[0-9]*.[0-9]*.[0-9]*` pattern, excluding non-standard tags like `v2` (d1aceb34)
- 🐛 Fix version parsing in release workflow using `cut` instead of IFS-based splitting for reliable cross-shell behavior (b4823bcf)

### Metrics

- Total Commits: 5
- Files Changed: 6
- Insertions: +322
- Deletions: -12

## [2.0.1] - 2025-12-31

This release introduces **Iris Studio**, a stunning terminal interface for AI-powered Git workflows, and completes the transition to an **agent-first architecture** where Iris actively explores your codebase using tool calls rather than static context dumps.

### Added

- ✨ Add **Iris Studio** TUI with six specialized modes: Explore, Commit, Review, PR, Changelog, and Release Notes (82ad0ac)
- ✨ Add **agent-first architecture** powered by the Rig framework with dynamic tool-based context gathering
- ✨ Add **interactive chat** with Iris in all Studio modes via `/` key—Iris can modify content directly through tool calls (eae0604, b53dc1b)
- ✨ Add **token-based theme system** with TOML configuration and 13 built-in themes: SilkCircuit (Neon, Soft, Glow, Vibrant, Dawn), Catppuccin (Mocha, Latte), Dracula, Nord, Tokyo Night, Gruvbox Dark, One Dark, Solarized Light (bf4e80f, f72b66b, 372b7dc)
- ✨ Add **Iris Companion** for ambient session awareness with branch memory persistence and live file watching (6c42718)
- ✨ Add **GitHub Action** for automated release notes and changelog generation in CI/CD pipelines (fb776c5)
- ✨ Add **shell installer** (`curl -fsSL ... | sh`) and **Homebrew tap** (`brew install hyperb1iss/tap/git-iris`) for easy installation (bda1718)
- ✨ Add **AUR package** for Arch Linux users (0c93043)
- ✨ Add **semantic blame** feature—press `w` in Explore mode to ask "why does this code exist?" (1dc5dd6)
- ✨ Add **amend mode** (`git-iris gen --amend` and `Shift+A` in Studio) for modifying previous commits (c877314, 7e34b94)
- ✨ Add **real-time streaming** response display for chat and generation tasks (443ed16, 5a15d42)
- ✨ Add **dynamic status messages** using fast model for witty, contextual waiting feedback (7ccdb9a)
- ✨ Add **file history panel** in Explore mode with commit navigation and `y` to copy hash (a6ecb66)
- ✨ Add **syntax highlighting** for code blocks in chat and Explore mode using syntect (dd811c0, 5a15d42)
- ✨ Add **commit count picker** modal (`#` key) for quick HEAD~N ref selection (624644d)
- ✨ Add **shell completions** for bash, zsh, fish, elvish, and powershell via `completions` subcommand (bebffef)
- ✨ Add `--model` flag for per-operation LLM model override without changing global config (3e51088)
- ✨ Add `--amend` flag to `gen` command and `--update`/`--file` flags to `release-notes` command (bebffef)
- ✨ Add `--raw` flag to `review`, `pr`, `changelog`, and `release-notes` for clean markdown output (c67f17c)
- ✨ Add **project context** doc type for unified README and agent instructions fetching (a26592f)
- ✨ Add **parallel subagent analysis** for processing large changesets concurrently
- ✨ Add **progressive diff analysis**—summaries by default, full diffs on request (f4bd00e)
- ✨ Add **automatic commit style detection** that mirrors repository format (gitmoji, conventional, plain) (05d26bc)
- ✨ Add **commit message history** preservation across regenerations with arrow key navigation (9becc01)
- ✨ Add **content update tools** (`UpdateCommitTool`, `UpdatePRTool`, `UpdateReviewTool`) for chat integration (f241f14)
- ✨ Add **VitePress documentation site** at hyperb1iss.github.io/git-iris with comprehensive guides (6069a03)
- ✨ Add **theme selector modal** with live preview and search filtering (bc4ae63)
- ✨ Add **settings modal** (`Shift+S`) for in-app provider, model, theme, and preset configuration (dee1161, 62e285b)
- ✨ Add **file staging controls** (`s`/`u` for individual, `a`/`U` for bulk) in Commit mode (04138723)
- ✨ Add **mouse interaction** with click-to-navigate, double-click, and drag selection (8566330)
- ✨ Add **visual selection mode** (`v` key) with multi-line copy support in code views (8566330)
- ✨ Add **clipboard support** (`y` key) with visual feedback across all modes (6d48879)

### Changed

- ♻️ Migrate from prompt-based generation to **agent-first architecture** with Iris making tool calls to gather context dynamically (dfbd9c5)
- ♻️ Replace `DynClientBuilder` with explicit provider dispatch for rig-core 0.27+ compatibility (dfbd9c5)
- ♻️ Switch Studio generation tasks from streaming to non-streaming execution for reliable structured output parsing (228afdab)
- ♻️ Reorganize codebase with new `src/agents/`, `src/studio/`, `src/theme/`, `src/companion/`, `src/services/`, and `src/types/` modules
- ♻️ Split monolithic Studio files into modular submodules for handlers, render, state, and reducer (f34bda3, faf7ab5)
- ♻️ Implement **reducer pattern** for predictable, testable state management in Studio (82ad0ac)
- ♻️ Separate emoji styling logic for commits vs non-commit outputs—`--gitmoji`/`--no-gitmoji` now work independently of presets (a6381c8)
- ♻️ Skip serializing default config values for cleaner `.irisconfig` files (dccd1ea)
- ♻️ Improve JSON extraction with sanitization and markdown fallback handling (101ac8a)
- ♻️ Improve file exclusion patterns with path boundary anchoring for precision (6b8e204)
- ♻️ Replace Python release script with declarative GitHub Actions workflow (e231ad8)
- ♻️ Use git tags instead of Cargo.toml for release version detection (6cf244b)
- ♻️ Make TUI startup non-blocking with async git status loading and companion initialization (f470abf)
- ♻️ Replace unbounded channels with bounded channels (capacity 100) for backpressure handling (b5656049)
- 📝 Streamline README to ~10% of original size with links to hosted documentation (d6a4a5e6)
- 📝 Update documentation for Studio TUI and agent architecture (2770e5d7, 104cbe96)

### Fixed

- 🐛 Fix **tab handling** in TUI with `expand_tabs` utility that converts tabs to spaces and strips control characters (cd12f2b)
- 🐛 Fix **UTF-8 string truncation** to use char boundaries instead of byte slicing (f4bd00e, cccfbcdb)
- 🐛 Fix **background task cleanup** on Studio exit by aborting tasks in Drop implementation (b398b2b)
- 🐛 Fix **RPM artifact upload path** for cross-compiled targets (68f517c, c8dbadd)
- 🐛 Fix **DEB package asset path** to include target directory (0eb4ebd)
- 🐛 Fix **theme tests** for parallel execution safety by removing global state assertions (be6476a)
- 🐛 Fix **state management** when opening settings modal and switching to Explore mode (5dd47cd)
- 🐛 Fix **Docker entrypoint** gitmoji handling to use boolean flags correctly (45df4e0)
- 🐛 Fix **scroll direction** in chat modal to match standard conventions (e5e11bb)
- 🐛 Fix **search modal** input handling for character input and backspace (e5e11bb)
- 🐛 Fix **custom instructions** (`--instructions` flag) propagation to agent (cd12f2b)
- 🐛 Fix **file tree path construction** using full_path components (cccfbcdb)
- 🐛 Fix **scroll bounds** to clamp offsets and prevent over-scrolling (cccfbcdb)
- 🐛 Fix **memory growth** in long sessions with bounded capacity limits for chat (500 messages), tool history (20 entries), and content versions (50 per mode) (ee86f6c)
- 🐛 Fix **panicking unwrap/expect calls** replaced with proper error handling in ParallelAnalyze, FileWatcherService, and GitRepo (ee4a7dfe)
- ⚡️ Pass release notes between workflows via artifact to eliminate redundant generation (1c76200)

### Removed

- 🔥 **Remove MCP server** (`git-iris serve` command and all MCP tooling in `src/mcp/`)—the agent architecture provides superior integration
- 🔥 **Remove legacy TUI** (`src/tui/`)—fully replaced by Iris Studio
- 🔥 **Remove file analyzers module** (`src/file_analyzers/`)—the LLM agent handles file analysis directly via tool calls
- 🔥 **Remove token optimizer** (`src/token_optimizer.rs`)—no longer needed with agent-based context management
- 🔥 **Remove old generation modules** (`src/changes/`, `src/commit/`)—replaced by unified agent capabilities
- 🔥 **Remove `src/llm.rs`**—provider logic moved to `src/agents/` and `src/providers.rs`
- 🔥 **Remove Python release script** (`scripts/release.py`)—replaced by GitHub Actions workflow
- 🔥 Remove unused dependencies: `once_cell`, `tiktoken-rs`, `tokio-retry`

### Breaking Changes

- **MCP server removed**: The `git-iris serve` command no longer exists. Users relying on MCP integration should migrate to the GitHub Action or direct CLI usage
- **Architecture overhaul**: Internal APIs have changed significantly. Extensions built against v1.x will need updates for the new agent-based system

### Metrics

- Total Commits: 100
- Files Changed: 356
- Insertions: +62,529
- Deletions: -16,771

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-04-17

### ✨ Added

- ✨ Support explicit version name for changelog and release notes with new --version-name option (67d9ead)
- ✨ Add Model Context Protocol (MCP) server integration with stdio and SSE transport options (22cabd4, 72c1651)
- ✨ Implement MCP tools for commit messages, code reviews, changelogs, and release notes (251b070, f64ba1f, 67d0b8d, 891c416)
- ✨ Add remote repository support for working with Git repos without manual checkout (3800d04)
- ✨ Add project configuration command for team-shared settings via .irisconfig files (873c63a)
- ✨ Add changelog file update functionality with --update and --file flags (0bc59d8)
- ✨ Add GenericTextAnalyzer for improved text file support (e2ecaca)
- ✨ Add quiet mode (--quiet/-q) and custom log file options (--log-file) (48db0ca)
- 🐳 Add Docker support with multi-stage build for containerized usage (3db4460)
- 🚀 Add publish workflow to Docker Hub and crates.io in CI/CD pipeline (37007eb)
- 📝 Add project-specific configuration documentation (43d6611)
- 📝 Add detailed changelog entries for versions 0.9.0 through 1.0.1 (46e4ad6)
- ♻️ Refactor git module into specialized submodules for better organization (62e698f)

### 🔄 Changed

- 🔧 Make repository parameter required in MCP tools for improved reliability (c875ece)
- 🔧 Reorganize Cargo.toml structure for better readability and discoverability (b33b61e)
- 📝 Improve package description and keywords in Cargo.toml (d77fd02)
- ♻️ Update RMCP dependency to use released version 0.1.5 instead of git dependency (2273811)

### 🐛 Fixed

- ⬆️ Update dependencies to their latest compatible versions (aff5be1)
- 🔄 Update default LLM models to latest versions (OpenAI gpt-4.1, Anthropic claude-3-7-sonnet-latest) (2a5baf9)

### 📊 Metrics

- Total Commits: 30
- Files Changed: 171
- Insertions: 10405
- Deletions: 2825

<!-- -------------------------------------------------------------- -->

## [1.0.1] - 2025-03-30

### ✨ Added

- ✨ Implement comprehensive 10-dimension code quality analysis system with severity levels, specific locations, detailed explanations, and actionable recommendations (0a29915)
- ✨ Add dedicated waiting messages with cosmic and analytical themes for code reviews (37c921a)
- 🔍 Create QualityDimension enum with new "Best Practices" dimension for centralized quality analysis (e75a648)
- 📝 Add comprehensive documentation for all 11 code quality dimensions (5d7d394)
- 💄 Enhance code review UI with modern styling, decorative Unicode characters, and improved readability (95cd3d5)

### 🔄 Changed

- ⚡️ Optimize regex patterns with Lazy static initialization, eliminating redundant compilations (08debd3)
- ♻️ Refactor code types into dedicated modules for better organization and maintainability (78c6cca)
- ♻️ Refactor JSON parsing with JsonSchema implementation for improved type safety and validation (e77a442)
- 🚀 Release version 1.0.1 (98978d2)

### 🗑️ Removed

- 🔥 Remove String conversion implementations for response types in favor of more robust approaches (e77a442)

### 📊 Metrics

- Total Commits: 9
- Files Changed: 47
- Insertions: 1941
- Deletions: 908

<!-- -------------------------------------------------------------- -->

## [1.0.0] - 2025-03-25

### ✨ Added

- ✨ Add AI-powered code review functionality with structured feedback for staged changes (76bdf31)
- ✨ Add preset type categorization (Commit, Review, Both) for command-specific instruction presets (b8bd6b4)
- 🔄 Migrate to external llm crate for standardized provider handling, supporting additional providers like Groq, XAI, DeepSeek, and Phind (0cbfc40)
- 🎨 Improve commit prompt formatting with statistical summary and better organization of file changes (6fc706a)
- 📝 Add comprehensive documentation for code review and changelog features (3fb5c28)
- 🛡️ Improve error handling with defensive programming patterns throughout the codebase (61cf6c7)
- ⬆️ Update dependencies to latest versions including git2, dirs, colored, rand, and ratatui (46fbe7b)
- ✨ Enhance Git hooks with improved execution environment and proper repository context (88f9f80)
- 💄 Enhance config command with beautifully formatted, colorized output (55bf071)
- 🔧 Modernize CI/CD pipeline with updated GitHub Actions (3388590)
- ⚡️ Set default max_tokens (4096) for LLM requests when not specified (4bb34b6)
- 📝 Update man page with comprehensive documentation for all commands and features (3a67fe9)
- 🎨 Improve CLI interface with better organization and styled provider list (023b8b7)
- 🔄 Add backward compatibility for Claude provider naming (claude → anthropic) (f657841)
- 📝 Add GitHub funding configuration (9098e9f)
- 🔧 Update Rust edition from 2021 to 2024 (c81cd1c)

### 🔄 Changed

- ♻️ Improve config display to preserve instruction formatting with line-by-line output (ff76709)
- 🔍️ Update review prompt to focus on staged changes rather than historical context (ee9de53)
- 🔄 Reorder instruction sections to place user instructions before preset instructions (e74ab66)
- ♻️ Rename LLM interface function from get_refined_message to get_message for simplicity (93abf18)
- 🎨 Reorganize import statements for consistent ordering across the codebase (d3799cb)

### 🐛 Fixed

- 🐛 Fix file content handling for deleted files in review and commit generation (f1d04aa)
- 🔧 Simplify token limit handling across providers for more consistent behavior (c6dbfd1)

### 📊 Metrics

- Total Commits: 27
- Files Changed: 122
- Insertions: 4217
- Deletions: 2083

<!-- -------------------------------------------------------------- -->

## [0.9.0] - 2025-02-24

### ✨ Added

- 🚀 Upgrade to Claude 3.7 Sonnet model with backward compatibility (e4e806c7)
- ✨ Add Python script (scripts/lint.py) to enhance Rust linting and code quality (f6ad5f0e)
- ⚡️ Improve token optimization efficiency with integration in commit service (4e893818)
- ✨ Add Conventional Commits preset to InstructionPresetLibrary (7507a413)
- 📝 Create CHANGELOG.md file to track project history (2cbc567f)
- ♻️ Implement GitRepo struct to encapsulate Git operations (c1f4e5b1)
- 🐛 Add early return for empty input text in apply_gradient function (f66e4ffd)
- 🐛 Improve robustness of parent commit handling in analyze_commit (d895bde1)
- 🚨 Enable Clippy lints for unwrap_used with TODOs for future fixes (ee65a1cc)
- 🚨 Add additional Clippy lints to improve code quality (32f3002f)

### 🔄 Changed

- 🔧 Fine-tune Clippy lint settings for better code clarity and standards (6283d48b)
- 🔧 Update Claude model from 'claude-3-5-sonnet-20240620' to 'claude-3-5-sonnet-20241022' (b4a45bc6)
- ⬆️ Upgrade GitHub Actions artifact handling to v4 (76fca7fa)
- ♻️ Refactor commit message generation process for better readability (e161211a)
- ✅ Replace unwrap() with expect() in test files for better error messages (5be93820)
- 🎨 Apply rustfmt to standardize code style across the project (62a043ed)
- ♻️ Refactor apply_gradient function for better readability (c0e5250a)
- 🔧 Update .gitignore to exclude log files (df9446c9)
- 📝 Update TODO list to reflect current project priorities (44582a00)

### 🐛 Fixed

- 🐛 Fix Clippy lints across multiple files (db008c9b)
- 🚨 Fix Clippy warnings in test files with improved error handling (2196464058)
- ✨ Improve issue and PR extraction with enhanced regex patterns (82b61d3e)

### 🗑️ Removed

- 🔥 Remove unused crates to streamline dependencies (f9fdb81d)

### 📊 Metrics

- Total Commits: 26
- Files Changed: 171
- Insertions: 2565
- Deletions: 1661

<!-- -------------------------------------------------------------- -->

## [0.7.0] -

### 🗑️ Removed

- 🔥 Remove tracking of unstaged files across multiple modules (db9db44)
- 🔥 Delete legacy interactive and old TUI commit modules (630aa21)

### ✨ Added

- ✨ Introduce cosmic-themed TUI for commit message creation (99c9428)
- ✨ Add support for pre and post commit hooks (43c8b56)
- ✨ Implement retry mechanism for LLM requests with exponential backoff (b798758)
- 🚀 Integrate Gitmoji support in TUI for commit messages (217ed78)
- 📝 Create TODO.md file with project roadmap and goals (3e18ffa)
- 🎨 Enhance instruction presets with emojis for visual appeal (7927873)

### 🐛 Fixed

- 🐛 Fix TUI message editing and rendering issues (538552f)
- 🐛 Correct binary file detection in git status parsing (a95c228)
- 🐛 Address CI/CD release issues and improve asset handling (da7b239)

### 🔄 Changed

- ♻️ Refactor project structure for improved modularity and maintainability (f1d60bf, e67206d, b48d37a)
- ⚡️ Optimize performance by parallelizing metadata extraction and caching git context (3a8163d, f1d60bf)
- 🔧 Update logging configuration for flexible log file paths and optional stdout logging (d738d89)
- 📝 Revise README to reflect new Git workflow focus and update project description (c404eb5)

### 📊 Metrics

- Total Commits: 70
- Files Changed: 257
- Insertions: 9691
- Deletions: 6079

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
