# Journey 01: Alex - The Daily Committer

> *"Just shipped a bug fix, need to commit before standup!"*

## Persona Profile

- **Name**: Alex Chen
- **Role**: Junior Frontend Developer
- **Experience**: 6 months at the company
- **Goal**: Make a quick, well-formatted commit
- **Vibe**: Energetic, emoji-lover, wants things to look nice

## Pre-Journey Setup

```bash
# Create a fun test file to stage
cat > /tmp/alex-test-fix.rs << 'EOF'
// Fixed the bug where users couldn't see their notifications
// The issue was we were filtering by the wrong user ID (oops!)
pub fn get_notifications(user_id: UserId) -> Vec<Notification> {
    // OLD: notifications.filter(|n| n.sender_id == user_id)  // WRONG!
    // NEW: Filter by recipient, not sender
    notifications.filter(|n| n.recipient_id == user_id)
        .collect()
}
EOF

# Copy to repo and stage it
cp /tmp/alex-test-fix.rs src/alex_bugfix.rs
git add src/alex_bugfix.rs
```

## Journey Script

### Step 1: Launch Studio
```
ACTION: cargo run -- studio
WAIT: 5 seconds for startup
VERIFY: Lands in Commit mode (has staged changes)
SCREENSHOT: journey-alex-01-launch.png
```

### Step 2: View Generated Message
```
WAIT: 15 seconds for AI generation
VERIFY: Message panel shows generated commit
VERIFY: Emoji is present (likely bug fix emoji)
SCREENSHOT: journey-alex-02-generated.png
```

### Step 3: Cycle Through Emojis
```
ACTION: Press 'g' three times
WAIT: 0.5 seconds between presses
VERIFY: Emoji changes each press
SCREENSHOT: journey-alex-03-emoji-cycle.png
```

### Step 4: Edit the Message
```
ACTION: Press 'e' to enter edit mode
WAIT: 0.5 seconds
ACTION: Type " - fixed the notification bug that drove everyone crazy"
ACTION: Press Escape to exit edit
SCREENSHOT: journey-alex-04-edited.png
```

### Step 5: Preview Final Message
```
ACTION: Press 'p' to cycle to next/previous if available
VERIFY: Can see the full commit message
SCREENSHOT: journey-alex-05-preview.png
```

### Step 6: Open Chat for Fun
```
ACTION: Press '/' to open chat
WAIT: 0.5 seconds
ACTION: Type "make the commit message funnier but still professional"
ACTION: Press Enter
WAIT: 10 seconds for response
SCREENSHOT: journey-alex-06-chat.png
ACTION: Press Escape to close chat
```

### Step 7: Commit! (Optional - skip in test)
```
# In real test, we'd press Enter to commit
# For safety, we'll just capture the final state
ACTION: Press 'q' to quit (don't actually commit in test)
```

## Expected Screenshots

| Screenshot | Shows |
|------------|-------|
| 01-launch | Commit mode with staged file visible |
| 02-generated | AI-generated commit message with emoji |
| 03-emoji-cycle | Different emoji selected |
| 04-edited | Custom edit appended |
| 05-preview | Full message preview |
| 06-chat | Chat modal with Iris response |

## Success Criteria

- [ ] Studio launches into Commit mode
- [ ] File tree shows staged file
- [ ] Commit message generates within 20s
- [ ] Emoji cycling works (g key)
- [ ] Edit mode works (e key)
- [ ] Chat opens and responds (/ key)
- [ ] No crashes or visual glitches

## Cleanup

```bash
git reset HEAD src/alex_bugfix.rs
rm -f src/alex_bugfix.rs
```
