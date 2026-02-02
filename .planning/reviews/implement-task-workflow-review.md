---
title: Implement Task Workflow — Review
author: agent
date: 2026-01-31
---

Summary
-------
Updated the `implement-tasks` Agent skill to include a compact, actionable workflow that uses `.planning` artifacts (`PROJECT.md`, `ROADMAP.md`, `MILESTONES.md`, `STATE.md`, `milestones/*`, `phases/*`) to plan, implement, verify and review features/optimizations.

Files changed
- `.claude/skills/implement-tasks/SKILL.md` — replaced the verbose checklist with the new Agent workflow, inputs/outputs, templates, and quick verification commands.

Rationale
---------
Provide a minimal, repeatable process for Agents to: ingest planning artifacts, create a task checklist in `.planning/tasks/`, implement iteratively with tests/benchmarks, document changes, and produce a review note before opening a PR.

Verification steps (manual/automated)
-----------------------------------
1. Review markdown for `SKILL.md` changes.
2. Confirm `.planning/tasks/` template exists in the skill (agent will create per task).
3. Run basic repo checks (no edits to example-project):

   - Run core tests (if relevant):

     cd core && cargo test

   - Run performance helper (if needed):

     ./scripts/test-performance.sh

4. Confirm FFI safety notes present in `SKILL.md` (wrap `catch_unwind`, avoid `unwrap()` on FFI boundary).

Next steps
----------
- Run the quick verification commands above and report any test failures.
- If you want, I can create a `.planning/tasks/implement-task-workflow.md` instance and/or open a branch and a PR with the `SKILL.md` changes.

Notes
-----
This review note accompanies the Agent workflow change; it is meant to be committed alongside the skill update and linked from PR description for reviewers.
