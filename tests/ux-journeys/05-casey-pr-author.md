# Journey 05: Casey - The PR Author

> *"Feature's done! Time to write a killer PR description"*

## Persona Profile

- **Name**: Casey Williams
- **Role**: Full-Stack Developer
- **Experience**: 4 years, shipped many features
- **Goal**: Create a compelling PR description
- **Vibe**: Creative, detail-oriented, proud of their work

## Pre-Journey Setup

```bash
# For this test, we'll pretend main has some commits ahead
# The PR mode will compare current branch to base branch

# Check our current state
git branch --show-current
git log --oneline main..HEAD  # or origin/main..HEAD

# Our recent commits ARE the "feature" - perfect for testing!
```

## Journey Script

### Step 1: Launch Studio
```
ACTION: cargo run -- studio
WAIT: 5 seconds for startup
SCREENSHOT: journey-casey-01-launch.png
```

### Step 2: Switch to PR Mode
```
ACTION: Press Shift+P (uppercase P)
WAIT: 1 second
VERIFY: Mode indicator shows "P PR"
VERIFY: Shows base branch selector area
SCREENSHOT: journey-casey-02-pr-mode.png
```

### Step 3: Select Base Branch
```
ACTION: Press 'f' to select from/base branch
WAIT: 1 second
VERIFY: Branch/ref selector opens
SCREENSHOT: journey-casey-03-base-selector.png
```

### Step 4: Choose origin/main as Base
```
ACTION: Type "origin" to filter (if search works)
   OR: Navigate with j/k to find origin/main or dfbd9c5
ACTION: Press Enter to select
WAIT: 0.5 seconds
VERIFY: Base branch updated
SCREENSHOT: journey-casey-04-base-selected.png
```

### Step 5: Generate PR Description
```
ACTION: Press 'r' to generate PR description
WAIT: 30 seconds for AI generation
VERIFY: PR description appears in center panel
VERIFY: Has title suggestion
VERIFY: Has summary section
VERIFY: Has "Changes" or similar breakdown
VERIFY: Has "Testing" section
VERIFY: NO JSON artifacts!
SCREENSHOT: journey-casey-05-pr-generated.png
```

### Step 6: Scroll Through PR
```
ACTION: Press 'j' 5 times to scroll
WAIT: 0.2 seconds between
SCREENSHOT: journey-casey-06-pr-scrolled.png
```

### Step 7: Ask Iris to Enhance
```
ACTION: Press '/' to open chat
WAIT: 0.5 seconds
ACTION: Type "Make the PR description more exciting! Add some personality and highlight the user impact"
ACTION: Press Enter
WAIT: 20 seconds for response
VERIFY: Iris suggests enhanced version
SCREENSHOT: journey-casey-07-chat-enhance.png
```

### Step 8: Request Specific Change
```
ACTION: Type "Also add a 'Screenshots' section placeholder and mention this fixes the streaming bug"
ACTION: Press Enter
WAIT: 15 seconds
SCREENSHOT: journey-casey-08-chat-refine.png
```

### Step 9: Apply Changes (if update tool works)
```
# If Iris offers to update the PR directly:
VERIFY: Check if content updates in PR panel
SCREENSHOT: journey-casey-09-updated.png
```

### Step 10: Copy and Exit
```
ACTION: Press Escape to close chat
ACTION: Press 'y' to copy PR description
VERIFY: "Copied to clipboard" notification
ACTION: Press 'q' to quit
```

## Expected PR Content

Should include something like:

```markdown
## Summary

This PR refactors the Studio generation pipeline to use non-streaming
execution for structured outputs, fixing issues where raw LLM artifacts
were appearing in the UI.

## Changes

- **Streaming Refactor**: Review, PR, Changelog, and Release Notes now
  use `execute_task()` instead of streaming
- **State Management**: Fixed modal rendering and explore mode initialization
- **Provider Dispatch**: Cleaner provider abstraction

## Testing

- [ ] Verified changelog generation shows clean markdown
- [ ] Verified review mode produces formatted output
- [ ] Tested mode switching with Settings modal

## Screenshots

<!-- Add screenshots here -->
```

## Success Criteria

- [ ] PR mode accessible via Shift+P
- [ ] Base branch selector works
- [ ] Can select origin/main or specific commit
- [ ] PR description generates successfully
- [ ] Output is well-formatted markdown
- [ ] NO JSON wrappers in output
- [ ] Chat can enhance the PR
- [ ] Copy to clipboard works

## Key UX Test Points

1. **Base Branch Selection** - Is it clear how to select target?
2. **Commit Range** - Does it show what's being compared?
3. **PR Quality** - Is the generated PR actually useful?
4. **Chat Enhancement** - Can Iris make it better?
5. **Update Flow** - Can chat changes apply to PR?

## Fun PR Enhancement Prompts to Try

- "Add a haiku about this PR"
- "Make it sound like a movie trailer"
- "Add emoji section headers"
- "Write the summary as if explaining to a rubber duck"
- "Include a 'Risk Assessment' section"
