# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### CI/CD

- Add commit hash link for non-PR commits ([bca3505](https://github.com/JoShMiQueL/sig-maker/commit/bca3505e697258fe5fdfeaa21e34019110b58d7b)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Improve changelog format and remove redundant links ([d3f5b5e](https://github.com/JoShMiQueL/sig-maker/commit/d3f5b5e9d4451027fe92dca1544cfca3941ac299)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Fix cliff.toml template and filters ([a8359d9](https://github.com/JoShMiQueL/sig-maker/commit/a8359d99e6713b4af1a16caf6635243829b2c96a)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Improve changelog filtering in cliff.toml ([2e684e1](https://github.com/JoShMiQueL/sig-maker/commit/2e684e14525a2a784be95b5e117cc8d9098b59ae)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Only run tests when code changes ([bd8e1ec](https://github.com/JoShMiQueL/sig-maker/commit/bd8e1ec19c5db4fe1a1fdf4620ca89ec4be5be5b)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Only run tests when code changes ([af1a27e](https://github.com/JoShMiQueL/sig-maker/commit/af1a27e12a5e947e7d560bc0436789c43f42221f)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Simplify workflow and remove redundant steps ([6136261](https://github.com/JoShMiQueL/sig-maker/commit/6136261aaa18fdb0f33f73a08879375c56debb51)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Documentation

- Resolve merge conflict in CHANGELOG.md ([da7903d](https://github.com/JoShMiQueL/sig-maker/commit/da7903d46843b6761c7c194686dbc12e84a24e6f)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Improve changelog filtering and formatting ([06fa2b6](https://github.com/JoShMiQueL/sig-maker/commit/06fa2b65ca4521e4a2e0d44c204b3ff8d5d32c2f)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Improve changelog format and simplify CI workflow ([#29](https://github.com/JoShMiQueL/sig-maker/pull/29)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Improve changelog format ([7c9f186](https://github.com/JoShMiQueL/sig-maker/commit/7c9f186f7107895ba540dbf3ad8bd080718692e0)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Features

- *(analyzer)* Support combining patterns with wildcards and raw AOBs ([8ecbcde](https://github.com/JoShMiQueL/sig-maker/commit/8ecbcde6f1a2d778db3f36569f7e8145d78dd321)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Miscellaneous

- Fix formatting in .devin/config.json ([6eb5482](https://github.com/JoShMiQueL/sig-maker/commit/6eb5482d0bfaec80226eaccfa49caa3d2b90beae)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Refactor

- Simplify dependencies and improve CLI experience ([#27](https://github.com/JoShMiQueL/sig-maker/pull/27)) by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL)

## [0.1.1-beta] - 2026-06-10

### Bug Fixes

- *(tests)* Use debug binary in CI, release locally ([ac70afb](https://github.com/JoShMiQueL/sig-maker/commit/ac70afb8a7c8a529eb8b21e2b3e2896cc8eb9c1c)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(hooks)* Explicitly run integration tests ([af16fa2](https://github.com/JoShMiQueL/sig-maker/commit/af16fa27fae4e866b88794ce31633c95777a2784)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(ci)* Run CI on all pull requests ([1909350](https://github.com/JoShMiQueL/sig-maker/commit/1909350e15e717059c0dbb8f1938f2fcaaf8fdaf)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(ci)* Run CI on changelog branches ([3c723e9](https://github.com/JoShMiQueL/sig-maker/commit/3c723e9ecaf85ab48e09ab0db71155da8ab8a8f6)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(release)* Only run changelog job on tag pushes ([09682cf](https://github.com/JoShMiQueL/sig-maker/commit/09682cf19fa909dbccb7d98ff0e0c76592e68f9d)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(ci)* Move skip rules before group rules in cliff.toml ([16fc195](https://github.com/JoShMiQueL/sig-maker/commit/16fc1953de99f18fc7200d1717f4e5b487273662)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(ci)* Remove paths-ignore from changelog workflow ([2f1b68d](https://github.com/JoShMiQueL/sig-maker/commit/2f1b68d94a1f14e30035ab569305f323a40241cb)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(ci)* Improve changelog generation with links and anti-loop ([0fbe551](https://github.com/JoShMiQueL/sig-maker/commit/0fbe551e9814049e7bb93bb533324340921512f1)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### CI/CD

- Update release.yml for cargo-dist compatibility ([7a204d8](https://github.com/JoShMiQueL/sig-maker/commit/7a204d8a0890ad6cf2418c5a5b4678bf5b9d0ee0)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update actions to latest versions ([8c03fdb](https://github.com/JoShMiQueL/sig-maker/commit/8c03fdb7142b99874a6d20f00485efba50cb23ab)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Force Node.js 24 for GitHub Actions ([79b60ed](https://github.com/JoShMiQueL/sig-maker/commit/79b60ede9160d2a6883dbbef239ce2c2e54b7dca)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add Rust toolchain cache and pin Rust version ([a214e6b](https://github.com/JoShMiQueL/sig-maker/commit/a214e6b2787fe0123118073446b619b5f0659afe)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add Rust toolchain cache and pin Rust version ([2829ca1](https://github.com/JoShMiQueL/sig-maker/commit/2829ca1813bb9108b8d18b2a04be6fa4ce2ad3e3)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Use windows-latest instead of windows-2025-vs2026 ([084dd27](https://github.com/JoShMiQueL/sig-maker/commit/084dd2726325b2144fd205c8973e12e6fa9bec5f)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Use GH_PAT secret for changelog PR creation ([81addb5](https://github.com/JoShMiQueL/sig-maker/commit/81addb5f99a8d2a5e8d290d91d9f297f8c576c3d)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Use GH_PAT secret for changelog PR creation ([0ab8b7c](https://github.com/JoShMiQueL/sig-maker/commit/0ab8b7c613bb72ec44ee01769976418e1669c201)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Change changelog workflow to use PR with auto-merge ([0242063](https://github.com/JoShMiQueL/sig-maker/commit/024206355fb54ccdcc83602d79c4e42ac2e0dd6e)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Change changelog workflow to use PR with auto-merge ([f90336d](https://github.com/JoShMiQueL/sig-maker/commit/f90336d74639c75bc60e4e1cbc1ed6f1b06ded02)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add Dependabot and Stale bot, add PR template ([65c1952](https://github.com/JoShMiQueL/sig-maker/commit/65c1952269e7caf916d4f407eeda9f888c7881f0)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add git-cliff for automatic CHANGELOG.md generation ([1e862e6](https://github.com/JoShMiQueL/sig-maker/commit/1e862e68a2869c128b5b26f5711cbdc1048d536d)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Documentation

- Update TODO.md - mark fuzzing as complete ([f0b4e86](https://github.com/JoShMiQueL/sig-maker/commit/f0b4e86c054923bc4d74bd06ed3a1b8fbcb199fc)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update TODO.md - mark verbose and detailed stats as complete ([f6e354d](https://github.com/JoShMiQueL/sig-maker/commit/f6e354dd0c671a4e2b03c87fefdc472fe8c23519)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update TODO.md - mark pattern validation as complete ([9ff390a](https://github.com/JoShMiQueL/sig-maker/commit/9ff390ae970610240276cb638c9fcadf80ca1481)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update TODO.md - mark version and quiet as complete ([a69e9ed](https://github.com/JoShMiQueL/sig-maker/commit/a69e9edbb33278a6d0175e396ab3f24f3c0931db)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update TODO.md - mark output to file as complete ([64a916f](https://github.com/JoShMiQueL/sig-maker/commit/64a916f2fd309234010df785ef2427a9dd7a8b3e)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update README with badges, releases, and documentation links ([56aad26](https://github.com/JoShMiQueL/sig-maker/commit/56aad260e341368dbf65b1dd02ff3fe9ba08198d)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update TODO.md - mark Repository Quality as complete ([ead45fd](https://github.com/JoShMiQueL/sig-maker/commit/ead45fd098a4efeec3294ec8b4fd41bcb863d1a8)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add Pull Request template ([99b3b7a](https://github.com/JoShMiQueL/sig-maker/commit/99b3b7ab7a5a8f0f0a33182204fd232e92e450ad)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add AGENTS.md with project context for AI agents ([ef6932f](https://github.com/JoShMiQueL/sig-maker/commit/ef6932fa7be2c5228513e8162cce741a397d3f7f)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Document PR workflow and branch protection strategy ([6fc6014](https://github.com/JoShMiQueL/sig-maker/commit/6fc6014bb88ea62087b5982f964dc83b31d7fdb1)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Features

- *(release)* Add automatic changelog generation ([ffd36a5](https://github.com/JoShMiQueL/sig-maker/commit/ffd36a507e593804c97cf5b30949f7f47738bf11)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(release)* Integrate changelog generation into release workflow ([dd1c6ad](https://github.com/JoShMiQueL/sig-maker/commit/dd1c6adf8ac065359a58d5a097a12499f6d318df)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(stats)* Add entropy and compression ratio to verbose output ([dc55d9b](https://github.com/JoShMiQueL/sig-maker/commit/dc55d9b7cfd3f89897731533ea00a1d52b358af5)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(cli)* Add terminal colors support ([514a52d](https://github.com/JoShMiQueL/sig-maker/commit/514a52d1721c4814ac66c6d7c59fa9a2262f808f)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(cli)* Add pattern validation (-c, --check) ([2e41376](https://github.com/JoShMiQueL/sig-maker/commit/2e41376315969ac0aec4c9a35fe56ac723829741)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(cli)* Add version flag and quiet mode ([f6f5d92](https://github.com/JoShMiQueL/sig-maker/commit/f6f5d92e5efd76c6f2370e6bdcb1aab0bfcd9c2e)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(cli)* Add output to file option (-o, --output) ([f7a6d87](https://github.com/JoShMiQueL/sig-maker/commit/f7a6d8783de07189c2e4bb4781c25bf5c6e7059a)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- *(cli)* Show usage and pause on double-click launch ([0845ab1](https://github.com/JoShMiQueL/sig-maker/commit/0845ab18d8cf7a9f13ea1dad3fea789549c6f6cb)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Miscellaneous

- Remove TODO.md ([d33902f](https://github.com/JoShMiQueL/sig-maker/commit/d33902fb45a12a998d7b0c9cba58bc1c583d328c)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Remove automated changelog workflow ([67ae097](https://github.com/JoShMiQueL/sig-maker/commit/67ae09726ec0d5fb419d96ed2517d37988ac997f)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Remove automated changelog workflow ([601aa56](https://github.com/JoShMiQueL/sig-maker/commit/601aa56ef084eed68a7aef36f7b42b7f9bf850cd)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add build check and commit-msg hook for Conventional Commits ([5eabffb](https://github.com/JoShMiQueL/sig-maker/commit/5eabffbe7077ddf6499b209ba6f735e4728f9be9)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Add pre-commit git hooks (fmt, clippy, test) ([4e3e2c3](https://github.com/JoShMiQueL/sig-maker/commit/4e3e2c3597eba1167a3dff96b9fa62b7d2c679bb)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Styling

- Add space between author link and commit hash in changelog ([0e2fa51](https://github.com/JoShMiQueL/sig-maker/commit/0e2fa51dcac0683c19a0f0fe339f5c48767da475)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Testing

- Add integration tests for CLI features ([3f7514d](https://github.com/JoShMiQueL/sig-maker/commit/3f7514d7028b8cc89adc734f9ba83b882bba9acd)) by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL)

## [0.1.0-beta] - 2026-06-10

### Bug Fixes

- Resolve clippy warnings and formatting issues ([07f7d8f](https://github.com/JoShMiQueL/sig-maker/commit/07f7d8f7fbe4282e7909cfbab0558201e8828a9b)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### CI/CD

- Migrate to cargo-dist for releases ([f7f11d1](https://github.com/JoShMiQueL/sig-maker/commit/f7f11d16d58213fe16f8386bd8e8b4c3debdf415)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Upgrade changelog-builder to v6 and gh-release to v3 (Node.js 24) ([8f073ea](https://github.com/JoShMiQueL/sig-maker/commit/8f073ea2ff6c0c92def7140af2691fc33273d464)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Remove Codecov coverage job ([f55d68f](https://github.com/JoShMiQueL/sig-maker/commit/f55d68f9b1e3ca67f1bc070429d2fed76d6ca6ba)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Update GitHub Actions for Node.js 24 and Windows 2025 ([1ba2088](https://github.com/JoShMiQueL/sig-maker/commit/1ba2088a9b31ad246ebd5ccb9d05a59c356393fc)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Miscellaneous

- Set version to 0.1.0-beta ([d1750c2](https://github.com/JoShMiQueL/sig-maker/commit/d1750c26bd73bb440c54c0c51072b64704cbbb20)) by [@JoShMiQueL](https://github.com/JoShMiQueL)
- Remove temporary commit message file ([1251f75](https://github.com/JoShMiQueL/sig-maker/commit/1251f75a70e9e704baf6313f248a6442baf0a996)) by [@JoShMiQueL](https://github.com/JoShMiQueL)

### Styling

- Apply cargo fmt formatting ([d0caf09](https://github.com/JoShMiQueL/sig-maker/commit/d0caf093176114c73bfee47606dcac6a97cb3298)) by [@JoShMiQueL](https://github.com/JoShMiQueL)


### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL)


