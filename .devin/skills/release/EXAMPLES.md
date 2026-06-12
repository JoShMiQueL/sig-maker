# Release Examples

## Example 1: Standard Release v0.1.3-beta

### Step 1: Verify Quality

```bash
cargo fmt -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

### Step 2: Bump Version

Edit `Cargo.toml`:
```toml
[workspace.package]
version = "0.1.3-beta"
```

### Step 3: Build CLI (Windows x64)

```bash
cargo build --release --bin sig-maker-cli
# Binary: target/release/sig-maker-cli.exe
```

### Step 4: Build GUI (Windows x64)

```bash
cd crates/sig-maker-gui
cargo tauri build
# Installer: target/release/bundle/nsis/sig-maker-gui_0.1.3-beta_x64-setup.exe
# Portable: target/release/bundle/nsis/sig-maker-gui_0.1.3-beta_x64-portable.zip
```

### Step 5: Generate Checksums

```bash
cd target/release
sha256sum sig-maker-cli.exe > checksums.txt
sha256sum bundle/nsis/sig-maker-gui_0.1.3-beta_x64-setup.exe >> checksums.txt
sha256sum bundle/nsis/sig-maker-gui_0.1.3-beta_x64-portable.zip >> checksums.txt
```

### Step 6: Create Changelog

```markdown
## [v0.1.3-beta] - 2026-06-12

### Features

- *(gui)* Add Tauri GUI with real-time pattern conversion [[#37](https://github.com/JoShMiQueL/sig-maker/pull/37)] by @JoShMiQueL

### Bug Fixes

- *(analyzer)* Fix pattern optimization for multi-line AOBs [[#39](https://github.com/JoShMiQueL/sig-maker/pull/39)] by @JoShMiQueL

### Contributors

* @JoShMiQueL

**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/v0.1.2-beta...v0.1.3-beta
```

### Step 7: Create Git Tag

```bash
git add Cargo.toml
git commit -m "chore(release): v0.1.3-beta"
git tag v0.1.3-beta
git push
git push --tags
```

### Step 8: Create GitHub Release

1. Go to https://github.com/JoShMiQueL/sig-maker/releases/new
2. Tag: `v0.1.3-beta`
3. Title: `v0.1.3-beta`
4. Body: Paste changelog
5. Check "Set as a pre-release" (if beta/alpha)
6. Click "Generate release notes" (optional)
7. Click "Publish release" (creates draft first)
8. Upload artifacts:
   - `sig-maker-cli.exe`
   - `sig-maker-gui_0.1.3-beta_x64-setup.exe`
   - `sig-maker-gui_0.1.3-beta_x64-portable.zip`
   - `checksums.txt`
9. Click "Publish release"

## Example 2: Pre-release RC v0.2.0-rc.1

### Version Bump

```toml
[workspace.package]
version = "0.2.0-rc.1"
```

### Release Creation

Same as standard, but:
- Mark as "Pre-release" in GitHub
- Use `-rc.1` suffix
- Include "Testing Notes" in changelog:

```markdown
## [v0.2.0-rc.1] - 2026-06-12

### Features

- *(core)* Add new pattern optimization algorithm [[#50](https://github.com/JoShMiQueL/sig-maker/pull/50)] by @JoShMiQueL

### Testing Notes

This is a release candidate. Please test:
- Pattern conversion accuracy
- GUI performance
- Cross-platform compatibility

### Contributors

* @JoShMiQueL

**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/v0.1.3-beta...v0.2.0-rc.1
```

## Example 3: Rollback from Failed Release

### Scenario

Released v0.1.3-beta but GUI installer is broken.

### Rollback Steps

```bash
# 1. Delete git tag locally
git tag -d v0.1.3-beta

# 2. Delete git tag remotely
git push origin :refs/tags/v0.1.3-beta

# 3. Delete GitHub Release
# Go to GitHub → Releases → v0.1.3-beta → Delete release

# 4. Fix the issue
# (make code changes)

# 5. Create new release with incremented version
# Bump to v0.1.4-beta and follow standard release process
```

## Example 4: Multi-platform Release

### Build All CLI Binaries

```bash
# Windows x64 (native)
cargo build --release --bin sig-maker-cli

# Linux x64 (cross-compile)
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu --bin sig-maker-cli

# Linux ARM64 (cross-compile)
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu --bin sig-maker-cli

# macOS Intel (cross-compile)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin --bin sig-maker-cli

# macOS ARM64 (cross-compile)
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin --bin sig-maker-cli
```

### Build All GUI Installers

```bash
# Windows (native)
cd crates/sig-maker-gui
cargo tauri build

# Linux (native or build on Linux)
cd crates/sig-maker-gui
cargo tauri build

# macOS (native on Apple Silicon for universal)
cd crates/sig-maker-gui
cargo tauri build
```

### Upload All Artifacts

Upload these 9 files to GitHub Release:
- `sig-maker-cli.exe` (Windows x64)
- `sig-maker-cli` (Linux x64)
- `sig-maker-cli` (Linux ARM64)
- `sig-maker-cli` (macOS Intel)
- `sig-maker-cli` (macOS ARM64)
- `sig-maker-gui_0.1.3-beta_x64-setup.exe` (Windows)
- `sig-maker-gui_0.1.3-beta_amd64.AppImage` (Linux)
- `sig-maker-gui_0.1.3-beta_universal.dmg` (macOS)
- `checksums.txt` (all checksums)
