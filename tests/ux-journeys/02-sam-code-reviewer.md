# Journey 02: Sam - The Code Reviewer

> *"Let me review what the team shipped this week before the release"*

## Persona Profile

- **Name**: Sam Rodriguez
- **Role**: Senior Backend Engineer & Tech Lead
- **Experience**: 8 years, 3 at this company
- **Goal**: Review recent changes, identify risks, provide feedback
- **Vibe**: Thorough, security-conscious, mentorship-focused

## Pre-Journey Setup

No special setup needed - uses existing commit history.

```bash
# Verify we have good commit range
git log --oneline dfbd9c5..HEAD
# Should show: streaming fix, state management fix, etc.
```

## Journey Script

### Step 1: Launch Studio
```
ACTION: cargo run -- studio
WAIT: 5 seconds for startup
SCREENSHOT: journey-sam-01-launch.png
```

### Step 2: Switch to Review Mode
```
ACTION: Press Shift+R (uppercase R)
WAIT: 1 second
VERIFY: Mode indicator shows "R Review"
VERIFY: Review panel shows "Press 'r' to generate"
SCREENSHOT: journey-sam-02-review-mode.png
```

### Step 3: Select From Reference (THE KEY TEST!)
```
ACTION: Press 'f' to open from-ref selector
WAIT: 1 second
VERIFY: Ref selector modal opens
SCREENSHOT: journey-sam-03-ref-selector.png
```

### Step 4: Navigate and Select Commit
```
ACTION: Press 'j' three times to navigate down
WAIT: 0.3 seconds between presses
ACTION: Press Enter to select (should be ~dfbd9c5)
WAIT: 0.5 seconds
VERIFY: From ref updated in header
SCREENSHOT: journey-sam-04-from-selected.png
```

### Step 5: Keep To-Ref as HEAD (default)
```
# To-ref defaults to HEAD, which is what Sam wants
VERIFY: Header shows "dfbd9c5 -> HEAD" or similar
```

### Step 6: Generate Review
```
ACTION: Press 'r' to generate review
WAIT: 30 seconds for AI generation
VERIFY: Review content appears (markdown formatted)
VERIFY: Shows sections like "Summary", "Strengths", "Issues"
VERIFY: NO JSON artifacts or thinking text!
SCREENSHOT: journey-sam-05-review-generated.png
```

### Step 7: Scroll Through Review
```
ACTION: Press 'j' 5 times to scroll down
WAIT: 0.2 seconds between presses
SCREENSHOT: journey-sam-06-scrolled.png
```

### Step 8: Ask Iris About Risks
```
ACTION: Press '/' to open chat
WAIT: 0.5 seconds
ACTION: Type "What's the riskiest change in this review? Any security concerns?"
ACTION: Press Enter
WAIT: 15 seconds for response
VERIFY: Iris responds with specific analysis
SCREENSHOT: journey-sam-07-chat-risk.png
```

### Step 9: Copy Review
```
ACTION: Press Escape to close chat
ACTION: Press 'y' to copy review to clipboard
VERIFY: Notification shows "Copied to clipboard"
SCREENSHOT: journey-sam-08-copied.png
```

### Step 10: Exit
```
ACTION: Press 'q' to quit
```

## Expected Content Highlights

The review should mention:
- Streaming to non-streaming refactor
- State management improvements
- Provider abstraction changes
- Error handling improvements

## Success Criteria

- [ ] Review mode accessible via Shift+R
- [ ] Ref selector opens with 'f' key
- [ ] Can navigate commits with j/k
- [ ] Can select commit with Enter
- [ ] Review generates clean markdown
- [ ] No JSON wrappers in output
- [ ] Chat responds contextually
- [ ] Copy to clipboard works

## Key UX Test Points

1. **Ref Selection Flow** - Is it obvious how to select range?
2. **Loading States** - Does user know generation is happening?
3. **Output Quality** - Is the review useful and well-formatted?
4. **Chat Context** - Does Iris understand the review context?
