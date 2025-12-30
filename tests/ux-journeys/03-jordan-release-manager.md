# Journey 03: Jordan - The Release Manager

> *"Time to cut v2.0! Need changelog and release notes ASAP"*

## Persona Profile

- **Name**: Jordan Park
- **Role**: Engineering Manager / Release Coordinator
- **Experience**: 12 years, leads the platform team
- **Goal**: Generate professional release documentation
- **Vibe**: Organized, deadline-driven, quality-focused

## Pre-Journey Setup

```bash
# Check available tags/refs for range selection
git tag --list | tail -5
git log --oneline -15

# We'll use: dfbd9c5 (origin/main) -> HEAD for the range
# This captures our recent bug fixes and refactors
```

## Journey Script

### Step 1: Launch Studio
```
ACTION: cargo run -- studio
WAIT: 5 seconds for startup
SCREENSHOT: journey-jordan-01-launch.png
```

### Step 2: Switch to Changelog Mode
```
ACTION: Press Shift+L (uppercase L)
WAIT: 1 second
VERIFY: Mode indicator shows "L Changelog"
VERIFY: Shows commit list panel
SCREENSHOT: journey-jordan-02-changelog-mode.png
```

### Step 3: Select From Reference
```
ACTION: Press 'f' to open from-ref selector
WAIT: 1 second
VERIFY: Ref selector modal opens with commits
SCREENSHOT: journey-jordan-03-from-selector.png
```

### Step 4: Pick the Starting Point
```
ACTION: Press 'j' four times (navigate to dfbd9c5 - origin/main)
WAIT: 0.2 seconds between presses
ACTION: Press Enter to select
WAIT: 0.5 seconds
VERIFY: From ref updated in left panel header
SCREENSHOT: journey-jordan-04-from-selected.png
```

### Step 5: Verify To Reference (HEAD)
```
# To-ref should default to HEAD - perfect for "unreleased" changelog
VERIFY: Range shows something like "dfbd9c5 -> HEAD"
```

### Step 6: Generate Changelog
```
ACTION: Press 'r' to generate changelog
WAIT: 30 seconds for AI generation
VERIFY: Changelog appears in center panel
VERIFY: Format follows Keep a Changelog style
VERIFY: Shows [Unreleased] - YYYY-MM-DD header
VERIFY: Categories: Added, Changed, Fixed, etc.
VERIFY: NO JSON wrappers or {"content": ...} artifacts!
SCREENSHOT: journey-jordan-05-changelog-generated.png
```

### Step 7: Scroll and Review
```
ACTION: Press 'j' 3 times to scroll content
WAIT: 0.2 seconds between
SCREENSHOT: journey-jordan-06-changelog-scrolled.png
```

### Step 8: Copy Changelog
```
ACTION: Press 'y' to copy
VERIFY: "Copied to clipboard" notification
```

### Step 9: Switch to Release Notes Mode
```
ACTION: Press Shift+N (uppercase N)
WAIT: 1 second
VERIFY: Mode indicator shows "N Release"
SCREENSHOT: journey-jordan-07-release-mode.png
```

### Step 10: Verify Range Persists
```
# Range should carry over from Changelog mode
VERIFY: Same from/to refs shown
# If not, repeat ref selection (f, navigate, Enter)
```

### Step 11: Generate Release Notes
```
ACTION: Press 'r' to generate release notes
WAIT: 30 seconds for AI generation
VERIFY: Release notes appear (different format than changelog)
VERIFY: More narrative, user-focused content
VERIFY: Sections like "Highlights", "Breaking Changes", "Upgrade Notes"
VERIFY: NO JSON artifacts!
SCREENSHOT: journey-jordan-08-release-notes.png
```

### Step 12: Ask Iris to Polish
```
ACTION: Press '/' to open chat
WAIT: 0.5 seconds
ACTION: Type "Add a fun emoji-filled summary at the top for our Discord announcement"
ACTION: Press Enter
WAIT: 15 seconds
VERIFY: Iris responds with emoji-rich summary
SCREENSHOT: journey-jordan-09-chat-polish.png
```

### Step 13: Final Copy and Exit
```
ACTION: Press Escape to close chat
ACTION: Press 'y' to copy release notes
ACTION: Press 'q' to quit
```

## Expected Content

### Changelog Should Include:
```markdown
## [Unreleased] - 2025-12-30

### Changed
- Switched Studio generation tasks from streaming to non-streaming
- Replaced DynClientBuilder with explicit provider dispatch
- ...

### Fixed
- State management when opening settings
- ...
```

### Release Notes Should Include:
```markdown
# Release Notes

**Released:** 2025-12-30

## Highlights
- Cleaner output generation (no more streaming artifacts!)
- Improved provider abstraction
...

## Internal Changes
- Error handling improvements
...
```

## Success Criteria

- [ ] Changelog mode accessible via Shift+L
- [ ] Release Notes mode accessible via Shift+N
- [ ] Ref selector works for both modes
- [ ] Range persists when switching modes
- [ ] Changelog follows Keep a Changelog format
- [ ] Release Notes are user-friendly narrative
- [ ] NO JSON wrappers in either output
- [ ] Copy to clipboard works
- [ ] Chat can enhance content

## Critical Verification

**THE MOST IMPORTANT CHECK:**
Neither output should contain:
- `{"content": "..."}`
- `Now let me...` thinking text
- Raw tool call information
- Any JSON structure

If any of these appear, the streaming bug has regressed!
