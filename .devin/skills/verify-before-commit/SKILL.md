---
name: verify-before-commit
description: Run full verification (fmt, clippy, build, test) before committing any changes.
triggers: ["user"]
---

## Verification Steps

1. **Format check:** Run `cargo fmt -- --check` to ensure code is properly formatted
2. **Linting:** Run `cargo clippy --workspace -- -D warnings` to catch potential issues
3. **Build:** Run `cargo build --workspace` to ensure the project compiles
4. **Test:** Run `cargo test --workspace` to ensure all tests pass

## Context

- Current branch: !`git branch --show-current`
- Last commit: !`git log --oneline -1`
- Unstaged changes: !`git diff --stat`
- Staged changes: !`git diff --cached --stat`

## Success Criteria

All commands must exit with code 0. If any command fails:
1. Report the specific error
2. Suggest fixes based on the error output
3. Do not proceed with commit until all checks pass

## Notes

- These are the same checks enforced by the pre-commit hook
- The pre-commit hook runs automatically on commit, but this skill allows manual verification
- Always ask for user confirmation before committing (NEVER commit automatically)
