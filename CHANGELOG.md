# Changelog

## [0.6.2](https://github.com/elct9620/cedar-policy-rb/compare/v0.6.1...v0.6.2) (2025-08-26)


### Bug Fixes

* improve Ruby version compatibility and cross-compilation ([e08a3a0](https://github.com/elct9620/cedar-policy-rb/commit/e08a3a0c2266a1feb74a83ee38705205653f3625))
* revert rb_sys to 0.9.116 and restore mingw support ([6646b08](https://github.com/elct9620/cedar-policy-rb/commit/6646b08aea8954b0071b29936d03b48854eb203a))
* select correct minimal ruby version for different platform ([9d9fc09](https://github.com/elct9620/cedar-policy-rb/commit/9d9fc09a4c8dec6269cd05b22774c5e03aeb2dce))


### Reverts

* restore rb_sys to 0.9.117 after investigation ([f1e77dd](https://github.com/elct9620/cedar-policy-rb/commit/f1e77ddca61b7c48575c6548620a76cc5954524e))

## [0.6.1](https://github.com/elct9620/cedar-policy-rb/compare/v0.6.0...v0.6.1) (2025-08-26)


### Miscellaneous Chores

* release-as 0.6.1 ([882b4d9](https://github.com/elct9620/cedar-policy-rb/commit/882b4d9566a09f733195fc8216032a2fd0f3c7b9))

## [0.6.0](https://github.com/elct9620/cedar-policy-rb/compare/v0.5.3...v0.6.0) (2025-06-12)


### ⚠ BREAKING CHANGES

* depends on behavior in RubyGems >= 3.3.22

### Features

* adds support for musl libc ([#43](https://github.com/elct9620/cedar-policy-rb/issues/43)) ([563f4f4](https://github.com/elct9620/cedar-policy-rb/commit/563f4f45aeb9f97cccc8aad384702a7ac5dcbc7a))


### Miscellaneous Chores

* release 0.6.0 ([1e18e75](https://github.com/elct9620/cedar-policy-rb/commit/1e18e75574ef30d00cf21de212b88360d718e5b4))

## [0.5.3](https://github.com/elct9620/cedar-policy-rb/compare/v0.5.2...v0.5.3) (2025-06-10)


### Bug Fixes

* pass ruby versions to rb-sys-dock using --ruby-versions flag ([b94f1b7](https://github.com/elct9620/cedar-policy-rb/commit/b94f1b7e9c5d52c4e514c5f8553ccf7a71b25c53))

## [0.5.2](https://github.com/elct9620/cedar-policy-rb/compare/v0.5.1...v0.5.2) (2025-01-22)


### Bug Fixes

* rb-sys version too old cause build failed ([6b58381](https://github.com/elct9620/cedar-policy-rb/commit/6b583811e9d7e8eef3fe1843e7a04f8b9fc6f975))

## [0.5.1](https://github.com/elct9620/cedar-policy-rb/compare/v0.5.0...v0.5.1) (2025-01-22)


### Bug Fixes

* the Gemfile.lock not updated ([8853e74](https://github.com/elct9620/cedar-policy-rb/commit/8853e747188e83aa4ac1bfa7b4d1c361932cdb76))

## [0.5.0](https://github.com/elct9620/cedar-policy-rb/compare/v0.4.0...v0.5.0) (2025-01-22)


### Features

* **deps:** bump rust cedar_policy to v4.3.0 ([9aea4f1](https://github.com/elct9620/cedar-policy-rb/commit/9aea4f130867243b4d61be41a2f8abfd3d021df2))
* **deps:** update cedar-policy to 4.2.0 ([a7218d7](https://github.com/elct9620/cedar-policy-rb/commit/a7218d7c3af0384cfcdae735f07b06c624ea8002))

## [0.4.0](https://github.com/elct9620/cedar-policy-rb/compare/cedar_policy-v0.3.0...cedar_policy/v0.4.0) (2024-10-06)


### Features

* **deps:** upgrade cedar-policy to v4.1.0 ([61d6fa1](https://github.com/elct9620/cedar-policy-rb/commit/61d6fa1a59ab2edd71972410c1d9d697fde60776))


### Bug Fixes

* commitizen hook file bundled into gem ([b0c9c77](https://github.com/elct9620/cedar-policy-rb/commit/b0c9c77459ec614bfd4698d804969adb9b4bccc1))
