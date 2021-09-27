# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 9 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 144 commits contributed to the release over the course of 11 calendar days.
 - 39 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

## v0.7.3 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on causing
  runtime bugs. The latter were detected by tests and this release contains the fix to not rely on certain behaviour anymore.

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 16 commits contributed to the release.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

## v0.7.2 (2021-09-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 26 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

## v0.7.1 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.7.0 (2021-09-07)

### Breaking

* Replace `transaction::Create` with `transaction::PreviousValue` and remove `transaction::Create`
* Remove `file::Reference` in favor of `Reference`
* Move `file::log::Line` to `log::Line`
* `TargetRef::Symbolic(&BStr)` -> `TargetRef::Symbolic(FullNameRef)`
* replace `Transaction::namespacce()` with `file::Store::namespace`
 
### Commit Statistics

<csr-read-only-do-not-edit/>
 - 76 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.6.1

### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

## v0.5.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.5.3 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.5.2 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 16 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.5.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.4.1 (2020-12-19)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 46 commits contributed to the release over the course of 88 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.4.0 (2020-09-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 83 commits contributed to the release over the course of 29 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 3 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.1.0 (2020-07-15)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.5.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 406 commits contributed to the release over the course of 78 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.6.0 (2021-08-27)

### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`
### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 252 commits contributed to the release over the course of 8 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

