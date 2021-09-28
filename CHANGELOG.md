# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 271 commits contributed to the release over the course of 18 calendar days.
 - 57 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 19 times to make code idiomatic. 

## v0.8.4 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 47 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.8.3 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 367 commits contributed to the release over the course of 20 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 80 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.8.1 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1470 commits contributed to the release over the course of 98 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 46 times to make code idiomatic. 

## v0.7.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 807 commits contributed to the release over the course of 143 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 30 times to make code idiomatic. 

## v0.6.0 (2020-12-16)

Maintenance release without any new features.

These are created to account for breaking changes within the dependency graph of
`gitoxide` crates. Due to some blunders in the past the version on crates.io
could not be installed anymore.
This was eventually fixed with new minor releases across the ecosystem.

Finally, yet another breaking change due to the introduction of the `git-hash`
crate to break a dependency cycle between `git-object` and `git-features` caused
yet another maintenance release.
## v0.5.0 (2020-12-15)

Maintenance release without any new features.
## v0.4.3 (2020-09-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 2 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

## v0.4.1 (2020-09-18)

* fix installation via `cargo install`

## v0.4.0 (2020-09-12)

* add `remote-ref-list` and `pack-receive` subcommands to **gixp**

### CLI Breaking

 * rename plumbing sub-command from `index-from-pack` to `pack-index-from-data`

## v0.3.0 (2020-08-12)

* add `pack-explode` and `pack-index-from-data` sub-commands
* massive speed improvements for `pack-verify`

Many small and possibly breaking changes are not mentioned here.
## v0.1.0 (2020-07-12)

* Initial release with `pack-verify`
