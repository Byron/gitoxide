# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 (2022-04-03)

### New Features (BREAKING)

 - <csr-id-a052d79674ccfe8693994150ccbe965792579491/> `ansi_c::unquote()` returns the amount of consumed bytes.
   That way it's possible to continue parsing past the quoted string.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - `ansi_c::unquote()` returns the amount of consumed bytes. ([`a052d79`](https://github.com/Byron/gitoxide/commit/a052d79674ccfe8693994150ccbe965792579491))
    - validate out-of-quote portions can be passed ([`22c776b`](https://github.com/Byron/gitoxide/commit/22c776badd1ea26a2b1ece84fd8c551784c72212))
 * **Uncategorized**
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
</details>

## 0.1.0 (2022-03-24)

Initial release with ansi_c unquoting capability.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - use git-quote crate in git-odb alternate parsing ([`8e49aa6`](https://github.com/Byron/gitoxide/commit/8e49aa6090c1c361e3ddd44798754c44c179ab49))
    - Add ansic::undo ([`1be8f14`](https://github.com/Byron/gitoxide/commit/1be8f14128b673ea3399bc04b0a6747de9d6d404))
    - add empty git-quote crate ([`0d1aaf0`](https://github.com/Byron/gitoxide/commit/0d1aaf00160f98e40fb92fd401c67f59da2475fc))
 * **Uncategorized**
    - Release git-quote v0.1.0 ([`a8f6c4d`](https://github.com/Byron/gitoxide/commit/a8f6c4d9e039be7fe82899ed281edb37e17e2a77))
</details>

