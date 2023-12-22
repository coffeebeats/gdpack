# Changelog

## 0.0.11 (2023-12-22)

## What's Changed
* chore: configure `mdlint` to allow non-sibling repeat headings by @coffeebeats in https://github.com/coffeebeats/gdpack/pull/51


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
