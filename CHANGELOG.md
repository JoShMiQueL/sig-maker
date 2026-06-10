# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Bug Fixes

- *(changelog)* Use find to locate git-cliff binary by [@JoShMiQueL](https://github.com/JoShMiQueL) ([be3344a](https://github.com/JoShMiQueL/sig-maker/commit/be3344afc58272edae5359428dff22b221430037))

- *(changelog)* Remove [skip ci] from commit message by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0cd8d1d](https://github.com/JoShMiQueL/sig-maker/commit/0cd8d1d15995e87979100084e8f6416d49a5e7a3))

- *(ci)* Run CI on all pull requests by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1909350](https://github.com/JoShMiQueL/sig-maker/commit/1909350e15e717059c0dbb8f1938f2fcaaf8fdaf))

- *(ci)* Run CI on changelog branches by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3c723e9](https://github.com/JoShMiQueL/sig-maker/commit/3c723e9ecaf85ab48e09ab0db71155da8ab8a8f6))

- *(changelog)* Remove automatic trigger on push by [@JoShMiQueL](https://github.com/JoShMiQueL) ([cb34db7](https://github.com/JoShMiQueL/sig-maker/commit/cb34db79fcae6a045a67632277c3a7668daf08fb))

- *(changelog)* Create PR instead of direct push by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1933b1b](https://github.com/JoShMiQueL/sig-maker/commit/1933b1b7b1fdd2e5efd34b75fb050a2ae852ae43))

- *(changelog)* Use find to locate git-cliff binary by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1fe81c1](https://github.com/JoShMiQueL/sig-maker/commit/1fe81c16a8b94f13cbbf8849dc19dfdfdefec213))

- *(changelog)* Fix git-cliff installation in composite action by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d73632d](https://github.com/JoShMiQueL/sig-maker/commit/d73632d9a2a85a8a3df9a3e40e45d2daead330fc))

- *(release)* Only run changelog job on tag pushes by [@JoShMiQueL](https://github.com/JoShMiQueL) ([09682cf](https://github.com/JoShMiQueL/sig-maker/commit/09682cf19fa909dbccb7d98ff0e0c76592e68f9d))

- *(changelog)* Fix gh commands in workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d463db0](https://github.com/JoShMiQueL/sig-maker/commit/d463db02d0bf652116992bd1c97dcf4ae0a6a583))

- *(changelog)* Improve workflow to prevent PR pile-up by [@JoShMiQueL](https://github.com/JoShMiQueL) ([4293a8e](https://github.com/JoShMiQueL/sig-maker/commit/4293a8e2013a0ab6df8798d95f46967ead9ecdc0))

- *(changelog)* Add spacing between release sections by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1f54d25](https://github.com/JoShMiQueL/sig-maker/commit/1f54d25d3ac67db457d1371bcbcd454d42bfb115))

- *(changelog)* Filter bots from New Contributors section in template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f598cb8](https://github.com/JoShMiQueL/sig-maker/commit/f598cb8fd92125741266e1cfa671278d41ec57c9))

- *(changelog)* Filter out bot accounts from contributors by [@JoShMiQueL](https://github.com/JoShMiQueL) ([df74b97](https://github.com/JoShMiQueL/sig-maker/commit/df74b97379e9a43098cec0193fb64db0879e9e90))

- *(changelog)* Fix template errors (deprecated fields, missing release) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([758f338](https://github.com/JoShMiQueL/sig-maker/commit/758f338173433ce20afefdef45af5a800ae20af1))

- *(ci)* Move skip rules before group rules in cliff.toml by [@JoShMiQueL](https://github.com/JoShMiQueL) ([16fc195](https://github.com/JoShMiQueL/sig-maker/commit/16fc1953de99f18fc7200d1717f4e5b487273662))

- *(ci)* Remove paths-ignore from changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2f1b68d](https://github.com/JoShMiQueL/sig-maker/commit/2f1b68d94a1f14e30035ab569305f323a40241cb))

- *(ci)* Improve changelog generation with links and anti-loop by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0fbe551](https://github.com/JoShMiQueL/sig-maker/commit/0fbe551e9814049e7bb93bb533324340921512f1))


### CI/CD

- Use GH_PAT secret for changelog PR creation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([81addb5](https://github.com/JoShMiQueL/sig-maker/commit/81addb5f99a8d2a5e8d290d91d9f297f8c576c3d))

- Use GH_PAT secret for changelog PR creation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0ab8b7c](https://github.com/JoShMiQueL/sig-maker/commit/0ab8b7c613bb72ec44ee01769976418e1669c201))

- Change changelog workflow to use PR with auto-merge by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0242063](https://github.com/JoShMiQueL/sig-maker/commit/024206355fb54ccdcc83602d79c4e42ac2e0dd6e))

- Change changelog workflow to use PR with auto-merge by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f90336d](https://github.com/JoShMiQueL/sig-maker/commit/f90336d74639c75bc60e4e1cbc1ed6f1b06ded02))

- Add Dependabot and Stale bot, add PR template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([65c1952](https://github.com/JoShMiQueL/sig-maker/commit/65c1952269e7caf916d4f407eeda9f888c7881f0))

- Add git-cliff for automatic CHANGELOG.md generation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1e862e6](https://github.com/JoShMiQueL/sig-maker/commit/1e862e68a2869c128b5b26f5711cbdc1048d536d))


### Documentation

- Update TODO.md - mark fuzzing as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f0b4e86](https://github.com/JoShMiQueL/sig-maker/commit/f0b4e86c054923bc4d74bd06ed3a1b8fbcb199fc))

- Update TODO.md - mark verbose and detailed stats as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f6e354d](https://github.com/JoShMiQueL/sig-maker/commit/f6e354dd0c671a4e2b03c87fefdc472fe8c23519))

- Update TODO.md - mark pattern validation as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([9ff390a](https://github.com/JoShMiQueL/sig-maker/commit/9ff390ae970610240276cb638c9fcadf80ca1481))

- Update TODO.md - mark version and quiet as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([a69e9ed](https://github.com/JoShMiQueL/sig-maker/commit/a69e9edbb33278a6d0175e396ab3f24f3c0931db))

- Update TODO.md - mark output to file as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([64a916f](https://github.com/JoShMiQueL/sig-maker/commit/64a916f2fd309234010df785ef2427a9dd7a8b3e))

- Update README with badges, releases, and documentation links by [@JoShMiQueL](https://github.com/JoShMiQueL) ([56aad26](https://github.com/JoShMiQueL/sig-maker/commit/56aad260e341368dbf65b1dd02ff3fe9ba08198d))

- Update TODO.md - mark Repository Quality as complete by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ead45fd](https://github.com/JoShMiQueL/sig-maker/commit/ead45fd098a4efeec3294ec8b4fd41bcb863d1a8))

- Add Pull Request template by [@JoShMiQueL](https://github.com/JoShMiQueL) ([99b3b7a](https://github.com/JoShMiQueL/sig-maker/commit/99b3b7ab7a5a8f0f0a33182204fd232e92e450ad))

- Add AGENTS.md with project context for AI agents by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ef6932f](https://github.com/JoShMiQueL/sig-maker/commit/ef6932fa7be2c5228513e8162cce741a397d3f7f))

- Document PR workflow and branch protection strategy by [@JoShMiQueL](https://github.com/JoShMiQueL) ([6fc6014](https://github.com/JoShMiQueL/sig-maker/commit/6fc6014bb88ea62087b5982f964dc83b31d7fdb1))


### Features

- *(changelog)* Create reusable action for changelog generation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5d18d56](https://github.com/JoShMiQueL/sig-maker/commit/5d18d565e6632e46ac9636c8801275b886e81dc5))

- *(changelog)* Add auto-merge without CI wait by [@JoShMiQueL](https://github.com/JoShMiQueL) ([27a4c7d](https://github.com/JoShMiQueL/sig-maker/commit/27a4c7dfdee5c883153ad7405b94f30c55a38737))

- *(release)* Add automatic changelog generation by [@JoShMiQueL](https://github.com/JoShMiQueL) ([ffd36a5](https://github.com/JoShMiQueL/sig-maker/commit/ffd36a507e593804c97cf5b30949f7f47738bf11))

- *(changelog)* Add auto-merge to changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([b808873](https://github.com/JoShMiQueL/sig-maker/commit/b808873d1ee2c67defa3bc67ff37a5e73511adaf))

- *(changelog)* Add manual changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([b07e956](https://github.com/JoShMiQueL/sig-maker/commit/b07e95614c4ef3eab8bf720d1c8d6792fab51f1f))

- *(release)* Integrate changelog generation into release workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([dd1c6ad](https://github.com/JoShMiQueL/sig-maker/commit/dd1c6adf8ac065359a58d5a097a12499f6d318df))

- *(stats)* Add entropy and compression ratio to verbose output by [@JoShMiQueL](https://github.com/JoShMiQueL) ([dc55d9b](https://github.com/JoShMiQueL/sig-maker/commit/dc55d9b7cfd3f89897731533ea00a1d52b358af5))

- *(cli)* Add terminal colors support by [@JoShMiQueL](https://github.com/JoShMiQueL) ([514a52d](https://github.com/JoShMiQueL/sig-maker/commit/514a52d1721c4814ac66c6d7c59fa9a2262f808f))

- *(cli)* Add pattern validation (-c, --check) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2e41376](https://github.com/JoShMiQueL/sig-maker/commit/2e41376315969ac0aec4c9a35fe56ac723829741))

- *(cli)* Add version flag and quiet mode by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f6f5d92](https://github.com/JoShMiQueL/sig-maker/commit/f6f5d92e5efd76c6f2370e6bdcb1aab0bfcd9c2e))

- *(cli)* Add output to file option (-o, --output) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f7a6d87](https://github.com/JoShMiQueL/sig-maker/commit/f7a6d8783de07189c2e4bb4781c25bf5c6e7059a))

- *(changelog)* List all PRs per contributor in Contributors section by [@JoShMiQueL](https://github.com/JoShMiQueL) ([319452c](https://github.com/JoShMiQueL/sig-maker/commit/319452cb73ec55998235e6d90d03e6289e8e9f39))

- *(changelog)* Add Contributors section and Full Changelog link by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1497c29](https://github.com/JoShMiQueL/sig-maker/commit/1497c29f6327f21379ba0d940fe1c8dc95995f3c))

- *(changelog)* Add New Contributors section by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5449986](https://github.com/JoShMiQueL/sig-maker/commit/5449986a52d7df3bae618265ec33d95895b161d8))

- *(cli)* Show usage and pause on double-click launch by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0845ab1](https://github.com/JoShMiQueL/sig-maker/commit/0845ab18d8cf7a9f13ea1dad3fea789549c6f6cb))


### Miscellaneous

- Remove automated changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([67ae097](https://github.com/JoShMiQueL/sig-maker/commit/67ae09726ec0d5fb419d96ed2517d37988ac997f))

- Remove automated changelog workflow by [@JoShMiQueL](https://github.com/JoShMiQueL) ([601aa56](https://github.com/JoShMiQueL/sig-maker/commit/601aa56ef084eed68a7aef36f7b42b7f9bf850cd))

- *(deps)* Bump actions/stale from 9 to 10 (#1) by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#1](https://github.com/JoShMiQueL/sig-maker/pull/1) ([f425a24](https://github.com/JoShMiQueL/sig-maker/commit/f425a244a3a893e84506a5f2e9e2ab35ba1413fb))

- Add build check and commit-msg hook for Conventional Commits by [@JoShMiQueL](https://github.com/JoShMiQueL) ([5eabffb](https://github.com/JoShMiQueL/sig-maker/commit/5eabffbe7077ddf6499b209ba6f735e4728f9be9))

- Add pre-commit git hooks (fmt, clippy, test) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([4e3e2c3](https://github.com/JoShMiQueL/sig-maker/commit/4e3e2c3597eba1167a3dff96b9fa62b7d2c679bb))


### Refactor

- *(changelog)* Create composite action to reuse code by [@JoShMiQueL](https://github.com/JoShMiQueL) ([2f3fa32](https://github.com/JoShMiQueL/sig-maker/commit/2f3fa320c6f31da7cf05705b0d61b50e7929b0d2))


### Styling

- Add space between author link and commit hash in changelog by [@JoShMiQueL](https://github.com/JoShMiQueL) ([0e2fa51](https://github.com/JoShMiQueL/sig-maker/commit/0e2fa51dcac0683c19a0f0fe339f5c48767da475))


### Testing

- Add integration tests for CLI features by [@JoShMiQueL](https://github.com/JoShMiQueL) ([3f7514d](https://github.com/JoShMiQueL/sig-maker/commit/3f7514d7028b8cc89adc734f9ba83b882bba9acd))



### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL) — [`be3344a`](https://github.com/JoShMiQueL/sig-maker/commit/be3344afc58272edae5359428dff22b221430037), [`5d18d56`](https://github.com/JoShMiQueL/sig-maker/commit/5d18d565e6632e46ac9636c8801275b886e81dc5), [`27a4c7d`](https://github.com/JoShMiQueL/sig-maker/commit/27a4c7dfdee5c883153ad7405b94f30c55a38737), [`ffd36a5`](https://github.com/JoShMiQueL/sig-maker/commit/ffd36a507e593804c97cf5b30949f7f47738bf11), [`67ae097`](https://github.com/JoShMiQueL/sig-maker/commit/67ae09726ec0d5fb419d96ed2517d37988ac997f), [`0cd8d1d`](https://github.com/JoShMiQueL/sig-maker/commit/0cd8d1d15995e87979100084e8f6416d49a5e7a3), [`1909350`](https://github.com/JoShMiQueL/sig-maker/commit/1909350e15e717059c0dbb8f1938f2fcaaf8fdaf), [`3c723e9`](https://github.com/JoShMiQueL/sig-maker/commit/3c723e9ecaf85ab48e09ab0db71155da8ab8a8f6), [`cb34db7`](https://github.com/JoShMiQueL/sig-maker/commit/cb34db79fcae6a045a67632277c3a7668daf08fb), [`b808873`](https://github.com/JoShMiQueL/sig-maker/commit/b808873d1ee2c67defa3bc67ff37a5e73511adaf), [`1933b1b`](https://github.com/JoShMiQueL/sig-maker/commit/1933b1b7b1fdd2e5efd34b75fb050a2ae852ae43), [`1fe81c1`](https://github.com/JoShMiQueL/sig-maker/commit/1fe81c16a8b94f13cbbf8849dc19dfdfdefec213), [`d73632d`](https://github.com/JoShMiQueL/sig-maker/commit/d73632d9a2a85a8a3df9a3e40e45d2daead330fc), [`2f3fa32`](https://github.com/JoShMiQueL/sig-maker/commit/2f3fa320c6f31da7cf05705b0d61b50e7929b0d2), [`09682cf`](https://github.com/JoShMiQueL/sig-maker/commit/09682cf19fa909dbccb7d98ff0e0c76592e68f9d), [`b07e956`](https://github.com/JoShMiQueL/sig-maker/commit/b07e95614c4ef3eab8bf720d1c8d6792fab51f1f), [`dd1c6ad`](https://github.com/JoShMiQueL/sig-maker/commit/dd1c6adf8ac065359a58d5a097a12499f6d318df), [`601aa56`](https://github.com/JoShMiQueL/sig-maker/commit/601aa56ef084eed68a7aef36f7b42b7f9bf850cd), [`d463db0`](https://github.com/JoShMiQueL/sig-maker/commit/d463db02d0bf652116992bd1c97dcf4ae0a6a583), [`4293a8e`](https://github.com/JoShMiQueL/sig-maker/commit/4293a8e2013a0ab6df8798d95f46967ead9ecdc0), [`f0b4e86`](https://github.com/JoShMiQueL/sig-maker/commit/f0b4e86c054923bc4d74bd06ed3a1b8fbcb199fc), [`3f7514d`](https://github.com/JoShMiQueL/sig-maker/commit/3f7514d7028b8cc89adc734f9ba83b882bba9acd), [`f6e354d`](https://github.com/JoShMiQueL/sig-maker/commit/f6e354dd0c671a4e2b03c87fefdc472fe8c23519), [`dc55d9b`](https://github.com/JoShMiQueL/sig-maker/commit/dc55d9b7cfd3f89897731533ea00a1d52b358af5), [`514a52d`](https://github.com/JoShMiQueL/sig-maker/commit/514a52d1721c4814ac66c6d7c59fa9a2262f808f), [`9ff390a`](https://github.com/JoShMiQueL/sig-maker/commit/9ff390ae970610240276cb638c9fcadf80ca1481), [`2e41376`](https://github.com/JoShMiQueL/sig-maker/commit/2e41376315969ac0aec4c9a35fe56ac723829741), [`a69e9ed`](https://github.com/JoShMiQueL/sig-maker/commit/a69e9edbb33278a6d0175e396ab3f24f3c0931db), [`f6f5d92`](https://github.com/JoShMiQueL/sig-maker/commit/f6f5d92e5efd76c6f2370e6bdcb1aab0bfcd9c2e), [`64a916f`](https://github.com/JoShMiQueL/sig-maker/commit/64a916f2fd309234010df785ef2427a9dd7a8b3e), [`f7a6d87`](https://github.com/JoShMiQueL/sig-maker/commit/f7a6d8783de07189c2e4bb4781c25bf5c6e7059a), [`81addb5`](https://github.com/JoShMiQueL/sig-maker/commit/81addb5f99a8d2a5e8d290d91d9f297f8c576c3d), [`0ab8b7c`](https://github.com/JoShMiQueL/sig-maker/commit/0ab8b7c613bb72ec44ee01769976418e1669c201), [`0242063`](https://github.com/JoShMiQueL/sig-maker/commit/024206355fb54ccdcc83602d79c4e42ac2e0dd6e), [`f90336d`](https://github.com/JoShMiQueL/sig-maker/commit/f90336d74639c75bc60e4e1cbc1ed6f1b06ded02), [`56aad26`](https://github.com/JoShMiQueL/sig-maker/commit/56aad260e341368dbf65b1dd02ff3fe9ba08198d), [`ead45fd`](https://github.com/JoShMiQueL/sig-maker/commit/ead45fd098a4efeec3294ec8b4fd41bcb863d1a8), [`65c1952`](https://github.com/JoShMiQueL/sig-maker/commit/65c1952269e7caf916d4f407eeda9f888c7881f0), [`99b3b7a`](https://github.com/JoShMiQueL/sig-maker/commit/99b3b7ab7a5a8f0f0a33182204fd232e92e450ad), [`ef6932f`](https://github.com/JoShMiQueL/sig-maker/commit/ef6932fa7be2c5228513e8162cce741a397d3f7f), [`6fc6014`](https://github.com/JoShMiQueL/sig-maker/commit/6fc6014bb88ea62087b5982f964dc83b31d7fdb1), [`1f54d25`](https://github.com/JoShMiQueL/sig-maker/commit/1f54d25d3ac67db457d1371bcbcd454d42bfb115), [`f598cb8`](https://github.com/JoShMiQueL/sig-maker/commit/f598cb8fd92125741266e1cfa671278d41ec57c9), [`df74b97`](https://github.com/JoShMiQueL/sig-maker/commit/df74b97379e9a43098cec0193fb64db0879e9e90), [`758f338`](https://github.com/JoShMiQueL/sig-maker/commit/758f338173433ce20afefdef45af5a800ae20af1), [`319452c`](https://github.com/JoShMiQueL/sig-maker/commit/319452cb73ec55998235e6d90d03e6289e8e9f39), [`1497c29`](https://github.com/JoShMiQueL/sig-maker/commit/1497c29f6327f21379ba0d940fe1c8dc95995f3c), [`5449986`](https://github.com/JoShMiQueL/sig-maker/commit/5449986a52d7df3bae618265ec33d95895b161d8), [`0e2fa51`](https://github.com/JoShMiQueL/sig-maker/commit/0e2fa51dcac0683c19a0f0fe339f5c48767da475), [`16fc195`](https://github.com/JoShMiQueL/sig-maker/commit/16fc1953de99f18fc7200d1717f4e5b487273662), [`2f1b68d`](https://github.com/JoShMiQueL/sig-maker/commit/2f1b68d94a1f14e30035ab569305f323a40241cb), [`0fbe551`](https://github.com/JoShMiQueL/sig-maker/commit/0fbe551e9814049e7bb93bb533324340921512f1), [`5eabffb`](https://github.com/JoShMiQueL/sig-maker/commit/5eabffbe7077ddf6499b209ba6f735e4728f9be9), [`4e3e2c3`](https://github.com/JoShMiQueL/sig-maker/commit/4e3e2c3597eba1167a3dff96b9fa62b7d2c679bb), [`0845ab1`](https://github.com/JoShMiQueL/sig-maker/commit/0845ab18d8cf7a9f13ea1dad3fea789549c6f6cb), [`1e862e6`](https://github.com/JoShMiQueL/sig-maker/commit/1e862e68a2869c128b5b26f5711cbdc1048d536d)
* [@dependabot[bot]](https://github.com/dependabot[bot]) — [#1](https://github.com/JoShMiQueL/sig-maker/pull/1)



## [0.1.0-beta] - 2026-06-10

### CI/CD

- Migrate to cargo-dist for releases by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f7f11d1](https://github.com/JoShMiQueL/sig-maker/commit/f7f11d16d58213fe16f8386bd8e8b4c3debdf415))

- Upgrade changelog-builder to v6 and gh-release to v3 (Node.js 24) by [@JoShMiQueL](https://github.com/JoShMiQueL) ([8f073ea](https://github.com/JoShMiQueL/sig-maker/commit/8f073ea2ff6c0c92def7140af2691fc33273d464))

- Remove Codecov coverage job by [@JoShMiQueL](https://github.com/JoShMiQueL) ([f55d68f](https://github.com/JoShMiQueL/sig-maker/commit/f55d68f9b1e3ca67f1bc070429d2fed76d6ca6ba))

- Update GitHub Actions for Node.js 24 and Windows 2025 by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1ba2088](https://github.com/JoShMiQueL/sig-maker/commit/1ba2088a9b31ad246ebd5ccb9d05a59c356393fc))


### Fix

- Resolve clippy warnings and formatting issues by [@JoShMiQueL](https://github.com/JoShMiQueL) ([07f7d8f](https://github.com/JoShMiQueL/sig-maker/commit/07f7d8f7fbe4282e7909cfbab0558201e8828a9b))


### Miscellaneous

- Set version to 0.1.0-beta by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d1750c2](https://github.com/JoShMiQueL/sig-maker/commit/d1750c26bd73bb440c54c0c51072b64704cbbb20))

- Remove temporary commit message file by [@JoShMiQueL](https://github.com/JoShMiQueL) ([1251f75](https://github.com/JoShMiQueL/sig-maker/commit/1251f75a70e9e704baf6313f248a6442baf0a996))


### Styling

- Apply cargo fmt formatting by [@JoShMiQueL](https://github.com/JoShMiQueL) ([d0caf09](https://github.com/JoShMiQueL/sig-maker/commit/d0caf093176114c73bfee47606dcac6a97cb3298))



### Contributors

* [@JoShMiQueL](https://github.com/JoShMiQueL) — [`d1750c2`](https://github.com/JoShMiQueL/sig-maker/commit/d1750c26bd73bb440c54c0c51072b64704cbbb20), [`f7f11d1`](https://github.com/JoShMiQueL/sig-maker/commit/f7f11d16d58213fe16f8386bd8e8b4c3debdf415), [`8f073ea`](https://github.com/JoShMiQueL/sig-maker/commit/8f073ea2ff6c0c92def7140af2691fc33273d464), [`f55d68f`](https://github.com/JoShMiQueL/sig-maker/commit/f55d68f9b1e3ca67f1bc070429d2fed76d6ca6ba), [`1ba2088`](https://github.com/JoShMiQueL/sig-maker/commit/1ba2088a9b31ad246ebd5ccb9d05a59c356393fc), [`d0caf09`](https://github.com/JoShMiQueL/sig-maker/commit/d0caf093176114c73bfee47606dcac6a97cb3298), [`1251f75`](https://github.com/JoShMiQueL/sig-maker/commit/1251f75a70e9e704baf6313f248a6442baf0a996), [`07f7d8f`](https://github.com/JoShMiQueL/sig-maker/commit/07f7d8f7fbe4282e7909cfbab0558201e8828a9b)




