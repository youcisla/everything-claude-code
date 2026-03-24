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

- `feat`
- `fix`
- `docs`
- `test`

### Message Guidelines

- Average message length: ~62 characters
- Keep first line concise and descriptive
- Use imperative mood ("Add feature" not "Added feature")


*Commit message example*

```text
feat: add everything-claude-code ECC bundle (.claude/commands/add-or-update-skill.md)
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
feat: add everything-claude-code ECC bundle (.claude/commands/feature-development.md)
```

*Commit message example*

```text
feat: add everything-claude-code ECC bundle (.claude/commands/database-migration.md)
```

*Commit message example*

```text
feat: add everything-claude-code ECC bundle (.claude/enterprise/controls.md)
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
Add Turkish (tr) docs and update README (#744)
docs(zh-CN): translate code block(plain text) (#753)
fix(install): add rust, cpp, csharp to legacy language alias map (#747)
```

### Feature Development

Standard feature implementation workflow

**Frequency**: ~26 times per month

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
Merge pull request #736 from pvgomes/docs/add-brazilian-portuguese-translation
fix: bump plugin.json and marketplace.json to v1.9.0
Add Turkish (tr) docs and update README (#744)
```

### Add Or Update Command Doc

Adds or updates documentation for a command, typically in Markdown under a language or locale-specific docs directory.

**Frequency**: ~3 times per month

**Steps**:
1. Create or update a Markdown file for the command in the appropriate docs directory (e.g., docs/zh-CN/commands/ or docs/tr/commands/).
2. Commit the new or changed file with a message referencing the command.
3. Optionally update README or language count if adding a new language.

**Files typically involved**:
- `docs/zh-CN/commands/*.md`
- `docs/tr/commands/*.md`
- `docs/pt-BR/commands/*.md`

**Example commit sequence**:
```
Create or update a Markdown file for the command in the appropriate docs directory (e.g., docs/zh-CN/commands/ or docs/tr/commands/).
Commit the new or changed file with a message referencing the command.
Optionally update README or language count if adding a new language.
```

### Add Or Update Skill Doc

Adds or updates documentation for a skill, typically in SKILL.md under a language or locale-specific docs directory.

**Frequency**: ~3 times per month

**Steps**:
1. Create or update a SKILL.md file for the skill in the appropriate docs directory (e.g., docs/zh-CN/skills/, docs/tr/skills/, docs/pt-BR/skills/).
2. Commit the new or changed file with a message referencing the skill.

**Files typically involved**:
- `docs/zh-CN/skills/*/SKILL.md`
- `docs/tr/skills/*/SKILL.md`
- `docs/pt-BR/skills/*/SKILL.md`

**Example commit sequence**:
```
Create or update a SKILL.md file for the skill in the appropriate docs directory (e.g., docs/zh-CN/skills/, docs/tr/skills/, docs/pt-BR/skills/).
Commit the new or changed file with a message referencing the skill.
```

### Add Or Update Locale Docs

Adds or updates a full set of localized documentation for a new or existing language, including agents, commands, skills, and guides.

**Frequency**: ~2 times per month

**Steps**:
1. Create or update multiple Markdown files under a new or existing language directory (e.g., docs/tr/, docs/pt-BR/, docs/zh-CN/).
2. Update README.md to increment supported language count and add references.
3. Commit all new or changed files.

**Files typically involved**:
- `docs/tr/**/*`
- `docs/pt-BR/**/*`
- `docs/zh-CN/**/*`
- `README.md`

**Example commit sequence**:
```
Create or update multiple Markdown files under a new or existing language directory (e.g., docs/tr/, docs/pt-BR/, docs/zh-CN/).
Update README.md to increment supported language count and add references.
Commit all new or changed files.
```

### Add Or Update Ecc Bundle Command

Adds or updates ECC bundle command documentation or configuration, typically in .claude/commands/ or related ECC config directories.

**Frequency**: ~2 times per month

**Steps**:
1. Create or update a Markdown file in .claude/commands/ for the new or updated command.
2. Optionally update related config files (e.g., .claude/ecc-tools.json, .claude/identity.json).
3. Commit the changes.

**Files typically involved**:
- `.claude/commands/*.md`
- `.claude/ecc-tools.json`
- `.claude/identity.json`

**Example commit sequence**:
```
Create or update a Markdown file in .claude/commands/ for the new or updated command.
Optionally update related config files (e.g., .claude/ecc-tools.json, .claude/identity.json).
Commit the changes.
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
