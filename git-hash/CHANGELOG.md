# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - greatly reduce changelog size now that the traversal fix is applied (3924c03)
    - rename `oid::short_hex()` to `oid::to_hex()` (deb99b3)
    - Fixup remaining changelogs… (0ac488a)
    - Generate changelogs with details (fd0f3bd)
    - oid::short_hex(len) for truncated hex representations (d234b97)
</details>

## v0.6.0 (2021-09-07)

### Breaking

- `ObjectId::empty_tree()` now has a parameter: `Kind`
- `ObjectId::null_sha(…)` -> `ObjectId::null(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-hash v0.6.0 (6efd90d)
    - [repository #190] obtain the kind fo hash used in a repo (a985491)
</details>

## v0.5.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.5.1 (d826370)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 74 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (ae02dab)
    - thanks clippy (e1964e4)
    - [ref] flexible and simple support for different hash lengths (9c2edd5)
    - Revert "[ref] parameterize all uses of hash length…" (21f187e)
    - [ref] parameterize all uses of hash length… (5c7285e)
    - [ref] handle create-or-append when writing valid reflog files… (9175085)
    - [ref] another deletion test succeeds (6037900)
    - thanks clippy (6200ed9)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-repository] towards git-repository as one stop shop (aea6cc5)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.3.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 16 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (e9665c7)
    - [traversal] trying to get things done with gitoxide shows some teeth… (3fee661)
    - Nicer debug printing for oids, too (b4f94f8)
    - a new failing test (86b6c24)
    - fix git-hash docs (327a107)
</details>

## v0.2.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - Revert "Add additional variant for Sha256 in ObjectId" (bb24dc4)
    - Add additional variant for Sha256 in ObjectId (3dd7c43)
    - Make ObjectId into an enum to soon hold more bytes (and type) (4bf0c1a)
    - Impl == and != for common combinations of ObjectId/oid (2455178)
    - Remove now unused gith-hash::borrowed::Id (59ab1bd)
    - More general to-hex for ObjectId (e2be868)
    - Fix incorrectly implemented display for `oid` (c4186b0)
    - git-commitgraph uses `oid` now (0b72966)
    - Notes about future proofing `oid` type… (658c896)
    - Use new `oid` where possible in git-odb (68a709e)
    - oid with even more conversions and better hex-display (eecd664)
    - refactor; better errors for invalid hash sizes (be84b36)
    - Add quality-of-life parse() support for hex input (6f97063)
    - Make ObjectId/oid happen! (ca78d15)
    - A seemingly complete implementation of a referenced borrowed Id (b3fc365)
    - Fix doc string naming (59c3d45)
    - Move git-hash::owned::Id into git-hash::Id (fdbe704)
    - Make git-hash Error usage explicit (it's for decoding only) (4805cfc)
    - Rename `git_hash::*::Digest` to `Id` (188d90a)
 * **Uncategorized**
    - (cargo-release) version 0.2.0 (4ec09f4)
</details>

## v0.1.2 (2021-01-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.2 (d1b4436)
    - update tasks and dependencies (96938be)
    - Add missing '.' at end of doc comments (7136854)
</details>

## v0.1.1 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 (4224c5b)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
</details>

## v0.1.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - first incarnation of git-hash to separate concerns and resolve cycle (9803041)
</details>

