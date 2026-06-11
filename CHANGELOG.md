# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### CI/CD

- Fix cliff.toml template and filters by [@JoShMiQueL](https://github.com/JoShMiQueL) ([a8359d9](https://github.com/JoShMiQueL/sig-maker/commit/a8359d99e6713b4af1a16caf6635243829b2c96a))
- Only run tests when code changes by [@JoShMiQueL](https://github.com/JoShMiQueL) ([bd8e1ec](https://github.com/JoShMiQueL/sig-maker/commit/bd8e1ec19c5db4fe1a1fdf4620ca89ec4be5be5b))
- Simplify workflow and remove redundant steps by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6136261](https://github.com/JoShMiQueL/sig-maker/commit/6136261aaa18fdb0f33f73a08879375c56debb51))

### Features

- *(analyzer)* Support combining patterns with wildcards and raw AOBs by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8ecbcde](https://github.com/JoShMiQueL/sig-maker/commit/8ecbcde6f1a2d778db3f36569f7e8145d78dd321))

### Miscellaneous

- Fix formatting in .devin/config.json by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6eb5482](https://github.com/JoShMiQueL/sig-maker/commit/6eb5482d0bfaec80226eaccfa49caa3d2b90beae))


### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL)

## [0.1.1-beta] - 2026-06-10

### Bug Fixes

- *(tests)* Use debug binary in CI, release locally by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ac70afb](https://github.com/JoShMiQueL/sig-maker/commit/ac70afb8a7c8a529eb8b21e2b3e2896cc8eb9c1c))
- *(hooks)* Explicitly run integration tests by [@JoShMiQueL](https://github.com/JoShMiQueL) ([af16fa2](https://github.com/JoShMiQueL/sig-maker/commit/af16fa27fae4e866b88794ce31633c95777a2784))
- *(ci)* Move skip rules before group rules in cliff.toml by [@JoShMiQueL](https://github.com/JoShMiQueL) ([16fc195](https://github.com/JoShMiQueL/sig-maker/commit/16fc1953de99f18fc7200d1717f4e5b487273662))

### CI/CD

- Update actions to latest versions by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8c03fdb](https://github.com/JoShMiQueL/sig-maker/commit/8c03fdb7142b99874a6d20f00485efba50cb23ab))
- Force Node.js 24 for GitHub Actions by [@JoShMiQueL](https://github.com/JoShMiQueL) ([79b60ed](https://github.com/JoShMiQueL/sig-maker/commit/79b60ede9160d2a6883dbbef239ce2c2e54b7dca))
- Add Rust toolchain cache and pin Rust version by [@JoShMiQueL](https://github.com/JoShMiQueL) ([a214e6b](https://github.com/JoShMiQueL/sig-maker/commit/a214e6b2787fe0123118073446b619b5f0659afe))
- Add Rust toolchain cache and pin Rust version by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2829ca1](https://github.com/JoShMiQueL/sig-maker/commit/2829ca1813bb9108b8d18b2a04be6fa4ce2ad3e3))
- Use windows-latest instead of windows-2025-vs2026 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([084dd27](https://github.com/JoShMiQueL/sig-maker/commit/084dd2726325b2144fd205c8973e12e6fa9bec5f))
- Add Dependabot and Stale bot, add PR template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([65c1952](https://github.com/JoShMiQueL/sig-maker/commit/65c1952269e7caf916d4f407eeda9f888c7881f0))

### Documentation

- Update README with badges, releases, and documentation links by [@JoShMiQueL](https://github.com/JoShMiQueL) ([56aad26](https://github.com/JoShMiQueL/sig-maker/commit/56aad260e341368dbf65b1dd02ff3fe9ba08198d))
- Add Pull Request template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([99b3b7a](https://github.com/JoShMiQueL/sig-maker/commit/99b3b7ab7a5a8f0f0a33182204fd232e92e450ad))
- Add AGENTS.md with project context for AI agents by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ef6932f](https://github.com/JoShMiQueL/sig-maker/commit/ef6932fa7be2c5228513e8162cce741a397d3f7f))
- Document PR workflow and branch protection strategy by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6fc6014](https://github.com/JoShMiQueL/sig-maker/commit/6fc6014bb88ea62087b5982f964dc83b31d7fdb1))

### Features

- *(stats)* Add entropy and compression ratio to verbose output by [@JoShMiQueL](https://github.com/JoShMiQueL) ([dc55d9b](https://github.com/JoShMiQueL/sig-maker/commit/dc55d9b7cfd3f89897731533ea00a1d52b358af5))
- *(cli)* Add terminal colors support by [@JoShMiQueL](https://github.com/JoShMiQueL) ([514a52d](https://github.com/JoShMiQueL/sig-maker/commit/514a52d1721c4814ac66c6d7c59fa9a2262f808f))
- *(cli)* Add pattern validation (-c, --check) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2e41376](https://github.com/JoShMiQueL/sig-maker/commit/2e41376315969ac0aec4c9a35fe56ac723829741))
- *(cli)* Add version flag and quiet mode by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f6f5d92](https://github.com/JoShMiQueL/sig-maker/commit/f6f5d92e5efd76c6f2370e6bdcb1aab0bfcd9c2e))
- *(cli)* Add output to file option (-o, --output) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f7a6d87](https://github.com/JoShMiQueL/sig-maker/commit/f7a6d8783de07189c2e4bb4781c25bf5c6e7059a))
- *(cli)* Show usage and pause on double-click launch by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0845ab1](https://github.com/JoShMiQueL/sig-maker/commit/0845ab18d8cf7a9f13ea1dad3fea789549c6f6cb))

### Miscellaneous

- Add build check and commit-msg hook for Conventional Commits by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5eabffb](https://github.com/JoShMiQueL/sig-maker/commit/5eabffbe7077ddf6499b209ba6f735e4728f9be9))
- Add pre-commit git hooks (fmt, clippy, test) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([4e3e2c3](https://github.com/JoShMiQueL/sig-maker/commit/4e3e2c3597eba1167a3dff96b9fa62b7d2c679bb))

### Testing

- Add integration tests for CLI features by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3f7514d](https://github.com/JoShMiQueL/sig-maker/commit/3f7514d7028b8cc89adc734f9ba83b882bba9acd))


### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL)

## [0.1.0-beta] - 2026-06-10

### Bug Fixes

- Resolve clippy warnings and formatting issues by [@JoShMiQueL](https://github.com/JoShMiQueL) ([07f7d8f](https://github.com/JoShMiQueL/sig-maker/commit/07f7d8f7fbe4282e7909cfbab0558201e8828a9b))

### CI/CD

- Remove Codecov coverage job by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f55d68f](https://github.com/JoShMiQueL/sig-maker/commit/f55d68f9b1e3ca67f1bc070429d2fed76d6ca6ba))
- Update GitHub Actions for Node.js 24 and Windows 2025 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1ba2088](https://github.com/JoShMiQueL/sig-maker/commit/1ba2088a9b31ad246ebd5ccb9d05a59c356393fc))

### Miscellaneous

- Set version to 0.1.0-beta by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d1750c2](https://github.com/JoShMiQueL/sig-maker/commit/d1750c26bd73bb440c54c0c51072b64704cbbb20))
- Remove temporary commit message file by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1251f75](https://github.com/JoShMiQueL/sig-maker/commit/1251f75a70e9e704baf6313f248a6442baf0a996))

### Styling

- Apply cargo fmt formatting by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d0caf09](https://github.com/JoShMiQueL/sig-maker/commit/d0caf093176114c73bfee47606dcac6a97cb3298))


### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL)


