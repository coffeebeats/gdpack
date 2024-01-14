# Changelog

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
