# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Breaking

- Change return value of `prelude::RepositoryAccessExt::committer()` from `git_actor::Signature` to `Result<git_actor::Signature, easy::borrow:repo::Error>`
- Change return value of `prelude::ReferenceAccessExt` from `Result<Vec<RefEdit>>, _>` to `Result<easy::Reference, _>`.
- Rename `State` structs that serve as platform for iterators or other dependent types into `Platform`. These are usually intermediate objects only.
- Rename `easy::Reference::log()` into `easy::Reference::logs()`

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 204 commits contributed to the release over the course of 11 calendar days.
 - 49 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

## v0.9.1 (2021-09-10)

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 44 commits contributed to the release.
 - 5 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

## v0.9.0 (2021-09-08)

- rename `prelude::ConfigAccessExt` to `prelude::RepositoryAccessExt`
- `prelude::ObjectAccessExt::commit()` signature change
- cargo feature changed in incompatible ways. `network` was replaced by more finegrained options for _blocking_ and _async_ networking, as well as optional http transport
- 
### New

- `init()`
- `init_bare()`
- `Repository::init(Kind)`
- `open()`
- `Repository::open()`
- `easy::Head`
- `easy::ext::ReferenceAccessExt::head()`
- `ext::ReferenceExt` trait

### Breaking
- **renames / moves / Signature Changes**
    - `path::Path` to `Path`
    - `init::repository(dir)` -> `path::create::into(dir, **Kind**)`

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 8 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.8.2 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 50 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.7.2 (2021-08-17)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 20 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.7.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 19 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.7.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 40 commits contributed to the release over the course of 63 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.6.0 (2021-05-28)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 32 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.5.0 (2021-04-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 47 commits contributed to the release over the course of 196 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.4.0 (2020-09-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 99 commits contributed to the release over the course of 28 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 6 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 6 commits contributed to the release over the course of 17 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies
### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>
 - 292 commits contributed to the release over the course of 10 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

