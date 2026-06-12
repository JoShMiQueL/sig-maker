# Release Reference

## Cross-compilation Targets

### CLI

```bash
# Linux x64
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu --bin sig-maker-cli

# Linux ARM64
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu --bin sig-maker-cli

# macOS Intel
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin --bin sig-maker-cli

# macOS ARM64
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin --bin sig-maker-cli
```

### GUI

Use Tauri bundler for each platform:
- Windows: `cargo tauri build`
- Linux: `cargo tauri build` (produces AppImage)
- macOS: `cargo tauri build` (produces DMG)

## Changelog Format

```markdown
## [v{version}] - {date}

### Features

- *(scope)* Description of feature ([commit_hash](link)) by [@author](link)
- *(scope)* Description of feature [#pr_number] by [@author](link)

### Bug Fixes

- *(scope)* Description of fix ([commit_hash](link)) by [@author](link)
- *(scope)* Description of fix [#pr_number] by [@author](link)

### Documentation

- *(scope)* Description of doc change ([commit_hash](link)) by [@author](link)
- *(scope)* Description of doc change [#pr_number] by [@author](link)

### CI/CD

- *(scope)* Description of CI change ([commit_hash](link)) by [@author](link)
- *(scope)* Description of CI change [#pr_number] by [@author](link)

### New Contributors

* [@new_contributor](link) made their first contribution in [#pr_number]

### Contributors

* [@author1](link)
* [@author2](link)

**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/v{last_version}...v{version}
```

**Format rules:**
- Use conventional commit types as section headers
- Include scope in parentheses when applicable
- Link commits to GitHub (for direct commits)
- Use [#pr_number] format for PRs (GitHub auto-links)
- Link authors to GitHub profiles
- New contributors get their own section
- Group related changes together
- List all contributors
- Add Full Changelog link at the end

**Example:**

```markdown
## [v0.1.3-beta] - 2026-06-12

### Features

- *(gui)* Add Tauri GUI with real-time pattern conversion [[#37](https://github.com/JoShMiQueL/sig-maker/pull/37)] by @JoShMiQueL
- *(cli)* Add JSON output format support [[#38](https://github.com/JoShMiQueL/sig-maker/pull/38)] by @JoShMiQueL

### Bug Fixes

- *(analyzer)* Fix pattern optimization for multi-line AOBs [[#39](https://github.com/JoShMiQueL/sig-maker/pull/39)] by @NewContributor
- *(core)* Handle empty pattern lists gracefully (abc1234) by @JoShMiQueL

### Documentation

- Update README with build instructions [[#40](https://github.com/JoShMiQueL/sig-maker/pull/40)] by @JoShMiQueL

### New Contributors

* @NewContributor made their first contribution in [[#39](https://github.com/JoShMiQueL/sig-maker/pull/39)]

### Contributors

* @JoShMiQueL
* @NewContributor

**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/v0.1.2-beta...v0.1.3-beta
```

## Semantic Versioning

### Version Format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

### Pre-release Identifiers

- `v0.1.3-rc.1` - Release Candidate (stable, ready for testing)
- `v0.1.3-beta.1` - Beta (feature complete, may have bugs)
- `v0.1.3-alpha.1` - Alpha (early development, unstable)

### When to Increment

- **MAJOR**: Breaking changes to CLI API or core library
- **MINOR**: New features, new formats, new GUI capabilities
- **PATCH**: Bug fixes, documentation, CI improvements

## Pre-release Guidelines

### Alpha

- Early development, unstable
- For experimental features
- May have breaking changes
- Not for production use

### Beta

- Feature complete, may have bugs
- For wider testing
- Should be relatively stable
- May have minor breaking changes

### Release Candidate (RC)

- Stable, ready for production
- Only critical bug fixes
- No breaking changes
- Final testing before stable release

## Checklist

- [ ] Quality verified (fmt, clippy, tests)
- [ ] Version bumped in Cargo.toml
- [ ] CLI built for all 5 platforms
- [ ] GUI built for all 3 platforms
- [ ] Checksums generated for all artifacts
- [ ] Changelog created following format
- [ ] Git tag created and pushed
- [ ] GitHub Release created with all artifacts
- [ ] Release notes pasted
- [ ] Documentation updated (if needed)
