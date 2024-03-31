# Changelog

## 0.2.4 (2024-03-31)

## What's Changed
* chore(deps): bump clap from 4.5.3 to 4.5.4 by @dependabot in https://github.com/coffeebeats/gdpack/pull/188
* chore(deps): bump reqwest from 0.12.1 to 0.12.2 by @dependabot in https://github.com/coffeebeats/gdpack/pull/189
* chore(deps): bump openssl-sys from 0.9.101 to 0.9.102 by @dependabot in https://github.com/coffeebeats/gdpack/pull/191
* chore(deps): bump dependabot/fetch-metadata from 1 to 2 by @dependabot in https://github.com/coffeebeats/gdpack/pull/190
* feat: pre-build `arm64` on `linux` binaries by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/193


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.2.3...v0.2.4

## 0.2.3 (2024-03-22)

## What's Changed
* chore(deps): bump clap from 4.5.2 to 4.5.3 by @dependabot in https://github.com/coffeebeats/gdpack/pull/176
* chore(deps): bump toml from 0.8.11 to 0.8.12 by @dependabot in https://github.com/coffeebeats/gdpack/pull/177
* chore(deps): bump git2 from 0.18.2 to 0.18.3 by @dependabot in https://github.com/coffeebeats/gdpack/pull/180
* chore(deps): bump reqwest from 0.11.26 to 0.11.27 by @dependabot in https://github.com/coffeebeats/gdpack/pull/181
* chore(deps): bump toml_edit from 0.22.8 to 0.22.9 by @dependabot in https://github.com/coffeebeats/gdpack/pull/182
* chore(deps): bump tj-actions/changed-files from 42 to 43 by @dependabot in https://github.com/coffeebeats/gdpack/pull/179
* chore(deps): bump reqwest from 0.11.27 to 0.12.0 by @dependabot in https://github.com/coffeebeats/gdpack/pull/183
* fix(ci): ensure last remote ref is always used when detecting changes by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/185
* fix(ci): fetch full history to enable correct change detection by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/186


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.2.2...v0.2.3

## 0.2.2 (2024-03-14)

## What's Changed
* chore(deps): bump toml from 0.8.10 to 0.8.11 by @dependabot in https://github.com/coffeebeats/gdpack/pull/162
* chore(deps): bump anyhow from 1.0.80 to 1.0.81 by @dependabot in https://github.com/coffeebeats/gdpack/pull/164
* chore(deps): bump reqwest from 0.11.25 to 0.11.26 by @dependabot in https://github.com/coffeebeats/gdpack/pull/165
* chore(deps): bump thiserror from 1.0.57 to 1.0.58 by @dependabot in https://github.com/coffeebeats/gdpack/pull/166
* fix: correctly set default version in `install.sh` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/167
* fix(ci): correctly update `PATH` in setup action by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/169
* fix(ci): correctly export environment variable in setup action by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/170
* fix(ci): correctly reference home directory in action; add missing `gdpack` command by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/171
* fix(ci): correctly use environment variable in cache path by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/173
* chore(deps): bump rust-ini from 0.20.0 to 0.21.0 by @dependabot in https://github.com/coffeebeats/gdpack/pull/172
* fix(ci): use correct path in cache key by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/174
* fix(ci): conditionally check for executable on path during setup by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/175


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.2.1...v0.2.2

## 0.2.1 (2024-03-10)

## What's Changed
* fix(ci): increase timeout for building releases to 15 minutes by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/160


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.2.0...v0.2.1

## 0.2.0 (2024-03-10)

## What's Changed
* chore(deps): bump clap from 4.5.1 to 4.5.2 by @dependabot in https://github.com/coffeebeats/gdpack/pull/148
* feat(deps): allow insecure paths for direct dependencies by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/149
* refactor(manifest): move script template config under `project.script_templates` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/150
* chore(deps): bump reqwest from 0.11.24 to 0.11.25 by @dependabot in https://github.com/coffeebeats/gdpack/pull/152
* feat!: add support for included and excluded addon files by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/153
* fix(install): improve handling of version mismatches by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/154
* fix(ci): cache the entire cargo registry by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/155
* fix(addon): correctly filter files during addon installation by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/157
* feat(cmd): handle relative paths to local dependencies by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/156
* fix(core): migrate to `globset` lib by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/158
* feat(manifest): add support for pre- and post-install hooks by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/159


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.1.4...v0.2.0

## 0.1.4 (2024-03-06)

## What's Changed
* chore(deps): bump toml_edit from 0.22.0 to 0.22.4 by @dependabot in https://github.com/coffeebeats/gdpack/pull/130
* chore(deps): bump git2 from 0.18.1 to 0.18.2 by @dependabot in https://github.com/coffeebeats/gdpack/pull/131
* chore(deps): bump clap from 4.4.18 to 4.5.0 by @dependabot in https://github.com/coffeebeats/gdpack/pull/132
* chore(deps): bump thiserror from 1.0.56 to 1.0.57 by @dependabot in https://github.com/coffeebeats/gdpack/pull/134
* chore(deps): bump toml_edit from 0.22.4 to 0.22.5 by @dependabot in https://github.com/coffeebeats/gdpack/pull/135
* chore(deps): bump toml_edit from 0.22.5 to 0.22.6 by @dependabot in https://github.com/coffeebeats/gdpack/pull/136
* chore(deps): bump clap from 4.5.0 to 4.5.1 by @dependabot in https://github.com/coffeebeats/gdpack/pull/137
* chore(deps): bump anyhow from 1.0.79 to 1.0.80 by @dependabot in https://github.com/coffeebeats/gdpack/pull/139
* chore(deps): bump semver from 1.0.21 to 1.0.22 by @dependabot in https://github.com/coffeebeats/gdpack/pull/140
* chore(deps): bump openssl-sys from 0.9.99 to 0.9.100 by @dependabot in https://github.com/coffeebeats/gdpack/pull/138
* chore(deps): bump serde from 1.0.196 to 1.0.197 by @dependabot in https://github.com/coffeebeats/gdpack/pull/141
* chore(deps): bump openssl-sys from 0.9.100 to 0.9.101 by @dependabot in https://github.com/coffeebeats/gdpack/pull/142
* chore(deps): bump tempfile from 3.10.0 to 3.10.1 by @dependabot in https://github.com/coffeebeats/gdpack/pull/143
* chore(deps): bump walkdir from 2.4.0 to 2.5.0 by @dependabot in https://github.com/coffeebeats/gdpack/pull/144
* feat(ci): create a GitHub action for installing `gdpack` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/145
* chore(deps): bump mio from 0.8.10 to 0.8.11 by @dependabot in https://github.com/coffeebeats/gdpack/pull/146
* feat(ci): add support for installing dependencies via the `gdpack` action by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/147


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.1.3...v0.1.4

## 0.1.3 (2024-02-06)

## What's Changed
* chore(deps): bump codecov/codecov-action from 3 to 4 by @dependabot in https://github.com/coffeebeats/gdpack/pull/125
* chore(deps): bump toml from 0.8.9 to 0.8.10 by @dependabot in https://github.com/coffeebeats/gdpack/pull/128
* chore(deps): bump tempfile from 3.9.0 to 3.10.0 by @dependabot in https://github.com/coffeebeats/gdpack/pull/127
* chore(deps): bump toml_edit from 0.21.1 to 0.22.0 by @dependabot in https://github.com/coffeebeats/gdpack/pull/129


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.1.2...v0.1.3

## 0.1.2 (2024-02-05)

## What's Changed
* chore(deps): bump reqwest from 0.11.23 to 0.11.24 by @dependabot in https://github.com/coffeebeats/gdpack/pull/120
* chore(deps): bump toml_edit from 0.21.0 to 0.21.1 by @dependabot in https://github.com/coffeebeats/gdpack/pull/122
* chore(deps): bump toml from 0.8.8 to 0.8.9 by @dependabot in https://github.com/coffeebeats/gdpack/pull/121
* feat: add support for `dev` addons overriding `prod` addons within a target by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/123


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.1.1...v0.1.2

## 0.1.1 (2024-01-27)

## What's Changed
* chore(deps): bump openssl-sys from 0.9.98 to 0.9.99 by @dependabot in https://github.com/coffeebeats/gdpack/pull/116
* chore(deps): bump tj-actions/changed-files from 41 to 42 by @dependabot in https://github.com/coffeebeats/gdpack/pull/115
* feat(config): add support for declaring included and exported script templates by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/114
* chore(deps): bump serde from 1.0.195 to 1.0.196 by @dependabot in https://github.com/coffeebeats/gdpack/pull/118
* feat: implement script template installation by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/119


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.1.0...v0.1.1

## 0.1.0 (2024-01-22)

## What's Changed
* chore(ci): specify manifest path to cargo fetch and vendor commands by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/105
* fix(ci): update `crates.io` registry prior to running `cargo fetch` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/107
* fix(ci): eliminate redundant `cargo fetch` step by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/108
* fix(ci): remove `--frozen` limitation from `cargo vendor` step by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/109
* chore(deps): bump clap from 4.4.16 to 4.4.18 by @dependabot in https://github.com/coffeebeats/gdpack/pull/103
* chore(deps): bump typed-builder from 0.18.0 to 0.18.1 by @dependabot in https://github.com/coffeebeats/gdpack/pull/104
* chore(deps): bump h2 from 0.3.22 to 0.3.24 by @dependabot in https://github.com/coffeebeats/gdpack/pull/110
* feat(cmd/install): implement `install` command without transitive dependencies by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/111
* chore(core): refactor `addon` module into `core` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/112
* feat(core)!: create an `Install` implementation which supports transitive dependencies by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/113


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.17...v0.1.0

## 0.0.17 (2024-01-15)

## What's Changed
* fix(ci): ensure `bash` is used when building for Windows by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/100


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.16...v0.0.17

## 0.0.16 (2024-01-15)

## What's Changed
* fix(ci): correctly pass boolean type to build workflow by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/98


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.15...v0.0.16

## 0.0.15 (2024-01-15)

## What's Changed
* fix(ci): fix `openssl` cross-compilation issues by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/89
* refactor(ci): extract `gdpack` build steps into reusable workflow by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/91
* fix(ci): use correct build workflow name in release workflow by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/92
* feat(ci): allow calling the build workflow from the GitHub UI by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/93
* fix(ci): convert build workflow timeout type to expected type by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/94
* chore(ci): skip `cross` install if build artifact is cached by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/95
* fix(ci): set platform target in check workflow to utilize cached build artifacts by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/96
* chore(ci): increase build timeouts to 8 minutes during release by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/97


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.14...v0.0.15

## 0.0.14 (2024-01-14)

## What's Changed
* fix(ci): vendor `openssl` so that `cross` builds succeed by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/87


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.13...v0.0.14

## 0.0.13 (2024-01-14)

## What's Changed
* fix(ci): vendor `openssl` to support cross-compilation on CI hosts by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/83
* feat(ci/cargo-build): allow passing additional build arguments via `extra_args` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/85
* fix(cmd): improve command handling and when `gdpack` triggers a reinstallation of addons by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/86


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.12...v0.0.13

## 0.0.12 (2024-01-14)

## What's Changed
* chore(ci): auto merge patch-level dependabot dependency bumps by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/71
* chore(deps): bump semver from 1.0.20 to 1.0.21 by @dependabot in https://github.com/coffeebeats/gdpack/pull/67
* fix(ci): squash auto merges of Dependabot PRs by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/73
* chore(deps): bump anyhow from 1.0.77 to 1.0.79 by @dependabot in https://github.com/coffeebeats/gdpack/pull/68
* fix(ci): correctly invoke `gh pr merge` with squash option by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/74
* refactor(config): create a `config` module with largely refactored configuration implementations by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/75
* chore(deps): bump serde from 1.0.193 to 1.0.195 by @dependabot in https://github.com/coffeebeats/gdpack/pull/76
* chore(deps): bump clap from 4.4.12 to 4.4.13 by @dependabot in https://github.com/coffeebeats/gdpack/pull/70
* feat(ci): add support for code coverage via `cargo-llvm-cov` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/77
* chore(deps): bump clap from 4.4.13 to 4.4.14 by @dependabot in https://github.com/coffeebeats/gdpack/pull/78
* chore(deps): bump clap from 4.4.14 to 4.4.15 by @dependabot in https://github.com/coffeebeats/gdpack/pull/79
* chore(deps): bump clap from 4.4.15 to 4.4.16 by @dependabot in https://github.com/coffeebeats/gdpack/pull/80
* feat(addons): implement `Addon` installation from `Dependency` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/81
* fix(ci): use relative paths in code coverage report by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/82


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.11...v0.0.12

## 0.0.11 (2024-01-01)

## What's Changed
* chore: configure `mdlint` to allow non-sibling repeat headings by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/51
* feat(git): add support for downloading git dependencies by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/56
* fix(ci): ensure targets are built before testing by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/58
* chore(deps): bump anyhow from 1.0.76 to 1.0.77 by @dependabot in https://github.com/coffeebeats/gdpack/pull/55
* chore(deps): bump tj-actions/changed-files from 40 to 41 by @dependabot in https://github.com/coffeebeats/gdpack/pull/54
* fix(ci): use correct dependabot name by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/60
* chore(deps): bump clap from 4.4.11 to 4.4.12 by @dependabot in https://github.com/coffeebeats/gdpack/pull/59
* feat(ci): auto merge Dependabot PRs by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/61
* feat(dependency): add support for installing from a GitHub release by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/62
* chore(ci): update cache key to always specify platform by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/63
* fix(ci): ensure CI flow runs if actions or workflows change by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/64
* feat(ci): skip formatting, linting, and code coverage for non-source changes by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/65


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.10...v0.0.11

## 0.0.10 (2023-12-22)

## What's Changed
* feat(manifest): add support for target-specific dependencies by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/45
* chore(deps): bump actions/upload-artifact from 3 to 4 by @dependabot in https://github.com/coffeebeats/gdpack/pull/47
* chore(deps): bump actions/download-artifact from 3 to 4 by @dependabot in https://github.com/coffeebeats/gdpack/pull/48
* fix(ci): skip format job if triggered by dependabot by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/50
* chore(deps): bump anyhow from 1.0.75 to 1.0.76 by @dependabot in https://github.com/coffeebeats/gdpack/pull/49


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.9...v0.0.10

## 0.0.9 (2023-12-16)

## What's Changed
* fix(ci): correctly cache `$CARGO_HOME` directory by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/41
* feat(manifest): implement basic `manifest::Manifest` operations by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/44
* chore(deps): bump clap from 4.4.10 to 4.4.11 by @dependabot in https://github.com/coffeebeats/gdpack/pull/43

## New Contributors
* @dependabot made their first contribution in https://github.com/coffeebeats/gdpack/pull/43

**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.8...v0.0.9

## 0.0.8 (2023-12-04)

## What's Changed
* fix(ci): use correct working directory when publishing release artifacts by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/39


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.7...v0.0.8

## 0.0.7 (2023-12-04)

## What's Changed
* fix(ci): checkout repository during release asset publish job by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/37


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.6...v0.0.7

## 0.0.6 (2023-12-04)

## What's Changed
* fix(ci): update executable archive step on Windows by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/35


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.5...v0.0.6

## 0.0.5 (2023-12-04)

## What's Changed
* fix(ci): improve artifact archive steps in release workflow by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/33


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.4...v0.0.5

## 0.0.4 (2023-12-04)

## What's Changed
* fix(ci): correct syntax error when archiving gdpack.exe by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/31


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.3...v0.0.4

## 0.0.3 (2023-12-04)

## What's Changed
* feat(cmd/gdpack): define and document CLI sub-commands by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/11
* feat(pkg/manifest): add a draft implementation of a manifest file for recording dependencies by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/13
* feat(internal/git): add a `git` package with a `Clone` function for downloading git repositories by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/14
* feat: migrate project to Rust to take advantage of `toml_edit` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/16
* chore: update to latest release-please by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/19
* chore: specify `actions/upload-artifact` by major version only by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/20
* fix(ci): correctly specify `release-please` release type by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/21
* chore: refactor `release-please` config by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/22
* fix: add missing `release-please` manifest by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/23
* fix(ci): ensure `release-please` manifest isn't empty; add schema definition by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/24
* fix(ci): add missing version to `release-please` manifest by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/26
* fix(ci): remove schema definition from `release-please` manifest by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/27
* fix(ci): don't include component in tag by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/29
* fix: create zip portably; fix create release check by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/30


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.2...v0.0.3

## 0.0.2 (2023-11-10)

## What's Changed
* chore(ci): remove version pin from release action by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/4
* fix(docs,tools): correct Windows installation command; remove `tools.go` by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/9


**Full Changelog**: https://github.com/coffeebeats/gdpack/compare/v0.0.1...v0.0.2

## 0.0.1 (2023-11-08)

## What's Changed
* feat: scaffold the project by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/1
* fix(CI): move `dependabot.yml` and PR template to correct path by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/3

## New Contributors
* @coffeebeats made their first contribution in https://github.com/coffeebeats/gdpack/pull/1

**Full Changelog**: https://github.com/coffeebeats/gdpack/commits/v0.0.1
