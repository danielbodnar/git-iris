# Journey 04: Morgan - The Explorer

> *"Just joined the team, need to understand how this agent thing works!"*

## Persona Profile

- **Name**: Morgan Lee
- **Role**: New Backend Developer (Day 3!)
- **Experience**: 5 years total, but new to this codebase
- **Goal**: Understand the agent architecture
- **Vibe**: Curious, methodical, takes good notes

## Pre-Journey Setup

No setup needed - just exploring the existing codebase.

## Journey Script

### Step 1: Launch Studio in Explore Mode
```
ACTION: cargo run -- studio
WAIT: 5 seconds for startup
VERIFY: Lands in Explore mode (default when no staged changes)
VERIFY: File tree is populated (NOT EMPTY!)
SCREENSHOT: journey-morgan-01-launch.png
```

### Step 2: Navigate File Tree
```
ACTION: Press 'j' to move down in file tree
WAIT: 0.2 seconds
ACTION: Repeat 'j' 3 more times
SCREENSHOT: journey-morgan-02-navigate.png
```

### Step 3: Expand a Directory
```
ACTION: Navigate to 'src' directory (use j/k)
ACTION: Press Enter or 'l' to expand
WAIT: 0.5 seconds
VERIFY: src directory expands showing contents
SCREENSHOT: journey-morgan-03-expanded.png
```

### Step 4: Dive into Agents
```
ACTION: Navigate to 'src/agents' directory
ACTION: Press Enter to expand
ACTION: Navigate to 'iris.rs'
ACTION: Press Enter to open file
WAIT: 1 second
VERIFY: Code view shows iris.rs content
VERIFY: Syntax highlighting active
SCREENSHOT: journey-morgan-04-iris-file.png
```

### Step 5: View File History
```
ACTION: Press 'L' or Tab to focus History panel
WAIT: 0.5 seconds
VERIFY: History panel shows commits that touched iris.rs
SCREENSHOT: journey-morgan-05-history.png
```

### Step 6: Navigate Code
```
ACTION: Press Tab to focus Code panel
ACTION: Press 'j' 20 times to scroll down
WAIT: 0.1 seconds between presses
SCREENSHOT: journey-morgan-06-code-scroll.png
```

### Step 7: Ask Iris About the Code
```
ACTION: Press '/' to open chat
WAIT: 0.5 seconds
ACTION: Type "What does the IrisAgent struct do? Explain like I'm new to the codebase"
ACTION: Press Enter
WAIT: 20 seconds for response
VERIFY: Iris explains the agent architecture
SCREENSHOT: journey-morgan-07-chat-explain.png
```

### Step 8: Follow-up Question
```
ACTION: Type "How does it decide which tools to use?"
ACTION: Press Enter
WAIT: 15 seconds
VERIFY: Iris explains tool selection
SCREENSHOT: journey-morgan-08-chat-tools.png
```

### Step 9: Explore Another File
```
ACTION: Press Escape to close chat
ACTION: Tab to file tree
ACTION: Navigate to 'src/agents/tools/' directory
ACTION: Expand and select 'git.rs'
ACTION: Press Enter
WAIT: 1 second
VERIFY: Git tools file opens
SCREENSHOT: journey-morgan-09-git-tools.png
```

### Step 10: Ask About Specific Code
```
ACTION: Press '/' for chat
ACTION: Type "What git operations can Iris perform?"
ACTION: Press Enter
WAIT: 15 seconds
SCREENSHOT: journey-morgan-10-chat-git.png
```

### Step 11: Exit
```
ACTION: Press Escape
ACTION: Press 'q' to quit
```

## Success Criteria

- [ ] Explore mode shows populated file tree (THE FIX WE MADE!)
- [ ] Can navigate tree with j/k
- [ ] Can expand directories with Enter/l
- [ ] Code view shows syntax highlighting
- [ ] History panel shows file commits
- [ ] Chat understands code context
- [ ] Can ask follow-up questions
- [ ] Smooth navigation between panels

## Key UX Test Points

1. **File Tree Population** - Does it load immediately? (Fixed bug!)
2. **Navigation Feel** - Is j/k/Enter intuitive?
3. **Code Readability** - Is syntax highlighting good?
4. **Chat Context** - Does Iris know what file we're looking at?
5. **Panel Focus** - Is it clear which panel is active?

## Questions to Verify Chat Quality

Morgan might ask:
- "What's the main entry point?"
- "How do capabilities work?"
- "What's the difference between streaming and non-streaming?"
- "Where are the prompts defined?"

Iris should give helpful, contextual answers!
