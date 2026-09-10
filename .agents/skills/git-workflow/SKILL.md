---
name: git-workflow
description: Safe concurrent Git workflow for autonomous coding agents working on backend and frontend.
---

# Git Workflow Skill

## 1. Principles for Concurrent Multi-Agent Development
OpenCode (Backend) and Claude Code (Frontend) work on the same codebase concurrently. Strict hygiene prevents merge collisions and accidental file overwrites.

## 2. Standard Workflow
1. **Inspect Status Before Work**:
   ```bash
   git status
   git diff
   ```
2. **Feature Branching**:
   - Backend tasks: `feat/be-<feature-name>` or `fix/be-<bug-name>`
   - Frontend tasks: `feat/fe-<feature-name>` or `fix/fe-<bug-name>`
   - Contract changes: `contract/<change-name>`
3. **Inspect Diff Before Committing**:
   - Always run `git diff --staged` to verify that only intended files are staged.
   - Never commit unrelated scratch files, IDE configs, or credentials.
4. **Focused, Atomic Commits**:
   - Write clear, standard commit messages: `feat(report): implement docx placeholder replacement` or `test(auth): add rate limit middleware tests`.

## 3. Prohibited Git Actions
- ❌ **No `git push --force`** on shared or main branches.
- ❌ **No destructive resets** (`git reset --hard`) without explicit reason and verification.
- ❌ **No editing files owned by the other agent** without clear cross-layer coordination.
- ❌ **No committing secrets**: `.env` and private keys must always be in `.gitignore`.
