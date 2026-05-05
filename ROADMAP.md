# Iris Roadmap

> Plan of record for evolving Iris's agentic strategy, extraction stack, and code review pipeline. Synthesized from a deep multi-agent SOTA sweep (Claude Code, Cursor Composer, Aider, Codex CLI, CodeRabbit, Greptile, Bugbot, Cloudflare reviewer, Anthropic Code Review plugin, Gradial review-pr) plus a critical audit of the current codebase.

## Where We Are

Iris's loop is competent, single-shot, and has no critic. The capability TOMLs do most of the architectural work; the runtime around them is a thin Rig wrapper. The strategy is complete in shape (capabilities, tools, subagents, providers, streaming all wired) but thin in depth: one LLM pass per call, no verification, no per-finding structure, no cross-run memory, no permission surface, and tools that miss large categories of context modern agents take for granted (symbol resolution, blame, lint output, repo map). The single biggest gap by leverage is code review: the output is a `String`, which forecloses every downstream improvement (filtering, navigation, posting, telemetry).

What's already strong and worth preserving: capability TOMLs as the central intelligence, JSON extraction with provider-quirk recovery, mature `commit.toml` and `pr.toml`, profile-driven cascading (`MainAgent` / `Subagent` / `StatusMessage`), the recently hardened `project_docs(doc_type="context")` snapshot, and Studio's reducer-centric event flow.

## Phase Map

| Phase | Theme | Items | Approx. Size |
|---|---|---|---|
| **1** | Foundation cleanup + structured review | 1.1–1.6 | 1–2 weeks |
| **2** | Extraction stack | 2.1–2.6 | 2–3 weeks |
| **3** | Review differentiation | 3.1–3.6 | 3–4 weeks |
| **4** | Platform + calibration | 4.1–4.4 | open-ended |
| **5** | Optional / future tracks | 5.1+ | deferred |

Each item is named `<phase>.<n>` so the numbering survives reordering.

---

## Phase 1: Foundation Cleanup + Structured Review

The unlock phase. Item 1.1 (structured findings) blocks four downstream items, so it sequences first.

### 1.1 — Structured Review type [highest leverage]

Replace `MarkdownReview { content: String }` with a real type:

```rust
pub struct Review {
    pub findings: Vec<Finding>,
    pub summary: String,
    pub stats: ReviewStats,
}

pub struct Finding {
    pub id: FindingId,
    pub severity: Severity,            // Critical | High | Medium | Low
    pub confidence: u8,                // 0–100
    pub file: PathBuf,
    pub start_line: u32,
    pub end_line: u32,
    pub category: Category,            // Security | Perf | ErrorHandling | ...
    pub title: String,
    pub body: String,
    pub suggested_fix: Option<String>,
    pub evidence: Vec<EvidenceRef>,    // file:line or permalink
}
```

**Why**: every other review improvement (severity filter, per-finding chat, accept/dismiss telemetry, robust GitHub posting) depends on having parseable per-finding objects.

**Touch**: `src/types/review.rs`, `src/agents/capabilities/review.toml`, `src/studio/render/review.rs`, `src/github.rs` (replace regex extraction with iteration over `findings`), `src/agents/iris.rs` (`text_to_structured_response` and the `MarkdownReview` arm), `src/studio/state/modes.rs` (`ReviewState`).

### 1.2 — Critic stage

Add a `Critic` step after `Generation`/`Synthesis`. Cheap haiku/flash model takes the artifact + the diff and answers "what claims here aren't supported by the diff?" Re-prompt the main agent once if anything material is unsupported.

**Why**: every published benchmark says first-draft LLM output overclaims. Anthropic's plugin uses confidence ≥80 + an exclusion list; DeepSource hits 84.5% F1 vs CodeRabbit pure-LLM 36.19% by combining static signals with verification. We get most of that gain just by not trusting our own first draft.

**Touch**: `src/agents/capabilities/verify.toml` (new), `src/agents/iris.rs` (insert `Critic` phase between `Generation` and `Synthesis`, wire `IrisPhase::PlanExpansion` or rename it).

Applies to all capabilities, not just review. Default on; configurable off via `--no-critic`.

### 1.3 — Confidence threshold + "do not flag" list

In `review.toml`, default cutoff 70/100. Add the Anthropic exclusion list verbatim (pre-existing issues, lint-catchable, pedantic, lint-ignored). Filter pre-render in `src/studio/render/review.rs`.

**Why**: published numbers. The Lychee analysis of CodeRabbit found 28% noise/wrong; the threshold-plus-exclusion pattern is what separates Anthropic's plugin from open-the-firehose tools.

### 1.4 — GitHub permalink format on every finding

Resolve `https://github.com/{owner}/{repo}/blob/{full-sha}/{path}#L{start}-L{end}` from `git remote` + working SHA at render time. Pasteable everywhere; eliminates the fragile single-line regex in `src/github.rs:374-395`.

**Touch**: `src/github.rs`, `src/types/review.rs` (add `Permalink` helper).

### 1.5 — Prompt cache prefix lock

Audit Rig's prompt assembly. Stable byte order: preamble → capability TOML → project_docs → repo_map → diff context. For Anthropic, set `cache_control: ephemeral` on the static prefix block. For OpenAI, evaluate `previous_response_id` for chat continuity.

**Why**: Anthropic engineering blog on multi-agent: token usage explains 80% of performance variance and prompt caching is the architectural constraint, not an optimization. Cached reads are 10% of normal input price. This is a cost win with no quality risk.

**Touch**: `src/agents/iris.rs` (build_agent prompt assembly), `src/agents/provider.rs` (apply_completion_params).

### 1.6 — Cleanup: dead code and latent traps

- Remove `IrisPhase::PlanExpansion` (`src/agents/status.rs:29`) or wire it to the new Critic stage.
- Fold `AgentSetupService` into `IrisAgentService` (`src/agents/setup.rs`).
- Add the `GeneratedMessage` arm to `text_to_structured_response` (`src/agents/iris.rs:1167`).
- Set explicit `max_turns` in `SubagentRunner::run_task` (`src/agents/tools/parallel_analyze.rs:225`); current call to `agent.prompt(task)` does not guarantee the multi-turn tool contract that capability TOMLs assume.
- Surface `parallel_analyze` timeouts to the main agent as structured failures, not silent empty strings (`src/agents/tools/parallel_analyze.rs:397-403`).

**Why**: small bugs that block the next phase from building cleanly.

---

## Phase 2: Extraction Stack

Tools that change what Iris can know. Phase 1 makes review usable; Phase 2 makes every capability smarter.

### 2.1 — Repo map tool (Aider-style)

```rust
repo_map(token_budget: u32, mentioned_files: Vec<PathBuf>) -> RepoMap
```

Tree-sitter `tags.scm` queries extract definition vs reference tags per file. Build a directed file-to-file reference graph. NetworkX-style PageRank with personalization for the current chat context. Edge weight multipliers: 10x for mentioned identifiers, 10x for well-named identifiers, 50x for files in chat. Output sub-2K tokens for a typical repo.

**Why**: Aider's repo map is the canonical solution for "fit a whole codebase summary in 2K tokens." Beats grep+read 5–34x on cross-file queries (per `agent-lsp` benchmark). Gives Iris a semantic skeleton instead of needing blind `code_search` to find structure.

**Touch**: `src/agents/tools/repo_map.rs` (new), workspace deps for `tree-sitter`, `tree-sitter-rust`, `tree-sitter-typescript`, `tree-sitter-python`, etc.

### 2.2 — `git_blame` + author-history retrieval

`git_blame(file, line_range)` tool plus capability-side use: when generating a commit message that touches `foo.rs`, prepend the last 3 commits that touched `foo.rs` as in-context style anchors.

**Why**: HAFixAgent (arxiv 2511.01047) lifts repair quality by injecting blame context; same trick applies to commit and PR generation. Cheap, very git-iris specific, big perceived-quality jump.

**Touch**: `src/agents/tools/git.rs` (add blame), `src/agents/capabilities/commit.toml` and `pr.toml` (use blame for style anchors).

### 2.3 — Static analysis tool

`src/agents/tools/static_analysis.rs` wrapping the appropriate linter for the language: `clippy` (Rust), `ruff` (Python), `oxlint`/`biome` (JS/TS), `golangci-lint` (Go). Inject results as a callable tool. Update `review.toml`: "if a linter would catch this, do not flag; do prioritize what the linter found."

**Why**: DeepSource's hybrid approach (5,000+ static rules + LLM agent) hit 84.5% F1 on the OpenSSF CVE benchmark vs CodeRabbit's pure-LLM 36.19% (2.3x precision lift). This is the highest-precision review move with real published numbers.

### 2.4 — Per-tool render contract (cc-open pattern)

Every tool implements:

```rust
trait ToolRender {
    fn render_in_progress(&self) -> RenderedCell;
    fn render_result_full(&self) -> RenderedCell;
    fn render_result_collapsed(&self) -> RenderedCell;
    fn is_search_or_read(&self) -> bool { false }
}
```

Studio auto-collapses search/read results that are no longer the focus.

**Why**: cc-open's `Tool.ts:362` surface gives each tool ownership of its display in all four states. Currently every tool call surfaces identically in Studio (spinner, then text). Verbose tools like `file_read` and `code_search` benefit most.

**Touch**: tool trait in `src/agents/tools/mod.rs`, renderers in `src/studio/render/`.

### 2.5 — Source-retaining stream (codex-rs pattern)

Stream controller keeps `raw_source: String` alongside `rendered_lines`. On terminal resize, re-render from source rather than reflowing rendered lines. Committed/in-flight cell split: same struct represents both, with an `active_cell_revision` counter driving cache invalidation.

**Why**: fixes resize jitter and unlocks correct scroll-anchor behavior during streaming. Cited file: `~/dev/codex/codex-rs/tui/src/streaming/controller.rs`.

**Touch**: `src/studio/app.rs`, `src/studio/render/`.

### 2.6 — Project profile bootstrap

First time `git-iris` runs against a repo (or on `git-iris profile --bootstrap`), Iris spends 30–60 seconds analyzing the project and writes `.git-iris/project.md` covering:

- **Stack & conventions**: detected languages, frameworks, package managers, formatters/linters, test frameworks, build/test commands
- **Commit style**: patterns observed in `git log` (conventional / gitmoji / plain / ticket prefixes), typical body length, scope conventions, emoji vocabulary if any
- **Hot domains**: top 10 most-touched modules with dominant author per module (uses 2.2 blame)
- **Repo skeleton**: key entry points, public API surface, integration boundaries (uses 2.1 repo map)
- **Naming conventions**: file naming, identifier conventions, test file patterns
- **Workflow signals**: branch naming patterns, PR title patterns, release cadence

Loaded as part of the cached prefix on every capability run (slots between `project_docs` and `repo_map` in the 1.5 prefix lock order). Stale-aware: a `last_refreshed` timestamp + commit-density tracker triggers a "profile feels stale" nudge after large structural shifts (significant module additions, dependency changes, paradigm shifts).

User-readable and hand-editable: it's a markdown file in the repo's working tree. Iris updates it; humans can override. Conflicts resolve in favor of the human edit with a comment from Iris explaining what she would have written.

**Why**: replaces "agent re-discovers conventions on every call" with "agent looks them up." Cheaper per-call, more consistent across capabilities, and gives users a readable artifact they can shape. Cursor (`.cursorrules`), Claude Code (`CLAUDE.md`), Aider (`CONVENTIONS.md`) all rely on hand-written equivalents; Iris auto-generates the first draft and refines it from observed behavior.

**Touch**: new `src/agents/capabilities/profile.toml`, new `src/agents/profile.rs`, integration into 1.5 cache prefix order, surface in `src/cli.rs` as `git-iris profile [--bootstrap | --refresh | --show]`.

**Sequencing**: depends on 2.1 (repo map) and benefits from 2.2 (blame). Ship after both.

---

## Phase 3: Review Differentiation

Things competitors don't ship. This is where Iris becomes distinctive.

### 3.1 — Risk-tier dispatch

Classify the diff Critical/High/Medium/Low based on path patterns and size. Calibrate critic depth, parallel_analyze fan-out, and reasoning effort accordingly.

| Tier | Triggers | Fan-out |
|---|---|---|
| **Critical** | auth, RLS, secrets, payments, user input | 4–7 specialists, full critic, --rigor=high optional |
| **High** | migrations, API contracts, infra, agents | 3–4 specialists, full critic |
| **Medium** | business logic, UI with state, workflows | single pass + critic |
| **Low** | docs, config, styling, tests-only | single pass, no critic |

**Why**: Cloudflare's published numbers show tier-driven dispatch dropped median cost to $0.98/review across 131k runs while improving signal. Pulled from Gradial's `review-pr` skill (`.claude/skills/review-pr/references/review-prompt.md`).

**Touch**: `src/agents/capabilities/review.toml`, `src/agents/iris.rs` (route by tier), config schema for path-pattern tiers.

### 3.2 — Specialist sub-capabilities

For High/Critical-tier diffs, dispatch in parallel:

- `review_security.toml` — auth, RLS, secrets, input validation, OWASP
- `review_perf.toml` — algorithmic complexity, blocking ops, resource leaks
- `review_concurrency.toml` — async, locks, race conditions, lifetimes
- `review_tests.toml` — coverage gaps, brittle patterns, eval gates
- `review_api.toml` — contract changes, breaking changes, versioning

Each is a focused TOML with its own tool guidance and outputs `Vec<Finding>`. Lead Iris consolidates and dedups via LLM-as-judge.

**Why**: this is the gradial pattern (domain agents + cross-cutting layers) and the Cloudflare/Anthropic specialist pattern simultaneously. The codex-rs multi-agent v2 surface (`spawn`, `wait`, `send_message`, `close_agent` at `core/src/tools/handlers/multi_agents_v2/`) is the right primitive shape if/when we want an in-process registry instead of the simpler `parallel_analyze` fan-out.

**Touch**: `src/agents/capabilities/review_*.toml` (new), `src/agents/iris.rs` (specialist dispatch logic).

### 3.3 — HTML explainer artifact

On review completion, optional `e` keybind in Studio (or `--html` flag in CLI) exports a self-contained HTML to `pr-{n}-review.html` or `review-local-{branch}.html`. Sidebar nav, verdict card, mermaid architecture diff for the changed graph, per-file table, expandable findings, code-deep-dive snippets.

**Why**: Gradial's `review-pr` skill ships a production-grade design system at `.claude/skills/review-pr/references/html-explainer-guide.md`. Re-skin to SilkCircuit, integrate. Nobody else ships TUI-native review with a published HTML artifact.

**Touch**: `src/studio/handlers/review.rs` (export keybind), new `src/output/html_explainer.rs`, embed CSS as a `const` string.

### 3.4 — Per-finding chat-with-reviewer

Studio chat already exists. Add `StudioEvent::ChatAboutFinding { finding_id }`. Reducer prepends finding context (file, line range, current diff hunk, finding body) to the chat preamble.

**Why**: mirrors CodeRabbit `@coderabbitai` but TUI-native. No other AI-TUI tool ships per-finding contextual chat.

**Touch**: `src/studio/events.rs`, `src/studio/reducer.rs`, `src/studio/handlers/review.rs`, `src/studio/render/chat.rs`.

### 3.5 — Persistent memory: notes + playbook (ACE-lite)

```
.git-iris/
├── notes.md          # Iris's workspace notes, scoped to repo, persisted
├── playbook.md       # Per-capability "what worked / what didn't" deltas
└── learnings/        # User-confirmed patterns (CodeRabbit style)
```

Capability runs append deltas to `playbook.md`. The next run reads the relevant section. User can mark a learning durable via chat ("remember this") which writes to `learnings/`.

**Why**: ACE (arxiv 2510.04618, ICLR 2026) shows playbook-style evolving contexts beat monolithic rewrites by 10.6% on agent tasks with no labeled supervision. CodeRabbit's stale-learning surfacing (use counters) prevents drift.

**Touch**: new `src/agents/memory/` module on top of the existing `src/companion/storage.rs`, `src/companion/session.rs`, and `src/companion/branch_memory.rs` (already-built persistence and session-state layer; repurpose as the substrate for agentic memory rather than letting it sit dormant under "companion mode" framing).

### 3.6 — Memory curator: per-project refinement

Periodic LLM-driven curation of `.git-iris/notes.md`, `playbook.md`, `learnings/`, and `project.md`. The curator's job is to keep accumulated memory from rotting into noise.

**What the curator does:**

- **Consolidate** — collapse duplicate notes (same insight, different wording) into one canonical entry
- **Decay** — drop entries unused for N weeks unless explicitly pinned by the user
- **Promote** — move repeatedly-applied notes from `notes.md` into `playbook.md`; promote durable playbook entries into `learnings/`
- **Surface conflicts** — flag notes that contradict each other or conflict with current `project.md` (e.g., note says "team uses pnpm" but profile detects yarn)
- **Scope check** — verify each note still applies (file still exists, function still named the same, dependency still in the lock file). Stale-scoped notes get archived rather than silently misleading future runs.
- **Refresh `project.md`** — same pass that curates notes can update detected conventions if commit-history signals shifted

**Triggers:**

- On-demand: `git-iris memory refine`
- Automatic: after N capability runs (default 25), or weekly if `git-iris` is invoked at all in that period
- Proactive: when the curator observes a contradiction during a normal capability run, queue a refinement

**Per-project scope** is non-negotiable. Memory from one repo never bleeds into another. The curator runs against `.git-iris/` only.

**Why**: ACE's curator role (arxiv 2510.04618) and CodeRabbit's stale-learning surfacing with usage counters both prove the same point — without curation, accumulated memory becomes noise that *reduces* quality instead of improving it. The memory layer is only as good as the curation layer.

**Touch**: new `src/agents/memory/curator.rs`, new `src/agents/capabilities/refine_memory.toml`, scheduling hook in `src/cli.rs` (run counter + last-curate timestamp), surface in `git-iris stats` so users can see what was kept vs dropped on the last pass.

---

## Phase 4: Platform + Calibration

Things that turn Iris into a platform. Open-ended; pick when ready.

### 4.1 — BM25 deferred tool loading

```rust
tool_search(query: &str) -> Vec<ToolSpec>
```

As the toolbelt grows past ~12 tools (we're at 9), loading every schema on every call is wasteful. Iris calls `tool_search("find function references")` and pulls only matched specs. The `bm25` crate is pure Rust.

**Why**: codex-rs ships this at `core/src/tools/handlers/tool_search.rs`. Becomes essential when MCP integration arrives.

### 4.2 — Cross-model adversarial review

`--rigor=high` runs review with both Anthropic and OpenAI. Present agreement clusters. Disagreements get auto-routed to a tiebreaker model.

**Why**: SWR-Bench (arxiv 2509.01494) shows n=10 self-aggregation lifts F1 +43%, recall +118% over single-pass. We're at n=2 with strong models, so the curve flattens but the precision win is real for security-critical reviews.

**Touch**: `src/agents/iris.rs` (orchestrator), `src/cli.rs` (`--rigor`).

### 4.3 — Resolution-rate telemetry

Studio tracks accept/dismiss/edit per finding, local-only and opt-in. Persisted to `.git-iris/telemetry.jsonl`. Surfaces in `git-iris stats`. Feeds offline calibration of severity prompts.

**Why**: Cursor Bugbot tunes everything against this metric (52% → >70% resolution). Bugbot's case study explicitly identifies it as the killer ground truth that beats every LLM-self-reported quality signal.

**Touch**: new `src/telemetry.rs`, `src/studio/state/modes.rs` (capture finding interactions).

### 4.4 — Permission hooks + skills loading

Local rules in `.git-iris/permissions.toml` for tool-use gating (always allow `git diff`, deny `git push`, etc.) and `.git-iris/skills/*.toml` for user-supplied capabilities. Codex-rs's `run_permission_request_hooks` (`core/src/tools/orchestrator.rs:400`) is the cleanest model.

**Why**: makes Iris extensible without forking. Mirrors the externally-published git-iris Claude Code skill but pointing inward.

**Touch**: `src/agents/permissions.rs` (new), `src/agents/iris.rs` (capability loader extends to skills dir).

---

## Phase 5: Optional / Future Tracks

Not on the active path. Captured here so the design thinking isn't lost and so an outside contributor can pick one up cleanly.

### 5.1 — Ambient companion mode (deferred)

The original "Iris Companion" vision was a persistent daemon with file watchers, working-tree timeline, branch context greetings, upstream tracking, pre-commit guardian, desktop notifications, and pattern learning. The scaffold lives in `src/companion/` (`watcher.rs`, `session.rs`, `branch_memory.rs`, `storage.rs`).

**Why deferred**: it's a different product shape from "Iris the agentic CLI/TUI you invoke." Adjacent to git-iris's core strength rather than amplifying it. The maintainer hasn't used it; the demand is hypothetical.

**What we keep from it**: the persistence layer (`session.rs`, `branch_memory.rs`, `storage.rs`) becomes the Phase 3.5 memory substrate. Files keep their location and stay compiled — they just stop being framed as "companion mode" and start being the memory backend.

**What gets dropped from the roadmap**: file watcher integration as a foreground feature, daemon-mode CLI commands (`git-iris watch`, `git-iris status`, `git-iris stop`), desktop notifications, ambient insight engine. The `watcher.rs` module is fine to keep as dead-but-harmless code; an outside contributor wanting to revive ambient mode has a working starting point.

**If someone wants to ship it**: the natural next steps are (a) a `Daemon` mode in `src/cli.rs` that runs the watcher, (b) wiring `CompanionEvent` into a Studio sidebar panel via a new `StudioEvent::CompanionInsight`, and (c) a rules-first insight engine (no LLM) in `src/companion/insights.rs` that fires on event bus signals like "30 minutes of edits without a commit." LLM analysis can layer on top.

### 5.2 — Multi-agent decision-making with mailboxes

Codex-rs ships `AgentRegistry`, `Mailbox`, `SpawnReservation` (`~/dev/codex/codex-rs/core/src/agent/`) for spawnable named subagents that pass messages and trigger turns in each other. Impressive, more than git-iris needs today. Revisit if Phase 3.2 specialist sub-capabilities reveal a real need for inter-agent state beyond `parallel_analyze`.

### 5.3 — Self-modifying capability TOMLs (ACE in full)

ACE's full pattern (reflector + curator + delta updates) goes beyond Phase 3.5's playbook. Iris would propose edits to her own capability TOMLs based on user accept/reject feedback. Revisit when Phase 4.3 telemetry has six months of data showing prompt drift hurts quality.

---

## Non-Goals

Explicit decisions to not chase, with reasoning, so we don't drift:

- **Decision-making subagents with mailboxes**. Cognition's "Don't Build Multi-Agents" applies. `parallel_analyze` stays read-only and answers questions, not making decisions. The codex-rs mailbox surface is impressive but more than we need (see 5.2 for the deferred-not-rejected version).
- **SWE-RL training pipelines**. No infra ROI for git-iris's domain. Borrow the rule-based-reward concept (offline eval scoring generated commits against historical real commits) without the training loop.
- **Embedding-based RAG over the whole codebase**. Every signal in the research says graph + tree-sitter + LSP beats embeddings for code. We don't have either yet, so we leapfrog directly to the better stack.
- **Vector-DB-backed memory**. The Anthropic engineering blog and Claude Code reverse-engineering both endorse file-based markdown over vectors for short-to-medium horizon. ACE's playbook beats embeddings on agent tasks.
- **Self-modifying agent (Live-SWE-agent style)**. Lift-cost vs current return is wrong for git-iris. Revisit if Phase 4.3 telemetry suggests prompt drift is actually hurting us.

---

## Sequencing Notes

- **1.1 blocks 1.2, 1.3, 1.4, 3.4**: nothing to verify, filter, post, or chat about until findings are structured objects.
- **1.6 blocks Phase 2**: `parallel_analyze` single-turn fix is required before specialist sub-capabilities (3.2) can rely on parallel fan-out.
- **2.1 blocks 3.2**: specialists need a repo map to scope their work without burning context on file discovery.
- **2.1 + 2.2 block 2.6**: project profile bootstrap consumes both repo map and blame.
- **2.6 feeds 1.5**: once profile exists, it slots into the cached prefix between `project_docs` and `repo_map`.
- **3.5 blocks 3.6**: nothing to curate until something gets written.
- **3.5 blocks 4.3**: telemetry needs a stable persistence story to live in.
- **Phase 1 cleanup (1.6) and Phase 2.5 (source-retaining stream)** can ship as low-risk side PRs at any time.

---

## File Touchpoint Index

For quick navigation when picking up a phase item.

| Item | Primary file(s) |
|---|---|
| 1.1 | `src/types/review.rs`, `src/agents/capabilities/review.toml`, `src/studio/render/review.rs`, `src/github.rs` |
| 1.2 | `src/agents/capabilities/verify.toml` (new), `src/agents/iris.rs` |
| 1.3 | `src/agents/capabilities/review.toml`, `src/studio/render/review.rs` |
| 1.4 | `src/github.rs`, `src/types/review.rs` |
| 1.5 | `src/agents/iris.rs`, `src/agents/provider.rs` |
| 1.6 | `src/agents/status.rs`, `src/agents/setup.rs`, `src/agents/iris.rs`, `src/agents/tools/parallel_analyze.rs` |
| 2.1 | `src/agents/tools/repo_map.rs` (new), `Cargo.toml` |
| 2.2 | `src/agents/tools/git.rs`, `src/agents/capabilities/commit.toml`, `src/agents/capabilities/pr.toml` |
| 2.3 | `src/agents/tools/static_analysis.rs` (new), `src/agents/capabilities/review.toml` |
| 2.4 | `src/agents/tools/mod.rs`, `src/studio/render/` |
| 2.5 | `src/studio/app.rs`, `src/studio/render/` |
| 2.6 | `src/agents/capabilities/profile.toml` (new), `src/agents/profile.rs` (new), `src/cli.rs`, integrates into 1.5 cache prefix |
| 3.1 | `src/agents/capabilities/review.toml`, `src/agents/iris.rs`, config schema |
| 3.2 | `src/agents/capabilities/review_*.toml` (new), `src/agents/iris.rs` |
| 3.3 | `src/output/html_explainer.rs` (new), `src/studio/handlers/review.rs` |
| 3.4 | `src/studio/events.rs`, `src/studio/reducer.rs`, `src/studio/handlers/review.rs` |
| 3.5 | `src/agents/memory/` (new), `src/companion/storage.rs`, `src/companion/session.rs`, `src/companion/branch_memory.rs` (repurpose as memory substrate) |
| 3.6 | `src/agents/memory/curator.rs` (new), `src/agents/capabilities/refine_memory.toml` (new), `src/cli.rs`, `git-iris stats` |
| 4.1 | `src/agents/tools/tool_search.rs` (new), `src/agents/iris.rs` |
| 4.2 | `src/agents/iris.rs`, `src/cli.rs` |
| 4.3 | `src/telemetry.rs` (new), `src/studio/state/modes.rs` |
| 4.4 | `src/agents/permissions.rs` (new), `src/agents/iris.rs` |

---

## Research Index

The synthesis behind this roadmap pulls from:

- **Iris codebase audit**: agent loop, tools/extraction, code review pipeline (three parallel exploration agents on the current repo)
- **SOTA agentic coding**: Claude Code (Dive-into-Claude-Code reverse engineering), Cursor Composer, Aider repo map, Codex CLI, Devin/Cognition, Sourcegraph Cody/Amp, Augment, Replit Agent, Live-SWE-agent
- **SOTA review tools**: CodeRabbit, GitHub Copilot Code Review, Greptile v3, Cursor Bugbot, Codium PR-Agent / Qodo Merge, Anthropic Code Review plugin, Cloudflare orchestrator, Graphite Diamond, Macroscope v3, DeepSource, SWR-Bench
- **Reference implementations**: `~/dev/cc-open` (TS Claude Code clone), `~/dev/codex/codex-rs` (Rust Codex CLI)
- **Internal patterns**: `~/dev/v2/.claude/skills/review-pr` (Gradial's risk-tier multi-agent review skill with HTML explainer)
- **Papers (late 2025 / early 2026)**: ACE (2510.04618), HAFixAgent (2511.01047), SWE-RL (2502.18449), Live-SWE-agent (2511.13646), Self-Improving Coding Agent (2504.15228), LSPRAG (2510.22210), Agentic Harness Engineering (2604.25850), OpenDev (2603.05344), SWR-Bench (2509.01494)

Sibyl knowledge captured: `pattern_6340b3582a9e` (review type bottleneck), `pattern_0e1aec8bb01d` (DeepSource F1 numbers), `pattern_668d68a04711` (Aider repo map), `pattern_44e89fa6b499` (parallel_analyze single-turn).
