# Review Mode

**Review Mode** provides AI-powered code reviews analyzing security, performance, best practices, and potential bugs. Get detailed feedback before committing or creating PRs.

![Review Mode](/mode-review.png)

## When to Use Review Mode

- **Pre-commit quality checks**: Review changes before committing
- **PR preparation**: Catch issues before reviewers see them
- **Learning best practices**: Understand why certain patterns are recommended
- **Security audit**: Identify potential vulnerabilities
- **Performance analysis**: Find bottlenecks and inefficiencies

## Panel Layout

| Panel      | Content                                                                  |
| ---------- | ------------------------------------------------------------------------ |
| **Left**   | `Changed Files` tree of files in the selected ref range                  |
| **Center** | Structured markdown review: summary, coverage, and findings by severity  |
| **Right**  | Unified diff view with syntax highlighting and hunk navigation           |

### Left Panel: Changed Files

A `FileTree` titled `Changed Files`, listing each path touched between `from` and `to`. Ref selection itself happens in a modal (press <kbd>f</kbd> or <kbd>t</kbd>), not as inline UI in this panel.

### Center Panel: Review Output

Iris emits a **structured** `Review` (see `src/types/review.rs`), then renders it as markdown. Each finding carries a severity (`critical`, `high`, `medium`, `low`), a category (security, performance, error handling, complexity, abstraction, duplication, testing, style, API contract, concurrency, documentation, or other), and a confidence score from 0–100. The center panel scrolls through this rendered markdown.

### Right Panel: Diff View

- Unified diff for context
- Syntax-highlighted changes
- Hunk navigation
- Multi-file diff

## Essential Keybindings

### File List (Left Panel)

The `f`/`t` chords are handled when the **left panel** has focus — they open a ref selector modal scoped to Review's from/to refs.

| Key                            | Action                             |
| ------------------------------ | ---------------------------------- |
| <kbd>j</kbd> / <kbd>↓</kbd>    | Select next file                   |
| <kbd>k</kbd> / <kbd>↑</kbd>    | Select previous file               |
| <kbd>h</kbd> / <kbd>←</kbd>    | Collapse directory                 |
| <kbd>l</kbd> / <kbd>→</kbd>    | Expand directory                   |
| <kbd>Enter</kbd>               | Load file diff (focus right panel) |
| <kbd>f</kbd>                   | Open "from" ref selector modal     |
| <kbd>t</kbd>                   | Open "to" ref selector modal       |
| <kbd>r</kbd>                   | Generate review                    |
| <kbd>g</kbd> / <kbd>Home</kbd> | Jump to first file                 |
| <kbd>G</kbd> / <kbd>End</kbd>  | Jump to last file                  |

### Review Output (Center Panel)

| Key                                 | Action                   |
| ----------------------------------- | ------------------------ |
| <kbd>j</kbd> / <kbd>↓</kbd>         | Scroll down              |
| <kbd>k</kbd> / <kbd>↑</kbd>         | Scroll up                |
| <kbd>Ctrl+d</kbd> / <kbd>PgDn</kbd> | Page down                |
| <kbd>Ctrl+u</kbd> / <kbd>PgUp</kbd> | Page up                  |
| <kbd>g</kbd> / <kbd>Home</kbd>      | Jump to top              |
| <kbd>G</kbd> / <kbd>End</kbd>       | Jump to bottom           |
| <kbd>r</kbd>                        | Regenerate review (spawns a fresh `AgentTask::Review` for the current from/to refs — there is no incremental memory between runs) |
| <kbd>Shift+R</kbd>                  | Reset (clear review)     |
| <kbd>y</kbd>                        | Copy review to clipboard |

### Diff View (Right Panel)

| Key                                 | Action                |
| ----------------------------------- | --------------------- |
| <kbd>j</kbd> / <kbd>↓</kbd>         | Scroll down           |
| <kbd>k</kbd> / <kbd>↑</kbd>         | Scroll up             |
| <kbd>[</kbd>                        | Jump to previous hunk |
| <kbd>]</kbd>                        | Jump to next hunk     |
| <kbd>n</kbd>                        | Jump to next file     |
| <kbd>p</kbd>                        | Jump to previous file |
| <kbd>Ctrl+d</kbd> / <kbd>PgDn</kbd> | Page down             |
| <kbd>Ctrl+u</kbd> / <kbd>PgUp</kbd> | Page up               |

## Ref Selection

Press <kbd>f</kbd> or <kbd>t</kbd> to select **from/to refs**. The modal displays a filterable list of branches and tags with type-to-search functionality.

### Default Refs

- **From**: repository primary branch on feature branches
  On the primary branch itself, Review falls back to `HEAD~1`
- **To**: `HEAD` (current state)

### Common Ranges

| From                      | To       | Reviews                       |
| ------------------------- | -------- | ----------------------------- |
| `<default-branch>`        | `HEAD`   | All changes on current branch |
| `v1.0.0`                  | `v1.1.0` | Changes between releases      |
| `abc123f`                 | `HEAD`   | Changes since specific commit |
| `origin/<default-branch>` | `HEAD`   | Local changes not pushed      |

## Review Dimensions

Iris analyzes code across multiple dimensions:

### 🔒 Security

- SQL injection risks
- XSS vulnerabilities
- Authentication/authorization issues
- Cryptographic weaknesses
- Input validation gaps
- Secret exposure

### ⚡ Performance

- Algorithmic complexity (O(n²) loops, etc.)
- Memory leaks
- Inefficient queries
- Unnecessary allocations
- Hot path optimizations
- Caching opportunities

### ✨ Best Practices

- Code organization
- Naming conventions
- Error handling patterns
- Testing coverage
- Documentation quality
- SOLID principles

### 🐛 Potential Bugs

- Null/undefined dereferencing
- Off-by-one errors
- Race conditions
- Resource leaks
- Type mismatches
- Edge case handling

### 🧹 Code Quality

- Duplication (DRY violations)
- Complexity (cyclomatic, cognitive)
- Readability
- Maintainability
- Testability

### 📚 Documentation

- Missing docstrings
- Outdated comments
- Unclear naming
- API documentation
- README accuracy

## Review Output Format

Iris emits a structured `Review` (defined in `src/types/review.rs`) and renders it as markdown. Top-level sections always follow this order:

1. `# Code Review`
2. `## Summary` — the model's narrative paragraph
3. `## Review Coverage` — optional metadata: overall risk level, strategy, specialist passes, coverage notes
4. `## Findings` — a stats line followed by findings grouped under `### CRITICAL`, `### HIGH`, `### MEDIUM`, `### LOW`

Findings below the default confidence threshold (70%) are filtered out before display, so the stats line reports only the visible findings. When nothing remains, the section reads "No blocking issues found." instead of listing categories.

Each finding renders as:

```markdown
- [SEVERITY] **title in `file:line`**
  Category: <category>. Confidence: NN%.
  <body paragraph explaining the issue>
  **Fix**: <optional suggested fix>
  Evidence: file.rs:12, file.rs:30 (optional notes)
```

### Example

```markdown
# Code Review

## Summary

Reviewed 3 files with 145 additions and 32 deletions. The diff is mostly self-contained, but two changes need attention before merge.

## Review Coverage

Risk: high

Strategy: focused review of input validation and hot-path performance, with a security specialist pass on the message editor.

Specialist passes:
- security
- performance

## Findings

Reviewed 3 file(s). Found 2 issue(s): 0 critical, 1 high, 1 medium, 0 low.

### HIGH

- [HIGH] **Unvalidated input from message editor in `src/handlers/commit.rs:45`**
  Category: security. Confidence: 88%.
  The handler trusts every byte from the editor and forwards it straight into the commit pipeline. A pathological payload could exceed downstream buffers.
  **Fix**: Cap length and sanitize control characters before dispatch.
  Evidence: src/handlers/commit.rs:45, src/handlers/commit.rs:72

### MEDIUM

- [MEDIUM] **O(n²) lookup in commit indexing in `src/studio/state.rs:123-141`**
  Category: performance. Confidence: 76%.
  The nested loop scans every file for every commit, which scales poorly on large repos.
  **Fix**: Build a `HashMap<CommitId, Vec<FileId>>` once, then look up in O(1).
```

### Severity Indicators

The terminal renderer styles each severity badge instead of using check/warn/cross glyphs:

- `[CRITICAL]` and `[HIGH]` — error color, bold (the most urgent issues)
- `[MEDIUM]` — warning color, bold
- `[LOW]` — coral, bold

### Confidence Gating

The default cutoff is 70%, defined as `DEFAULT_MIN_FINDING_CONFIDENCE` in `src/types/review.rs`. Anything Iris is less than 70% sure about is dropped from the rendered view and from the stats counts — they exist in the structured payload but never reach the markdown the user sees.

## Workflow Examples

### Example 1: Pre-Commit Review

**Goal**: Check changes before committing

1. Make code changes
2. Switch to Review mode (<kbd>Shift+R</kbd>)
3. Default refs are `primary-branch..HEAD` on feature branches
   On the primary branch, Review falls back to `HEAD~1..HEAD`
4. Press <kbd>r</kbd> to generate review
5. Read the summary and walk the findings from CRITICAL down to LOW
6. Press <kbd>/</kbd> to chat: "Explain the O(n²) issue you found"
7. Fix issues in your editor
8. Press <kbd>r</kbd> to review again
9. When the findings list is empty (or shows "No blocking issues found."), switch to Commit mode (<kbd>Shift+C</kbd>)

### Example 2: PR Preparation

**Goal**: Get feedback before creating pull request

1. Finish feature branch
2. Switch to Review mode
3. Press <kbd>f</kbd> to select from ref: `origin/<default-branch>`
4. Press <kbd>t</kbd> to select to ref: `HEAD`
5. Press <kbd>r</kbd> to generate review
6. Address every `[CRITICAL]` and `[HIGH]` finding, then triage the `[MEDIUM]` and `[LOW]` ones
7. Press <kbd>y</kbd> to copy review to clipboard
8. Paste into PR description as "Self-Review" section

### Example 3: Release Audit

**Goal**: Review all changes between versions

1. Open Review mode
2. Press <kbd>f</kbd> → select `v1.0.0`
3. Press <kbd>t</kbd> → select `v1.1.0`
4. Press <kbd>r</kbd> to generate review
5. Focus on Security dimension (scroll to section)
6. Document any breaking changes found
7. Copy review with <kbd>y</kbd>
8. Use as release audit documentation

### Example 4: Learning from Reviews

**Goal**: Understand best practices by asking Iris

1. Generate review (<kbd>r</kbd>)
2. See warning: "Consider using iterators instead of for loops"
3. Press <kbd>/</kbd> to open chat
4. Ask: "Show me how to rewrite that loop with iterators"
5. Iris provides example code
6. Press <kbd>Shift+E</kbd> to switch to Explore mode
7. Navigate to the file
8. Press <kbd>w</kbd> on the loop to understand its history
9. Refactor based on learning

### Example 5: Security-Focused Review

**Goal**: Audit for security issues only

1. Generate review
2. Scan the Findings section for entries whose `Category:` line reads `security`
3. For each one:
   - Note the file and line range in the finding title
   - Press <kbd>Tab</kbd> to focus the diff panel
   - Navigate to that line with <kbd>j</kbd>/<kbd>k</kbd>
   - Press <kbd>/</kbd> to ask: "How would you exploit this?"
4. Fix vulnerabilities
5. Press <kbd>r</kbd> to regenerate the review against the updated tree

## Special Features

### Context-Aware Analysis

Iris reads project documentation to understand:

- **Architecture patterns**: From CLAUDE.md
- **Coding standards**: From README or CONTRIBUTING.md
- **Agent behavior**: From AGENTS.md

Reviews are customized to your project's standards.

### Diff-Integrated Review

Line number references in review correspond to diff view:

```
Review says:        Diff shows:
⚠️ Line 45          @@ -40,6 +40,10 @@
                     42  pub fn new() {
                     43    Self {
                     44      mode: Auto,
                     45      editing: false,  ← Line 45
```

Press <kbd>Tab</kbd> to jump between review and diff.

### Chat Integration

Ask follow-up questions about review findings:

```
You: Why is the O(n²) loop a problem here?

Iris: The nested loop in state.rs processes each commit
      against all files. For 100 commits × 500 files, that's
      50,000 iterations.

      With indexed lookup:
      1. Build HashMap<CommitId, Vec<FileId>>  // O(n)
      2. Look up files for each commit         // O(1)
      Total: O(n) instead of O(n²)

You: Show me the indexed version

Iris: [Provides code example]
```

### Re-running a Review

Each press of <kbd>r</kbd> spawns a brand-new `AgentTask::Review` for the current `from`/`to` refs (see `src/studio/handlers/mod.rs` and `handlers/review.rs`). There is no cross-run memory: Iris does not remember that you fixed an earlier issue, and there are no "quick" vs "deep" review modes — every run is a full re-analysis. To narrow the scope, change the refs (`f`/`t`) before regenerating.

## Tips & Tricks

### 1. Review Before Committing

Make it a habit:

1. Write code
2. <kbd>Shift+R</kbd> → Review
3. Fix issues
4. <kbd>Shift+C</kbd> → Commit

### 2. Use Chat for Explanations

Don't guess what warnings mean:

- See ⚠️ → Press <kbd>/</kbd> → Ask "Explain this warning"
- Iris provides detailed context and examples

### 3. Compare with Upstream

Before merging to your primary branch:

- From: `origin/<default-branch>`
- To: `HEAD`
- Reviews what will land in production

### 4. Save Reviews for PRs

Copy review (<kbd>y</kbd>) and paste into:

- PR description
- Commit message (for complex changes)
- Team wiki (as examples)

### 5. Filter by Category in Your Head

Findings carry an explicit `Category:` line (security, performance, error handling, etc.). Scan the rendered output and ignore categories that aren't your current focus — useful when you only have time to triage security or performance issues.

### 6. Iterative Improvement

Don't try to fix everything at once:

1. First pass: clear all `[CRITICAL]` and `[HIGH]` findings
2. Second pass: address `[MEDIUM]`
3. Third pass: polish `[LOW]` items where it's worth the effort

## Troubleshooting

### Review is empty

**Symptom**: Center panel shows "No review generated"

**Fix**:

1. Check that from/to refs are different
2. Ensure there are actual changes in range
3. Press <kbd>r</kbd> to manually trigger
4. Check status bar for errors

### Review takes too long

**Symptom**: Iris status shows "Thinking..." for >1 minute

**Cause**: Very large diff (1000+ lines)

**Fix**:

1. Narrow the ref range (fewer commits)
2. Review files individually (select in left panel)
3. Use chat instead: "Review the security of iris.rs"

### Line numbers don't match

**Symptom**: Review mentions line 45, but diff shows line 50

**Cause**: Line numbers are from **after** changes (in "to" ref)

**Fix**: Navigate diff to find context around that area.

### No security findings but I'm suspicious

**Symptom**: The findings section shows "No blocking issues found." (or no `security` category entries) but you're not convinced.

**Cause**: Confidence gating may have hidden a borderline finding (anything below 70% is filtered before display).

**Fix**:

1. Press <kbd>/</kbd> to open chat
2. Ask specific questions: "Could this be vulnerable to XSS?"
3. Request focused analysis: "Review line 45 for SQL injection"

## Next Steps

- Use review findings to improve [Commit Messages](commit.md)
- Combine with [Explore Mode](explore.md) to understand flagged code
- Generate [PR Descriptions](pr.md) that include review summary
- Learn [Chat](../chat.md) for detailed review discussions
