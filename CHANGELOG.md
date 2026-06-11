# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### CI/CD

- Only run tests when code changes by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Only run tests when code changes by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Simplify workflow and remove redundant steps by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Documentation

- [#29](https://github.com/JoShMiQueL/sig-maker/pull/29)Improve changelog format and simplify CI workflow (#29) by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Improve changelog format by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Features

- *(analyzer)* Support combining patterns with wildcards and raw AOBs by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Miscellaneous

- Fix formatting in .devin/config.json by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Refactor

- [#27](https://github.com/JoShMiQueL/sig-maker/pull/27)Simplify dependencies and improve CLI experience (#27) by [@JoShMiQueL](https://github.com/JoShMiQueL)



### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL) —,, [#29](https://github.com/JoShMiQueL/sig-maker/pull/29), [#27](https://github.com/JoShMiQueL/sig-maker/pull/27),,,,



## [0.1.1-beta] - 2026-06-10

### Bug Fixes

- *(tests)* Use debug binary in CI, release locally by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(hooks)* Explicitly run integration tests by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Pass admin token as input from workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Use find to locate git-cliff binary by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Remove [skip ci] from commit message by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(ci)* Run CI on all pull requests by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(ci)* Run CI on changelog branches by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Remove automatic trigger on push by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Create PR instead of direct push by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Use find to locate git-cliff binary by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Fix git-cliff installation in composite action by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(release)* Only run changelog job on tag pushes by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Fix gh commands in workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Improve workflow to prevent PR pile-up by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Add spacing between release sections by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Filter bots from New Contributors section in template by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Filter out bot accounts from contributors by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Fix template errors (deprecated fields, missing release) by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(ci)* Move skip rules before group rules in cliff.toml by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(ci)* Remove paths-ignore from changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(ci)* Improve changelog generation with links and anti-loop by [@JoShMiQueL](https://github.com/JoShMiQueL)


### CI/CD

- Update release.yml for cargo-dist compatibility by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update actions to latest versions by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Force Node.js 24 for GitHub Actions by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Add Rust toolchain cache and pin Rust version by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Add Rust toolchain cache and pin Rust version by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Use windows-latest instead of windows-2025-vs2026 by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Use GH_PAT secret for changelog PR creation by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Use GH_PAT secret for changelog PR creation by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Change changelog workflow to use PR with auto-merge by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Change changelog workflow to use PR with auto-merge by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Add Dependabot and Stale bot, add PR template by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Add git-cliff for automatic CHANGELOG.md generation by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Documentation

- Update TODO.md - mark fuzzing as complete by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update TODO.md - mark verbose and detailed stats as complete by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update TODO.md - mark pattern validation as complete by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update TODO.md - mark version and quiet as complete by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update TODO.md - mark output to file as complete by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update README with badges, releases, and documentation links by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update TODO.md - mark Repository Quality as complete by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Add Pull Request template by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Add AGENTS.md with project context for AI agents by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Document PR workflow and branch protection strategy by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Features

- *(changelog)* Add [skip ci] to commit message by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Add admin token for bypassing branch protection by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Create reusable action for changelog generation by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Add auto-merge without CI wait by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(release)* Add automatic changelog generation by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Add auto-merge to changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Add manual changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(release)* Integrate changelog generation into release workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(stats)* Add entropy and compression ratio to verbose output by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(cli)* Add terminal colors support by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(cli)* Add pattern validation (-c, --check) by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(cli)* Add version flag and quiet mode by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(cli)* Add output to file option (-o, --output) by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* List all PRs per contributor in Contributors section by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Add Contributors section and Full Changelog link by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Add New Contributors section by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(cli)* Show usage and pause on double-click launch by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Miscellaneous

- Remove TODO.md by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Remove automated changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Remove automated changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL)

- [#1](https://github.com/JoShMiQueL/sig-maker/pull/1)*(deps)* Bump actions/stale from 9 to 10 (#1) by [@dependabot[bot]](https://github.com/dependabot[bot])

- Add build check and commit-msg hook for Conventional Commits by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Add pre-commit git hooks (fmt, clippy, test) by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Refactor

- *(changelog)* Use secret directly in action by [@JoShMiQueL](https://github.com/JoShMiQueL)

- *(changelog)* Create composite action to reuse code by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Styling

- Add space between author link and commit hash in changelog by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Testing

- Add integration tests for CLI features by [@JoShMiQueL](https://github.com/JoShMiQueL)



### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL) —,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
* [@dependabot[bot]](https://github.com/dependabot[bot]) — [#1](https://github.com/JoShMiQueL/sig-maker/pull/1)



## [0.1.0-beta] - 2026-06-10

### CI/CD

- Migrate to cargo-dist for releases by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Upgrade changelog-builder to v6 and gh-release to v3 (Node.js 24) by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Remove Codecov coverage job by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Update GitHub Actions for Node.js 24 and Windows 2025 by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Fix

- Resolve clippy warnings and formatting issues by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Miscellaneous

- Set version to 0.1.0-beta by [@JoShMiQueL](https://github.com/JoShMiQueL)

- Remove temporary commit message file by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Styling

- Apply cargo fmt formatting by [@JoShMiQueL](https://github.com/JoShMiQueL)



### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL) —,,,,,,,




