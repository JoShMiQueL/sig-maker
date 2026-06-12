# Release Checklist

## Pre-release

- [ ] Quality verified (fmt, clippy, tests)
- [ ] Version bumped in Cargo.toml
- [ ] Version follows semantic versioning
- [ ] Pre-release identifier set if needed (rc/beta/alpha)

## Build

- [ ] CLI built for Windows x64
- [ ] CLI built for Linux x64
- [ ] CLI built for Linux ARM64
- [ ] CLI built for macOS Intel
- [ ] CLI built for macOS ARM64
- [ ] GUI built for Windows (NSIS installer + portable)
- [ ] GUI built for Linux (AppImage)
- [ ] GUI built for macOS (DMG universal)

## Artifacts

- [ ] SHA-256 checksums generated for all artifacts
- [ ] Checksums file created (checksums.txt)
- [ ] All artifacts tested (if possible)

## Changelog

- [ ] Commits since last release reviewed
- [ ] Changes grouped by type (feat, fix, docs, ci)
- [ ] Changelog follows format in REFERENCE.md
- [ ] New contributors section added if applicable
- [ ] Contributors section complete
- [ ] Full Changelog link included

## Git

- [ ] Version committed to Cargo.toml
- [ ] Commit message: `chore(release): v{version}`
- [ ] Git tag created: `git tag v{version}`
- [ ] Git tag pushed: `git push --tags`

## GitHub Release

- [ ] Draft release created
- [ ] Tag selected: `v{version}`
- [ ] Title: `v{version}`
- [ ] All 9 artifacts uploaded
- [ ] Checksums.txt uploaded
- [ ] Changelog pasted as release body
- [ ] Release body verified
- [ ] Marked as pre-release if applicable
- [ ] Release published

## Post-release

- [ ] Documentation updated (if needed)
- [ ] README updated (if needed)
- [ ] Announcement made (if applicable)
