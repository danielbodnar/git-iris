# Iris Studio UX Journey Tests

Automated user journey tests using Ghostty terminal automation.

## Personas

| Persona | Role | Primary Journey |
|---------|------|-----------------|
| **Alex** | Junior Dev | Quick commit with emoji cycling |
| **Sam** | Senior Dev | Code review with ref selection |
| **Jordan** | Release Manager | Changelog + release notes |
| **Morgan** | New Team Member | Codebase exploration |
| **Casey** | Feature Developer | PR description creation |

## Running Tests

Each journey can be run via Claude Code with Ghostty MCP:

```
"Run the Alex journey test"
"Execute Sam's code review journey"
```

## Test Data Requirements

- **Alex**: Needs staged changes (will stage test file)
- **Sam**: Needs commit history (uses last 5 commits)
- **Jordan**: Needs tag history (uses dfbd9c5..HEAD)
- **Morgan**: Just needs the codebase
- **Casey**: Needs feature branch (or uses main)

## Verification Points

Each journey captures screenshots at key moments for visual regression testing.
