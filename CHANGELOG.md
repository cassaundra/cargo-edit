# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

<!-- next-header -->
## Unreleased - ReleaseDate

### Fixes

`upgrade`
- Hide "note" column when unused
- Summarize uninteresting rows by default

## 0.10.3 - 2022-07-27

### Fixes

`upgrade`
- Provide table view of upgrades, like `cargo outdated`, to raise visibility for why a change isn't made
- Fix where we didn't respect `--offline`
- Fix `--to-lockfile` to update non-registry version requirements
- Update lockfile for upgraded requirements
- Update `--help` to be consistent with `cargo add`

`rm`
- Update `--help` to be consistent with `cargo add`

## 0.10.2 - 2022-07-21

### Fixes

`upgrade`
- Only fail on bad lockfile if `--to-lockfile` is set

`rm`
- Don't duplicate error messages

## 0.10.1 - 2022-07-15

### Features

`upgrade`
- Note the `--pinned` flag when pinned dependencies are skipped

### Fixes

`add`
- Provide a failing command to tell people how to get it

## 0.10.0 - 2022-07-14

### Breaking changes

- Many programmatic APIs changed
- `cargo add` remove in favor of the version included with cargo 1.62.0
- `cargo upgrade` skips pinned dependencies by default, run with `--pinned` to do them all
- `cargo upgrade --skip-compatible` is now default, run with `--to-lockfile` to upgrade all
- `cargo upgrade` now accepts dependency keys rather than crate names
- `cargo upgrade` now preserves version req precision
- `cargo upgrade --allow-prerelease` was removed to match `cargo add`

### Fixes

All
- Align console messages
- Allow using `--manifest-path` with `--pkgid`
- Allow relative paths with `--manifest-path`

`upgrade`
- Positional arguments are now dependency keys, allowing forcing of renamed dependencies to upgrade
- Make compatible upgrades and precision preservation work together
- Cleaned up output
- Preserve user formatting of dependencies
- Don't confuse dependencies

### Features

`upgrade`
- Always preserve version req precision
- With `--verbose`, see why dependencies didn't upgrade
- Error if upgrades possible with `--locked`
- Allow multiple occurrences of `--pkgid`

`rm`
- Add `--target` flag
- Add `--dry-run` flag

## 0.9.1 - 2022-05-17

### Fixes

set-version
- Don't overwrite updated dependencies with stale data when modifying multiple packages

## 0.9.0 - 2022-03-28

In large part, this release is a test-bed for changes proposed as part of the
path to merging `cargo-add` into cargo.  See
[internals](https://internals.rust-lang.org/t/feedback-on-cargo-add-before-its-merged/16024)
for more background on the changes.

### Breaking Changes

- Many programmatic APIs changed
- Feature flag `vendored-libgit2` is activated by default

cargo-add
- Removed `--upgrade <policy>`
- Removed `--sort`
- Removed `--allow-prerelease`
- Removed `cargo add <git-url>`, requiring `cargo add --git <git-url>`
- Removed `--path <path>` in favor of `cargo add <path>`
- Removed `--vers <version-req>` in favor of `cargo add <name>@<version-req>`
- `--git` support is now feature gated as we work out how to expose it

### Features

cargo-add
- Lists available features
- Warn when adding non-existent features
- git `--tag` and `--rev` support
- `--default-features` flag for when updating an existing entry
- `--no-optional` flag for when updating an existing entry
- Allow `,` to separate `--features`
- Added `-F` short flag for `--features`
- `cargo add serde +derive` feature activation
- `--dry-run` support

### Fixes

General
- TOML 1.0 compliant parser
- Use stderr for user messages
- Improve detection for enabling colored output
- Handle empty cargo config `source` table

cargo-add
- Allow `--registry` with `name@version` and path dependencies
- Don't panic on `--target=` (ie empty target)
- Cleaned up "Adding" message
- Improve overwrite behavior (re-adding the same dependency)
- Allow using both `--manifest-path` and `--package`
- Remove invalid dependency activation
- When adding an existing dependency to another table, reuse the existing source information (e.g. version requirement)

cargo-rm
- Don't create empty feature tables
- Remove dep activation when no longer optional

cargo-upgrade
- Preserve version requirement precision (behind a feature flag)

cargo-set-version
- Allow `--metadata` to override version metadata
- Improve dependent detection

## 0.8.0 - 2021-09-22
#### Breaking Changes

Many programmatic APIs changed

cargo-add
- Dependency paths are now relative to current working directory, rather than affect crate root (#497)
- Sane defaults when adding a dependency from within the workspace (#504)

#### Features

- New `vendored-openssl` crate feature (#447)
- New `vendored-libgit2` crate feature (#488)
- Support for dotted keys in TOML (#491)

cargo-set-version
- New command to bump crate versions (#482)
- Automatically update all workspace dependents (#506)

cargo-upgrade
- Add `--exclude` (#446)

#### Fixes

- Fixed various bugs when interacting with the registry (e.g. #433, #484)
- Read config files with extensions as added with Rust 1.39 (#439)
- rustsec
  - Removed unmaintained `dirs` dependency (#440)
  - Remove dependency on old `hyper` v0.13 (#431)
- Respect `--quiet` when updating the index (#462)
- Lookup pkg id's relative to `--manifest-path` rather than current working directory (#505)

cargo-add
- Look up versions *after* updating the index (#483)
- Allow optional build dependencies (#494)
- Dependency paths are now relative to current working directory, rather than affect crate root (#497)
- Prevent `cargo add .` from working (#501)
- Sane defaults when adding a dependency from within the workspace (#504)

cargo-upgrade
- Update optional dependencies with `--to-lockfile` (#427)
- Actually report upgrade when `package` key is used (#409)

cargo-rm
- Remove references among features to crate being removed (#500)

## 0.7.0 - 2020-10-03

New features:
- Keep dependencies in sorted order if they were already sorted (#421 by @joshtriplett)

Fixes:
- Fix for cargo-nightly (#413 by @meltinglava)
- Normalise windows-style paths (#403 by @Michael-F-Bryan)
- Fix for non-lowercase crate names (#398)

## 0.6.0

New features:
* You can now specify a branch for git dependencies (#379 by @struktured)
* A long awaited feature to support `-p` flag in the workspace is finally there :tada: ` (#390 by @pwoolcoc)

Fixes:
* `--all` flag is now deprecated in favor of `--workspace` to match cargo (#392 by @pwoolcoc)

## 0.5.0

This is a minor release that updates the dependencies so that it's easier to use `cargo-edit` as a library.

Fixes:
- Adding a dependency that was renamed previously (#351 by @stiiifff)

Full changes: https://github.com/killercup/cargo-edit/compare/v0.4.2...v0.5.0

## 0.4.2

New features:
- Add a `--skip-compatible` flag to cargo upgrade (#360)

  This flag will make cargo upgrade ignore upgrades where the old
  version is semver compatible with the new one. This is useful in cases
  where you don't want to churn the `Cargo.toml` files in the whole project
  knowing that the lockfile is already forcing the versions to be up to date.

Other:
- Bunch of internal clean-ups

## 0.4.1

New features:
- new cool feature: try passing `--to-lockfile` to `cargo upgrade` (#337 by @tofay)
- alternative registries support (#336 by @tofay)
- `cargo add` now supports `--rename` flag (#345)

Bug fixes:
- `cargo upgrade` works correctly with renamed dependencies (#342 by @stiiifff)
- `cargo-{add, upgrade}` now works with ssh auth for git (#334)
- `cargo upgrade` does not downgrade prerelease dependencies (#348)

## 0.4.0

Major changes:
- `cargo add` and `cargo upgrade` now supports `--offline` mode 
and minimizes network queries (#317 by @DCjanus)
- `cargo add` now accepts `--sort` flag to sort dependencies (#322 by @thiagoarrais)

## 0.3.3

- Update dependencies to most recent versions

## 0.3.2

New features:
* add multiple local packages (#295)
* support for `--no-default-features` flag (#290)
* rm multiple crates (#289)

Bug fixes:
* strip semver metadata on versions (#304)

## 0.3.1

Update dependencies, which fixes issues with OpenSSL 1.1.1 (#245)

## 0.3.0

A lot has happened since the last stable release!

The most important feature sure is that we try to not mess up your `Cargo.toml` files anymore!
While we are not 100% there yet, `cargo add foo` should give you much nicer edits now.

Other cool features:

- Add proxy support via env variable (#179)
- Allow simultaneous specification of both version and path
  (thanks, @dherman!)
- Add specific error for a missing crate (thanks, @bjgill!)
- `cargo-upgrade` now supports `--precise`, `--dry-run`, and has nicer output
