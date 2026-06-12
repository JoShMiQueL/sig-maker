# Versioning

## Semantic Versioning

### Version Format: `MAJOR.MINOR.PATCH-PRERELEASE`

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes
- **PRERELEASE**: `-rc.1`, `-beta.1`, `-alpha.1` (optional)

### When to Increment

- **MAJOR**: Breaking changes to CLI API or core library
- **MINOR**: New features, new formats, new GUI capabilities
- **PATCH**: Bug fixes, documentation, CI improvements

## Pre-release Detection

### Detect Pre-release from Version

```bash
# Check if version contains pre-release identifier
if [[ "$VERSION" == *"-rc"* ]] || [[ "$VERSION" == *"-beta"* ]] || [[ "$VERSION" == *"-alpha"* ]]; then
  echo "Pre-release detected"
  IS_PRERELEASE=true
else
  echo "Stable release"
  IS_PRERELEASE=false
fi
```

### Pre-release Priority

- `v0.1.3-rc.1` > `v0.1.3-beta.1` > `v0.1.3-alpha.1`
- `v0.1.3-rc.2` > `v0.1.3-rc.1`

### Mark as Pre-release in GitHub

When creating GitHub Release:
- If version contains `-rc`, `-beta`, or `-alpha`: check "Set as a pre-release"
- If version has no suffix: leave unchecked (stable release)

## Pre-release Guidelines

### Alpha

- Early development, unstable
- For experimental features
- May have breaking changes
- Not for production use
- Version: `v0.1.3-alpha.1`

### Beta

- Feature complete, may have bugs
- For wider testing
- Should be relatively stable
- May have minor breaking changes
- Version: `v0.1.3-beta.1`

### Release Candidate (RC)

- Stable, ready for production
- Only critical bug fixes
- No breaking changes
- Final testing before stable release
- Version: `v0.1.3-rc.1`

### Stable

- Production-ready
- No breaking changes
- Full support
- Version: `v0.1.3`
