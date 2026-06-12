---
name: release
description: Guide manual release process for sig-maker across all platforms. Use when user mentions release, publish, or distributing new version.
---

# Release

## Quick start

```bash
# 1. Verify quality
cargo fmt -- --check && cargo clippy --workspace -- -D warnings && cargo test --workspace

# 2. Bump version in Cargo.toml
# 3. Build artifacts
cargo build --release --bin sig-maker-cli
cargo build --release --bin sig-maker-gui

# 4. Create git tag
git tag v{version}
git push --tags

# 5. Create GitHub Release with artifacts and changelog
```

## Workflows

### Standard Release

1. Verify quality (fmt, clippy, tests)
2. Bump version in `Cargo.toml`
3. Build CLI for 5 platforms
4. Build GUI for 3 platforms
5. Generate SHA-256 checksums
6. Create changelog
7. Create git tag and push
8. Create GitHub Release as draft
9. Upload all artifacts to draft
10. Paste changelog and verify
11. Publish release (mark as pre-release if applicable)

### Rollback

If release has issues:
1. Delete the GitHub Release
2. Delete the git tag: `git tag -d v{version}` and `git push origin :refs/tags/v{version}`
3. Fix issues and create new release with incremented version

## Advanced

See [REFERENCE.md](REFERENCE.md) for cross-compilation and changelog format
See [VERSIONING.md](VERSIONING.md) for semantic versioning and pre-release detection
See [EXAMPLES.md](EXAMPLES.md) for complete release walkthroughs
See [CHECKLIST.md](CHECKLIST.md) for comprehensive release checklist
