---
name: everything-claude-code-conventions
description: Development conventions and patterns for everything-claude-code. JavaScript project with conventional commits.
---

# Everything Claude Code Conventions

> Generated from [affaan-m/everything-claude-code](https://github.com/affaan-m/everything-claude-code) on 2026-03-24

## Overview

This skill teaches Claude the development patterns and conventions used in everything-claude-code.

## Tech Stack

- **Primary Language**: JavaScript
- **Architecture**: hybrid module organization
- **Test Location**: separate

## When to Use This Skill

Activate this skill when:
- Making changes to this repository
- Adding new features following established patterns
- Writing tests that match project conventions
- Creating commits with proper message format

## Commit Conventions

Follow these commit message conventions based on 500 analyzed commits.

### Commit Style: Conventional Commits

### Prefixes Used

- `fix`
- `feat`
- `docs`
- `test`

### Message Guidelines

- Average message length: ~62 characters
- Keep first line concise and descriptive
- Use imperative mood ("Add feature" not "Added feature")


*Commit message example*

```text
feat(ecc2): add split-pane dashboard resizing
```

*Commit message example*

```text
perf(hooks): move post-edit-format and post-edit-typecheck to strict-only (#757)
```

*Commit message example*

```text
fix: safe Codex config sync — merge AGENTS.md + add-only MCP servers (#723)
```

*Commit message example*

```text
docs(zh-CN): translate code block(plain text) (#753)
```

*Commit message example*

```text
security: remove supply chain risks, external promotions, and unauthorized credits
```

*Commit message example*

```text
feat: scaffold ECC 2.0 Rust TUI — agentic IDE control plane
```

*Commit message example*

```text
feat(skills): add santa-method - multi-agent adversarial verification (#760)
```

*Commit message example*

```text
feat: pending instinct TTL pruning and /prune command (#725)
```

## Architecture

### Project Structure: Single Package

This project uses **hybrid** module organization.

### Configuration Files

- `.github/workflows/ci.yml`
- `.github/workflows/maintenance.yml`
- `.github/workflows/monthly-metrics.yml`
- `.github/workflows/release.yml`
- `.github/workflows/reusable-release.yml`
- `.github/workflows/reusable-test.yml`
- `.github/workflows/reusable-validate.yml`
- `.opencode/package.json`
- `.opencode/tsconfig.json`
- `.prettierrc`
- `eslint.config.js`
- `package.json`

### Guidelines

- This project uses a hybrid organization
- Follow existing patterns when adding new code

## Code Style

### Language: JavaScript

### Naming Conventions

| Element | Convention |
|---------|------------|
| Files | camelCase |
| Functions | camelCase |
| Classes | PascalCase |
| Constants | SCREAMING_SNAKE_CASE |

### Import Style: Relative Imports

### Export Style: Mixed Style


*Preferred import style*

```typescript
// Use relative imports
import { Button } from '../components/Button'
import { useAuth } from './hooks/useAuth'
```

## Testing

### Test Framework

No specific test framework detected — use the repository's existing test patterns.

### File Pattern: `*.test.js`

### Test Types

- **Unit tests**: Test individual functions and components in isolation
- **Integration tests**: Test interactions between multiple components/services

### Coverage

This project has coverage reporting configured. Aim for 80%+ coverage.


## Error Handling

### Error Handling Style: Try-Catch Blocks


*Standard error handling pattern*

```typescript
try {
  const result = await riskyOperation()
  return result
} catch (error) {
  console.error('Operation failed:', error)
  throw new Error('User-friendly message')
}
```

## Common Workflows

These workflows were detected from analyzing commit patterns.

### Database Migration

Database schema changes with migration files

**Frequency**: ~2 times per month

**Steps**:
1. Create migration file
2. Update schema definitions
3. Generate/update types

**Files typically involved**:
- `migrations/*`

**Example commit sequence**:
```
feat(rules): add C# language support (#704)
fix: sanitize SessionStart session summaries (#710)
feat: add MCP health-check hook (#711)
```

### Feature Development

Standard feature implementation workflow

**Frequency**: ~16 times per month

**Steps**:
1. Add feature implementation
2. Add tests for feature
3. Update documentation

**Files typically involved**:
- `manifests/*`
- `**/*.test.*`
- `**/api/**`

**Example commit sequence**:
```
feat: agent description compression with lazy loading (#696)
feat: add nuxt 4 patterns skill (#702)
feat(rules): add C# language support (#704)
```

### Add Or Update Skill Documentation

Adds a new skill or updates documentation for an existing skill. Typically involves creating or modifying SKILL.md files under skills/ or docs/xx/skills/ directories.

**Frequency**: ~4 times per month

**Steps**:
1. Create or update SKILL.md under skills/<skill-name>/ or docs/<lang>/skills/<skill-name>/
2. Optionally update AGENTS.md or README.md to reflect new skill count or catalog
3. Commit with message referencing the skill and a summary of changes

**Files typically involved**:
- `skills/*/SKILL.md`
- `docs/*/skills/*/SKILL.md`
- `AGENTS.md`
- `README.md`

**Example commit sequence**:
```
Create or update SKILL.md under skills/<skill-name>/ or docs/<lang>/skills/<skill-name>/
Optionally update AGENTS.md or README.md to reflect new skill count or catalog
Commit with message referencing the skill and a summary of changes
```

### Add Or Update Localization

Adds or updates documentation translations for a new or existing language. Involves creating or modifying multiple files under docs/<lang>/, often in bulk.

**Frequency**: ~3 times per month

**Steps**:
1. Add or update markdown files under docs/<lang>/ for agents, commands, skills, rules, etc.
2. Update README.md to include the new language or increment language count
3. Commit with message referencing the language and scope (e.g., pt-BR, zh-CN, tr)

**Files typically involved**:
- `docs/*/README.md`
- `docs/*/agents/*.md`
- `docs/*/commands/*.md`
- `docs/*/skills/*/SKILL.md`
- `docs/*/rules/**/*.md`
- `README.md`

**Example commit sequence**:
```
Add or update markdown files under docs/<lang>/ for agents, commands, skills, rules, etc.
Update README.md to include the new language or increment language count
Commit with message referencing the language and scope (e.g., pt-BR, zh-CN, tr)
```

### Add Or Update Hook

Adds a new hook or updates an existing one for agentic workflows. Involves changes to hooks.json and corresponding scripts.

**Frequency**: ~2 times per month

**Steps**:
1. Add or update entry in hooks/hooks.json
2. Create or modify corresponding script in scripts/hooks/
3. Optionally add or update tests in tests/hooks/ or tests/integration/
4. Commit with message referencing the hook and its purpose

**Files typically involved**:
- `hooks/hooks.json`
- `scripts/hooks/*.js`
- `tests/hooks/*.test.js`
- `tests/integration/*.test.js`

**Example commit sequence**:
```
Add or update entry in hooks/hooks.json
Create or modify corresponding script in scripts/hooks/
Optionally add or update tests in tests/hooks/ or tests/integration/
Commit with message referencing the hook and its purpose
```

### Add Or Update Session Adapter Feature

Adds or updates features in the session adapter, typically involving implementation, contract documentation, and tests.

**Frequency**: ~1 times per month

**Steps**:
1. Update implementation in scripts/lib/session-adapters/canonical-session.js
2. Update contract documentation in docs/SESSION-ADAPTER-CONTRACT.md
3. Add or update corresponding tests in tests/lib/session-adapters.test.js

**Files typically involved**:
- `scripts/lib/session-adapters/canonical-session.js`
- `docs/SESSION-ADAPTER-CONTRACT.md`
- `tests/lib/session-adapters.test.js`

**Example commit sequence**:
```
Update implementation in scripts/lib/session-adapters/canonical-session.js
Update contract documentation in docs/SESSION-ADAPTER-CONTRACT.md
Add or update corresponding tests in tests/lib/session-adapters.test.js
```

### Add Or Update Language Support

Adds or updates support for a programming language, including rules, patterns, and install scripts/tests.

**Frequency**: ~1 times per month

**Steps**:
1. Add or update rules under rules/<language>/ (coding-style.md, hooks.md, patterns.md, security.md, testing.md)
2. Update scripts/lib/install-manifests.js and manifests/install-components.json for install support
3. Add or update tests in tests/lib/install-manifests.test.js

**Files typically involved**:
- `rules/*/*.md`
- `scripts/lib/install-manifests.js`
- `manifests/install-components.json`
- `tests/lib/install-manifests.test.js`

**Example commit sequence**:
```
Add or update rules under rules/<language>/ (coding-style.md, hooks.md, patterns.md, security.md, testing.md)
Update scripts/lib/install-manifests.js and manifests/install-components.json for install support
Add or update tests in tests/lib/install-manifests.test.js
```

### Codex Config Sync And Merge

Synchronizes and merges Codex configuration files, ensuring user content is preserved and ECC-managed sections are updated.

**Frequency**: ~1 times per month

**Steps**:
1. Update or create scripts/codex/merge-mcp-config.js or scripts/sync-ecc-to-codex.sh
2. Update AGENTS.md or .codex/AGENTS.md as needed
3. Update README.md to reflect server/catalog changes
4. Commit with message referencing sync/merge and affected files

**Files typically involved**:
- `scripts/codex/merge-mcp-config.js`
- `scripts/sync-ecc-to-codex.sh`
- `.codex/AGENTS.md`
- `README.md`

**Example commit sequence**:
```
Update or create scripts/codex/merge-mcp-config.js or scripts/sync-ecc-to-codex.sh
Update AGENTS.md or .codex/AGENTS.md as needed
Update README.md to reflect server/catalog changes
Commit with message referencing sync/merge and affected files
```


## Best Practices

Based on analysis of the codebase, follow these practices:

### Do

- Use conventional commit format (feat:, fix:, etc.)
- Follow *.test.js naming pattern
- Use camelCase for file names
- Prefer mixed exports

### Don't

- Don't write vague commit messages
- Don't skip tests for new features
- Don't deviate from established patterns without discussion

---

*This skill was auto-generated by [ECC Tools](https://ecc.tools). Review and customize as needed for your team.*
