# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Bug Fixes



- **cli:** serialize env-var tests with mutex to prevent race conditions by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6055174](https://github.com/JoShMiQueL/sig-maker/commit/6055174e3f1a054d10d1c62e29aea6279a646631))


- **gui:** clear beforeBuildCommand to prevent double frontend build in CI by [@JoShMiQueL](https://github.com/JoShMiQueL) ([62ca44b](https://github.com/JoShMiQueL/sig-maker/commit/62ca44bc8935d784a2ac1fafad874aa5fd3a2e1a))


- **ci:** pin tauri-cli to 2.11.2 to fix E0119 compile error by [@JoShMiQueL](https://github.com/JoShMiQueL) ([59aa7af](https://github.com/JoShMiQueL/sig-maker/commit/59aa7af4c220966f68ec8c25e461a76b029f3526))


- **ci:** use npm @tauri-apps/cli instead of cargo install tauri-cli by [@JoShMiQueL](https://github.com/JoShMiQueL) ([c022864](https://github.com/JoShMiQueL/sig-maker/commit/c022864a29c6a8b1314562f50cdb92646c0b6b89))


- **ci:** pin rustc to 1.95.0 for GUI builds to avoid tauri-utils E0119 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([9dc2907](https://github.com/JoShMiQueL/sig-maker/commit/9dc2907a7a6dd69f9a4cae86ffec26caf6a1a6b0))


- **ci:** use separate --bundles flags for nsis and zip on Windows by [@JoShMiQueL](https://github.com/JoShMiQueL) ([46e6e44](https://github.com/JoShMiQueL/sig-maker/commit/46e6e44825268bbc14be1556d2e863c77df4d7be))


- **deps:** pin tauri-utils to 2.8.3 and time to 0.3.46 to avoid E0119 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([c4c7906](https://github.com/JoShMiQueL/sig-maker/commit/c4c79060962587055ee5ddbc1d85811cb52ce34a))

### CI/CD



- **release:** wait for CI to pass before releasing by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3ba72b7](https://github.com/JoShMiQueL/sig-maker/commit/3ba72b71fec205984475378ee7b566384b4d6adf))


- **release:** auto-bump Cargo.toml and tauri.conf.json from tag version by [@JoShMiQueL](https://github.com/JoShMiQueL) ([b8e3f9f](https://github.com/JoShMiQueL/sig-maker/commit/b8e3f9fa8c9ba17d021869893ec7e131018f508e))


- **release:** build GUI for Windows only until tauri-utils#15525 is fixed by [@JoShMiQueL](https://github.com/JoShMiQueL) ([72b6a1f](https://github.com/JoShMiQueL/sig-maker/commit/72b6a1f39b7bad7c352d4853bd5b0d35ae01278d))


- **release:** add portable ZIP bundle for Windows GUI by [@JoShMiQueL](https://github.com/JoShMiQueL) ([4e7eb6a](https://github.com/JoShMiQueL/sig-maker/commit/4e7eb6a854274ad9f3592c0b7eae1012cb2b254c))


- **release:** restore all-platform GUI builds + optimize cache by [@JoShMiQueL](https://github.com/JoShMiQueL) ([55023de](https://github.com/JoShMiQueL/sig-maker/commit/55023de810717b105cd288e423d4f151ea1053a3))

### Documentation



- update AGENTS.md to reflect current GUI build status by [@JoShMiQueL](https://github.com/JoShMiQueL) ([fc4196a](https://github.com/JoShMiQueL/sig-maker/commit/fc4196a4a7af305131b161e0d43a10ff139185c0))

### Features



- **install:** add install.sh and install.ps1 one-liner installers by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1c3eef2](https://github.com/JoShMiQueL/sig-maker/commit/1c3eef2c2db3d9b3f62ac79b9250ff1f8c797b1c))

### Miscellaneous



- **release:** bump version to 0.1.3-beta by [@JoShMiQueL](https://github.com/JoShMiQueL) ([c04bcec](https://github.com/JoShMiQueL/sig-maker/commit/c04bcecfe16baf9148ea01ec68a4f87a8cc6dac8))




### Contributors

- [@JoShMiQueL](https://github.com/JoShMiQueL)

## [v0.1.2-beta-test](https://github.com/JoShMiQueL/sig-maker/releases/tag/v0.1.2-beta-test) - 2026-06-12

### Bug Fixes



- **cli:** handle empty input in test mode. Add early return for empty input in test mode. Prevents parse error and pause in tests. Fixes CI test failure on Linux by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3a82f71](https://github.com/JoShMiQueL/sig-maker/commit/3a82f712f292b27f693c40e19252295f4c1bbd75))


- **cli:** remove error print in test mode. Remove eprintln from parse error in test mode to prevent CI failures by [@JoShMiQueL](https://github.com/JoShMiQueL) ([47abf40](https://github.com/JoShMiQueL/sig-maker/commit/47abf4048f713b8061a298bdd9a62b3a092f35aa))

### CI/CD



- simplify workflow and remove redundant steps by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6136261](https://github.com/JoShMiQueL/sig-maker/commit/6136261aaa18fdb0f33f73a08879375c56debb51))


- only run tests when code changes by [@JoShMiQueL](https://github.com/JoShMiQueL) ([af1a27e](https://github.com/JoShMiQueL/sig-maker/commit/af1a27e12a5e947e7d560bc0436789c43f42221f))


- only run tests when code changes by [@JoShMiQueL](https://github.com/JoShMiQueL) ([bd8e1ec](https://github.com/JoShMiQueL/sig-maker/commit/bd8e1ec19c5db4fe1a1fdf4620ca89ec4be5be5b))


- improve changelog filtering in cliff.toml by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2e684e1](https://github.com/JoShMiQueL/sig-maker/commit/2e684e14525a2a784be95b5e117cc8d9098b59ae))


- fix cliff.toml template and filters by [@JoShMiQueL](https://github.com/JoShMiQueL) ([a8359d9](https://github.com/JoShMiQueL/sig-maker/commit/a8359d99e6713b4af1a16caf6635243829b2c96a))


- add commit links and filter all changelog-scope commits by [@JoShMiQueL](https://github.com/JoShMiQueL) ([7f509a2](https://github.com/JoShMiQueL/sig-maker/commit/7f509a279523e395d6ef6f8ca7be811653d92db9))


- rebalance commit_parsers filters by [@JoShMiQueL](https://github.com/JoShMiQueL) ([be242e5](https://github.com/JoShMiQueL/sig-maker/commit/be242e585a59bc05e80a99f1dd4b91987f8e8922))


- improve changelog format and remove redundant links by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d3f5b5e](https://github.com/JoShMiQueL/sig-maker/commit/d3f5b5e9d4451027fe92dca1544cfca3941ac299))


- add commit hash link for non-PR commits by [@JoShMiQueL](https://github.com/JoShMiQueL) ([bca3505](https://github.com/JoShMiQueL/sig-maker/commit/bca3505e697258fe5fdfeaa21e34019110b58d7b))


- ignore non-code changes in CI trigger by [@JoShMiQueL](https://github.com/JoShMiQueL) ([a20b730](https://github.com/JoShMiQueL/sig-maker/commit/a20b730b8970a740bf0e171d90a63cb4f59d1fa0))


- add workflow_dispatch and fix Windows test pattern by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5fa90b1](https://github.com/JoShMiQueL/sig-maker/commit/5fa90b116fbba3ab6816b979c45033bea51a6bdb))


- remove GUI tests from CI by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ff2551e](https://github.com/JoShMiQueL/sig-maker/commit/ff2551eecbed671c29c59e17186cdbcdbd7aab5a))


- use windows-2025-vs2026 to avoid redirect notice by [@JoShMiQueL](https://github.com/JoShMiQueL) ([894e0bf](https://github.com/JoShMiQueL/sig-maker/commit/894e0bf7a3c4de541260f3190a030cf7f9160003))


- remove GUI tests and use windows-2025-vs2026 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([762b9df](https://github.com/JoShMiQueL/sig-maker/commit/762b9dfa70e92cda739e97834c52f32d484489a7))


- add release and changelog workflows by [@JoShMiQueL](https://github.com/JoShMiQueL) ([c29d5bd](https://github.com/JoShMiQueL/sig-maker/commit/c29d5bd2ed27b01e69465498ef603529a8c40514))


- **changelog:** use GH_PAT to bypass branch protection on push by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3b54a02](https://github.com/JoShMiQueL/sig-maker/commit/3b54a0221ca4ff940386c31ed5aa7babff433cd9))


- **changelog:** add New Contributors section and fix header/footer split by [@JoShMiQueL](https://github.com/JoShMiQueL) ([63ad40a](https://github.com/JoShMiQueL/sig-maker/commit/63ad40a74296bf9b36280912a9f2d0a833a02ecf))


- **changelog:** hide New Contributors section when empty by [@JoShMiQueL](https://github.com/JoShMiQueL) ([856c3fb](https://github.com/JoShMiQueL/sig-maker/commit/856c3fb16fb016ab78e8a7569fac53d592ba9762))


- **changelog:** link version header to GitHub release tag by [@JoShMiQueL](https://github.com/JoShMiQueL) ([b90c12e](https://github.com/JoShMiQueL/sig-maker/commit/b90c12ef5385c72c755e880921bc4671b9706b33))


- **changelog:** reorder commit format to 'by @author (hash)' by [@JoShMiQueL](https://github.com/JoShMiQueL) ([450f07f](https://github.com/JoShMiQueL/sig-maker/commit/450f07f9b42bc7835864c777413e2c09cddaafaa))


- **changelog:** move PR link after author in commit format by [@JoShMiQueL](https://github.com/JoShMiQueL) ([cdf7f37](https://github.com/JoShMiQueL/sig-maker/commit/cdf7f37106109216edb15208ea1edf4cfcd16ce3))

### Documentation



- improve changelog format by [@JoShMiQueL](https://github.com/JoShMiQueL) ([7c9f186](https://github.com/JoShMiQueL/sig-maker/commit/7c9f186f7107895ba540dbf3ad8bd080718692e0))


- improve changelog format and simplify CI workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) [[#29](https://github.com/JoShMiQueL/sig-maker/pull/29)]


- improve changelog filtering and formatting by [@JoShMiQueL](https://github.com/JoShMiQueL) ([06fa2b6](https://github.com/JoShMiQueL/sig-maker/commit/06fa2b65ca4521e4a2e0d44c204b3ff8d5d32c2f))


- resolve merge conflict in CHANGELOG.md by [@JoShMiQueL](https://github.com/JoShMiQueL) ([da7903d](https://github.com/JoShMiQueL/sig-maker/commit/da7903d46843b6761c7c194686dbc12e84a24e6f))

### Features



- **analyzer:** support combining patterns with wildcards and raw AOBs by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8ecbcde](https://github.com/JoShMiQueL/sig-maker/commit/8ecbcde6f1a2d778db3f36569f7e8145d78dd321))


- **gui:** add Tauri GUI with automated release workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) [[#37](https://github.com/JoShMiQueL/sig-maker/pull/37)]


- **release:** add release skill and optimize CI paths by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2958b66](https://github.com/JoShMiQueL/sig-maker/commit/2958b66043dfe3c202faf4e3a8e6beb643abe540))

### Miscellaneous



- fix formatting in .devin/config.json by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6eb5482](https://github.com/JoShMiQueL/sig-maker/commit/6eb5482d0bfaec80226eaccfa49caa3d2b90beae))


- remove automated release workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([98bb3aa](https://github.com/JoShMiQueL/sig-maker/commit/98bb3aa25e3753ec26caa7b602460d9ba6d9fd96))


- remove release automation scripts by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8e9c751](https://github.com/JoShMiQueL/sig-maker/commit/8e9c751171bba8b5ed5f9aff8b1fd1f08ead03b9))


- bump version to 0.1.2-beta-test by [@JoShMiQueL](https://github.com/JoShMiQueL) ([edfe924](https://github.com/JoShMiQueL/sig-maker/commit/edfe924317e6cd2a32dc3c04e9e208d395f97947))

### Refactoring



- simplify dependencies and improve CLI experience by [@JoShMiQueL](https://github.com/JoShMiQueL) [[#27](https://github.com/JoShMiQueL/sig-maker/pull/27)]




### Contributors

- [@JoShMiQueL](https://github.com/JoShMiQueL)

**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/v0.1.1-beta...v0.1.2-beta-test
## [v0.1.1-beta](https://github.com/JoShMiQueL/sig-maker/releases/tag/v0.1.1-beta) - 2026-06-10

### Bug Fixes



- **ci:** Improve changelog generation with links and anti-loop by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0fbe551](https://github.com/JoShMiQueL/sig-maker/commit/0fbe551e9814049e7bb93bb533324340921512f1))


- **ci:** Remove paths-ignore from changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2f1b68d](https://github.com/JoShMiQueL/sig-maker/commit/2f1b68d94a1f14e30035ab569305f323a40241cb))


- **ci:** Move skip rules before group rules in cliff.toml by [@JoShMiQueL](https://github.com/JoShMiQueL) ([16fc195](https://github.com/JoShMiQueL/sig-maker/commit/16fc1953de99f18fc7200d1717f4e5b487273662))


- **changelog:** Fix template errors (deprecated fields, missing release) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([758f338](https://github.com/JoShMiQueL/sig-maker/commit/758f338173433ce20afefdef45af5a800ae20af1))


- **changelog:** Filter out bot accounts from contributors by [@JoShMiQueL](https://github.com/JoShMiQueL) ([df74b97](https://github.com/JoShMiQueL/sig-maker/commit/df74b97379e9a43098cec0193fb64db0879e9e90))


- **changelog:** Filter bots from New Contributors section in template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f598cb8](https://github.com/JoShMiQueL/sig-maker/commit/f598cb8fd92125741266e1cfa671278d41ec57c9))


- **changelog:** Add spacing between release sections by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1f54d25](https://github.com/JoShMiQueL/sig-maker/commit/1f54d25d3ac67db457d1371bcbcd454d42bfb115))


- **changelog:** Improve workflow to prevent PR pile-up by [@JoShMiQueL](https://github.com/JoShMiQueL) ([4293a8e](https://github.com/JoShMiQueL/sig-maker/commit/4293a8e2013a0ab6df8798d95f46967ead9ecdc0))


- **changelog:** Fix gh commands in workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d463db0](https://github.com/JoShMiQueL/sig-maker/commit/d463db02d0bf652116992bd1c97dcf4ae0a6a583))


- **release:** Only run changelog job on tag pushes by [@JoShMiQueL](https://github.com/JoShMiQueL) ([09682cf](https://github.com/JoShMiQueL/sig-maker/commit/09682cf19fa909dbccb7d98ff0e0c76592e68f9d))


- **changelog:** Fix git-cliff installation in composite action by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d73632d](https://github.com/JoShMiQueL/sig-maker/commit/d73632d9a2a85a8a3df9a3e40e45d2daead330fc))


- **changelog:** Use find to locate git-cliff binary by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1fe81c1](https://github.com/JoShMiQueL/sig-maker/commit/1fe81c16a8b94f13cbbf8849dc19dfdfdefec213))


- **changelog:** Create PR instead of direct push by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1933b1b](https://github.com/JoShMiQueL/sig-maker/commit/1933b1b7b1fdd2e5efd34b75fb050a2ae852ae43))


- **changelog:** Remove automatic trigger on push by [@JoShMiQueL](https://github.com/JoShMiQueL) ([cb34db7](https://github.com/JoShMiQueL/sig-maker/commit/cb34db79fcae6a045a67632277c3a7668daf08fb))


- **ci:** Run CI on changelog branches by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3c723e9](https://github.com/JoShMiQueL/sig-maker/commit/3c723e9ecaf85ab48e09ab0db71155da8ab8a8f6))


- **ci:** Run CI on all pull requests by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1909350](https://github.com/JoShMiQueL/sig-maker/commit/1909350e15e717059c0dbb8f1938f2fcaaf8fdaf))


- **changelog:** Remove [skip ci] from commit message by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0cd8d1d](https://github.com/JoShMiQueL/sig-maker/commit/0cd8d1d15995e87979100084e8f6416d49a5e7a3))


- **changelog:** Use find to locate git-cliff binary by [@JoShMiQueL](https://github.com/JoShMiQueL) ([be3344a](https://github.com/JoShMiQueL/sig-maker/commit/be3344afc58272edae5359428dff22b221430037))


- **changelog:** Pass admin token as input from workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([9d4c68e](https://github.com/JoShMiQueL/sig-maker/commit/9d4c68e69bde50fb06013e11ae72a17cf6f6a701))


- **hooks:** Explicitly run integration tests by [@JoShMiQueL](https://github.com/JoShMiQueL) ([af16fa2](https://github.com/JoShMiQueL/sig-maker/commit/af16fa27fae4e866b88794ce31633c95777a2784))


- **tests:** Use debug binary in CI, release locally by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ac70afb](https://github.com/JoShMiQueL/sig-maker/commit/ac70afb8a7c8a529eb8b21e2b3e2896cc8eb9c1c))

### CI/CD



- Add git-cliff for automatic CHANGELOG.md generation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1e862e6](https://github.com/JoShMiQueL/sig-maker/commit/1e862e68a2869c128b5b26f5711cbdc1048d536d))


- Add Dependabot and Stale bot, add PR template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([65c1952](https://github.com/JoShMiQueL/sig-maker/commit/65c1952269e7caf916d4f407eeda9f888c7881f0))


- Change changelog workflow to use PR with auto-merge by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f90336d](https://github.com/JoShMiQueL/sig-maker/commit/f90336d74639c75bc60e4e1cbc1ed6f1b06ded02))


- Change changelog workflow to use PR with auto-merge by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0242063](https://github.com/JoShMiQueL/sig-maker/commit/024206355fb54ccdcc83602d79c4e42ac2e0dd6e))


- Use GH_PAT secret for changelog PR creation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0ab8b7c](https://github.com/JoShMiQueL/sig-maker/commit/0ab8b7c613bb72ec44ee01769976418e1669c201))


- Use GH_PAT secret for changelog PR creation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([81addb5](https://github.com/JoShMiQueL/sig-maker/commit/81addb5f99a8d2a5e8d290d91d9f297f8c576c3d))


- use windows-latest instead of windows-2025-vs2026 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([084dd27](https://github.com/JoShMiQueL/sig-maker/commit/084dd2726325b2144fd205c8973e12e6fa9bec5f))


- add Rust toolchain cache and pin Rust version by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2829ca1](https://github.com/JoShMiQueL/sig-maker/commit/2829ca1813bb9108b8d18b2a04be6fa4ce2ad3e3))


- add Rust toolchain cache and pin Rust version by [@JoShMiQueL](https://github.com/JoShMiQueL) ([a214e6b](https://github.com/JoShMiQueL/sig-maker/commit/a214e6b2787fe0123118073446b619b5f0659afe))


- force Node.js 24 for GitHub Actions by [@JoShMiQueL](https://github.com/JoShMiQueL) ([79b60ed](https://github.com/JoShMiQueL/sig-maker/commit/79b60ede9160d2a6883dbbef239ce2c2e54b7dca))


- update actions to latest versions by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8c03fdb](https://github.com/JoShMiQueL/sig-maker/commit/8c03fdb7142b99874a6d20f00485efba50cb23ab))


- update release.yml for cargo-dist compatibility by [@JoShMiQueL](https://github.com/JoShMiQueL) ([7a204d8](https://github.com/JoShMiQueL/sig-maker/commit/7a204d8a0890ad6cf2418c5a5b4678bf5b9d0ee0))

### Documentation



- Document PR workflow and branch protection strategy by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6fc6014](https://github.com/JoShMiQueL/sig-maker/commit/6fc6014bb88ea62087b5982f964dc83b31d7fdb1))


- Add AGENTS.md with project context for AI agents by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ef6932f](https://github.com/JoShMiQueL/sig-maker/commit/ef6932fa7be2c5228513e8162cce741a397d3f7f))


- Add Pull Request template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([99b3b7a](https://github.com/JoShMiQueL/sig-maker/commit/99b3b7ab7a5a8f0f0a33182204fd232e92e450ad))


- Update TODO.md - mark Repository Quality as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ead45fd](https://github.com/JoShMiQueL/sig-maker/commit/ead45fd098a4efeec3294ec8b4fd41bcb863d1a8))


- Update README with badges, releases, and documentation links by [@JoShMiQueL](https://github.com/JoShMiQueL) ([56aad26](https://github.com/JoShMiQueL/sig-maker/commit/56aad260e341368dbf65b1dd02ff3fe9ba08198d))


- Update TODO.md - mark output to file as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([64a916f](https://github.com/JoShMiQueL/sig-maker/commit/64a916f2fd309234010df785ef2427a9dd7a8b3e))


- Update TODO.md - mark version and quiet as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([a69e9ed](https://github.com/JoShMiQueL/sig-maker/commit/a69e9edbb33278a6d0175e396ab3f24f3c0931db))


- Update TODO.md - mark pattern validation as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([9ff390a](https://github.com/JoShMiQueL/sig-maker/commit/9ff390ae970610240276cb638c9fcadf80ca1481))


- Update TODO.md - mark verbose and detailed stats as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f6e354d](https://github.com/JoShMiQueL/sig-maker/commit/f6e354dd0c671a4e2b03c87fefdc472fe8c23519))


- Update TODO.md - mark fuzzing as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f0b4e86](https://github.com/JoShMiQueL/sig-maker/commit/f0b4e86c054923bc4d74bd06ed3a1b8fbcb199fc))

### Features



- **cli:** Show usage and pause on double-click launch by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0845ab1](https://github.com/JoShMiQueL/sig-maker/commit/0845ab18d8cf7a9f13ea1dad3fea789549c6f6cb))


- **changelog:** Add New Contributors section by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5449986](https://github.com/JoShMiQueL/sig-maker/commit/5449986a52d7df3bae618265ec33d95895b161d8))


- **changelog:** Add Contributors section and Full Changelog link by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1497c29](https://github.com/JoShMiQueL/sig-maker/commit/1497c29f6327f21379ba0d940fe1c8dc95995f3c))


- **changelog:** List all PRs per contributor in Contributors section by [@JoShMiQueL](https://github.com/JoShMiQueL) ([319452c](https://github.com/JoShMiQueL/sig-maker/commit/319452cb73ec55998235e6d90d03e6289e8e9f39))


- **cli:** Add output to file option (-o, --output) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f7a6d87](https://github.com/JoShMiQueL/sig-maker/commit/f7a6d8783de07189c2e4bb4781c25bf5c6e7059a))


- **cli:** Add version flag and quiet mode by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f6f5d92](https://github.com/JoShMiQueL/sig-maker/commit/f6f5d92e5efd76c6f2370e6bdcb1aab0bfcd9c2e))


- **cli:** Add pattern validation (-c, --check) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2e41376](https://github.com/JoShMiQueL/sig-maker/commit/2e41376315969ac0aec4c9a35fe56ac723829741))


- **cli:** Add terminal colors support by [@JoShMiQueL](https://github.com/JoShMiQueL) ([514a52d](https://github.com/JoShMiQueL/sig-maker/commit/514a52d1721c4814ac66c6d7c59fa9a2262f808f))


- **stats:** Add entropy and compression ratio to verbose output by [@JoShMiQueL](https://github.com/JoShMiQueL) ([dc55d9b](https://github.com/JoShMiQueL/sig-maker/commit/dc55d9b7cfd3f89897731533ea00a1d52b358af5))


- **release:** Integrate changelog generation into release workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([dd1c6ad](https://github.com/JoShMiQueL/sig-maker/commit/dd1c6adf8ac065359a58d5a097a12499f6d318df))


- **changelog:** Add manual changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([b07e956](https://github.com/JoShMiQueL/sig-maker/commit/b07e95614c4ef3eab8bf720d1c8d6792fab51f1f))


- **changelog:** Add auto-merge to changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([b808873](https://github.com/JoShMiQueL/sig-maker/commit/b808873d1ee2c67defa3bc67ff37a5e73511adaf))


- **release:** Add automatic changelog generation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ffd36a5](https://github.com/JoShMiQueL/sig-maker/commit/ffd36a507e593804c97cf5b30949f7f47738bf11))


- **changelog:** Add auto-merge without CI wait by [@JoShMiQueL](https://github.com/JoShMiQueL) ([27a4c7d](https://github.com/JoShMiQueL/sig-maker/commit/27a4c7dfdee5c883153ad7405b94f30c55a38737))


- **changelog:** Create reusable action for changelog generation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5d18d56](https://github.com/JoShMiQueL/sig-maker/commit/5d18d565e6632e46ac9636c8801275b886e81dc5))


- **changelog:** Add admin token for bypassing branch protection by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8da8637](https://github.com/JoShMiQueL/sig-maker/commit/8da8637be4841cf65ff2d02b6c4c4b9d813c7808))


- **changelog:** Add [skip ci] to commit message by [@JoShMiQueL](https://github.com/JoShMiQueL) ([14ef148](https://github.com/JoShMiQueL/sig-maker/commit/14ef148479ea6555691a13a35ba94460418c7c18))

### Miscellaneous



- Add pre-commit git hooks (fmt, clippy, test) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([4e3e2c3](https://github.com/JoShMiQueL/sig-maker/commit/4e3e2c3597eba1167a3dff96b9fa62b7d2c679bb))


- Add build check and commit-msg hook for Conventional Commits by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5eabffb](https://github.com/JoShMiQueL/sig-maker/commit/5eabffbe7077ddf6499b209ba6f735e4728f9be9))


- **deps:** bump actions/stale from 9 to 10 by [@dependabot[bot]](https://github.com/dependabot[bot]) [[#1](https://github.com/JoShMiQueL/sig-maker/pull/1)]


- Remove automated changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([601aa56](https://github.com/JoShMiQueL/sig-maker/commit/601aa56ef084eed68a7aef36f7b42b7f9bf850cd))


- Remove automated changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([67ae097](https://github.com/JoShMiQueL/sig-maker/commit/67ae09726ec0d5fb419d96ed2517d37988ac997f))


- remove TODO.md by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d33902f](https://github.com/JoShMiQueL/sig-maker/commit/d33902fb45a12a998d7b0c9cba58bc1c583d328c))


- **release:** v0.1.1-beta by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0bb0151](https://github.com/JoShMiQueL/sig-maker/commit/0bb0151cefe80a34fb22cc63d45b7bbd2111a91c))

### Refactoring



- **changelog:** Create composite action to reuse code by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2f3fa32](https://github.com/JoShMiQueL/sig-maker/commit/2f3fa320c6f31da7cf05705b0d61b50e7929b0d2))


- **changelog:** Use secret directly in action by [@JoShMiQueL](https://github.com/JoShMiQueL) ([4d1b0c3](https://github.com/JoShMiQueL/sig-maker/commit/4d1b0c39bace4c7f6608958317763213f0d1def2))

### Testing



- Add integration tests for CLI features by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3f7514d](https://github.com/JoShMiQueL/sig-maker/commit/3f7514d7028b8cc89adc734f9ba83b882bba9acd))




### Contributors

- [@JoShMiQueL](https://github.com/JoShMiQueL)


**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/v0.1.0-beta...v0.1.1-beta
## [v0.1.0-beta](https://github.com/JoShMiQueL/sig-maker/releases/tag/v0.1.0-beta) - 2026-06-10

### CI/CD



- Update GitHub Actions for Node.js 24 and Windows 2025 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1ba2088](https://github.com/JoShMiQueL/sig-maker/commit/1ba2088a9b31ad246ebd5ccb9d05a59c356393fc))


- Remove Codecov coverage job by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f55d68f](https://github.com/JoShMiQueL/sig-maker/commit/f55d68f9b1e3ca67f1bc070429d2fed76d6ca6ba))


- Upgrade changelog-builder to v6 and gh-release to v3 (Node.js 24) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8f073ea](https://github.com/JoShMiQueL/sig-maker/commit/8f073ea2ff6c0c92def7140af2691fc33273d464))


- Migrate to cargo-dist for releases by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f7f11d1](https://github.com/JoShMiQueL/sig-maker/commit/f7f11d16d58213fe16f8386bd8e8b4c3debdf415))

### Fix



- Resolve clippy warnings and formatting issues by [@JoShMiQueL](https://github.com/JoShMiQueL) ([07f7d8f](https://github.com/JoShMiQueL/sig-maker/commit/07f7d8f7fbe4282e7909cfbab0558201e8828a9b))

### Miscellaneous



- Remove temporary commit message file by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1251f75](https://github.com/JoShMiQueL/sig-maker/commit/1251f75a70e9e704baf6313f248a6442baf0a996))


- Set version to 0.1.0-beta by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d1750c2](https://github.com/JoShMiQueL/sig-maker/commit/d1750c26bd73bb440c54c0c51072b64704cbbb20))




### New Contributors
- [@JoShMiQueL](https://github.com/JoShMiQueL) made their first contribution


### Contributors

- [@JoShMiQueL](https://github.com/JoShMiQueL)


