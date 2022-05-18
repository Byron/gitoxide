# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### New Features

 - <csr-id-35f146a8573dcc9a1de3230373c0cf0794c6b897/> Add `absolutize_components()`
   It helps to cleanup paths a little which comes in handy when dealing
   with `commondir` appended paths.

### Other

 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> :discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absoluteize' the path later (std::path::absolute()) is
   on the way for that.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 19 calendar days.
 - 20 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - `absolutize_*(dir)` is now `absolutize(dir, Option<cwd>)` ([`de87657`](https://github.com/Byron/gitoxide/commit/de87657194ad976cc73ebcc13c231537b35b4195))
    - More robust absolutize-paths implementation ([`4800ebe`](https://github.com/Byron/gitoxide/commit/4800ebec42f9bb6298cb5b2efdab71d6baf3b1ba))
    - Allow reading patterns from stdin ([`0c597fe`](https://github.com/Byron/gitoxide/commit/0c597fe78acdd5672b4535a7d82620c5f7f93649))
    - Add `absolutize_components()` ([`35f146a`](https://github.com/Byron/gitoxide/commit/35f146a8573dcc9a1de3230373c0cf0794c6b897))
    - :discover()` now returns the shortest path. ([`e4f4c4b`](https://github.com/Byron/gitoxide/commit/e4f4c4b2c75a63a40a174e3a006ea64ef8d78809))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
 * **Uncategorized**
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
</details>

## 0.1.0 (2022-04-28)

<csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/>

### Refactor (BREAKING)

 - <csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/> various name changes for more convenient API

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 1 calendar day.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - frame for `gix repo exclude query` ([`a331314`](https://github.com/Byron/gitoxide/commit/a331314758629a93ba036245a5dd03cf4109dc52))
    - refactor ([`21d4076`](https://github.com/Byron/gitoxide/commit/21d407638285b728d0c64fabf2abe0e1948e9bec))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - various name changes for more convenient API ([`5480159`](https://github.com/Byron/gitoxide/commit/54801592488416ef2bb0f34c5061b62189c35c5e))
    - Use bstr intead of [u8] ([`9380e99`](https://github.com/Byron/gitoxide/commit/9380e9990065897e318b040f49b3c9a6de8bebb1))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - Copy all existing functions from git-features::path to git-path:: ([`725e198`](https://github.com/Byron/gitoxide/commit/725e1985dc521d01ff9e1e89b6468ef62fc09656))
    - add empty git-path crate ([`8d13f81`](https://github.com/Byron/gitoxide/commit/8d13f81068b4663d322002a9617d39b307b63469))
 * **Uncategorized**
    - Release git-path v0.1.0 ([`ca019fc`](https://github.com/Byron/gitoxide/commit/ca019fca03c4ea0d70fabbf09808732925b58077))
</details>

## 0.0.0 (2022-03-31)

An empty crate without any content to reserve the name for the gitoxide project.

