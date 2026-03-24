---
name: add-or-update-skill
description: Workflow command scaffold for add-or-update-skill in everything-claude-code.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /add-or-update-skill

Use this workflow when working on **add-or-update-skill** in `everything-claude-code`.

## Goal

Adds or updates a skill in the ECC system, including documentation and provenance.

## Common Files

- `skills/*/SKILL.md`
- `.claude/skills/*/SKILL.md`
- `.agents/skills/*/SKILL.md`
- `.agents/skills/*/agents/*.yaml`
- `schemas/provenance.schema.json`
- `docs/SKILL-PLACEMENT-POLICY.md`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Create or update SKILL.md in the appropriate skills directory (e.g., skills/{skill-name}/SKILL.md or .claude/skills/{skill-name}/SKILL.md)
- Optionally add or update agent YAML files (e.g., .agents/skills/{skill-name}/agents/*.yaml)
- Optionally update provenance or placement policy files (e.g., schemas/provenance.schema.json, docs/SKILL-PLACEMENT-POLICY.md)

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.