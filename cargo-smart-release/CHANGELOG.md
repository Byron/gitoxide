# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Conservative pre-release version handling along with a flag to turn it off. See [this issue][194] for details.
- Rename `--allow-auto-publish-of-stable-crates` to `--no-auto-publish-of-stable-crates`, inverting its meaning.
- Add `--no-multi-crate-release` flag to return to previous default behaviour. All crate manifest changes are put into one commit.
- automatically bump pre-release transient dependents of published crates to prevent breakage down the road unless 
  `--no-isolate-dependencies-from-breaking-change` is set.

## v0.3.1 (2021-09-07) - internal improvements
### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 20 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 268 commits contributed to the release over the course of 18 calendar days.
 - 54 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

## v0.3.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 54 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.3.0 (2021-08-27)

- add `--skip-dependencies` flag
- add `--verbose` flag and be less verbose in dry-runs by default to provide only essential information
- improvements to notification clarity

### Breaking

- Use short flag for `--no-bump-on-demand` in `--bump-dependencies`

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 9 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 287 commits contributed to the release over the course of 11 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.2.4 (2021-08-15)

- Fix auto-push functionality

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.2.3 (2021-08-15)

- Less verbosity by default which is helpful on the first run to get an overview. Use `--verbose/-v` for all the details.
- Also push tags and HEAD by default, unless `--skip-push` is specified.

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.2.2 (2021-08-15)

- support for unsorted packed-refs files

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.2.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.2.0 (2021-08-13)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 8 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.1.0 (2021-08-13)

- initial release
### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 46 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

