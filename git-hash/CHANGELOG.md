# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - Generate changelogs with details (fd0f3bd)
    - oid::short_hex(len) for truncated hex representations (d234b97)
</details>

## v0.6.0 (2021-09-07)

### Breaking

- `ObjectId::empty_tree()` now has a parameter: `Kind`
- `ObjectId::null_sha(…)` -> `ObjectId::null(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 76 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.6.0 (06b4927)
    - Merge branch 'repository-integration' (49f5453)
    - [features #189] simple UTC-offset support for git-features (b58134b)
    - [odb #190] Read all eligble packed refs, no "pack-" prefix needed (ab250f7)
    - [features #???] WIP local time (1388ebf)
    - [repository #190] test for oid.ancestors().all() (fdc3678)
    - [#189] remove special handling of time from deny.toml (72050ef)
    - [repository #190] fix build, lets just make traversal available by default (6da3599)
    - [#189] Upgrade to prodash 16… (8e98418)
    - Bump git-pack v0.10.0 (e5e3c80)
    - [repository #185] rustfmt (dfbb015)
    - [repository #190] access to repository directories (f4d1ec4)
    - [config #185] refactor (509c938)
    - [repository #190] first shot at ancestor iteration… (85f1a48)
    - [config #185] Count lines correctly on windows… (57203ce)
    - [repository #190] refactor (e7188e0)
    - [config #185] add test for handling windows formatted files… (2a2a89f)
    - [ref #190] refactor (010be48)
    - [repository #185] remove quick-error infavor of thiserror (212c44c)
    - [ref #190] fix tests (e426e15)
    - [repository #185] on the way to removing quick-error (6ecd431)
    - [repository #190] fix tests; needs inbound transaction handling… (e5a5c09)
    - [config #185] flyby refactor (9b9ffa3)
    - [ref #190] don't provide namespace support for loose and packed refs… (c663da1)
    - [repository #185] support for initializing bare repositories (9e8a39e)
    - [ref #190] find() with namespace support (1240c21)
    - [repository #185] use git-config to handle bare repos more properly (8a5aac5)
    - [ref #190] prepare test for namespaced find(…) (5fcd0e4)
    - [repository #185] sketch of how to open a repository… (48207b5)
    - [repository #190] leverage git-ref namespace support (1aa9c11)
    - [repository #185] refactor (63089ff)
    - [ref #190] iteration with namespace support (d5987d4)
    - [repository #185] refactor (7604935)
    - [ref #190] refactor (3c7968c)
    - [repository #185] refactor repository initialization… (5ff7eaa)
    - [#190] disable caching to see if this fixes windows (0660a6f)
    - Notes about 'git-notes' and 'git-trailers' (459dd37)
    - [repository #190] prepare for namespacing support on file store level (d2d1db6)
    - Release gitoxide-core v0.10.3 (e132680)
    - [repository #190] refactor (609c249)
    - Release git-protocol v0.10.2 (54a4400)
    - [ref #190] refactor (1ef6cb3)
    - Release git-transport v0.11.1 (0952976)
    - [repository #190] fix build (f5e118c)
    - Release git-config v0.1.5 (150ed76)
    - [repository #190] note a known limitation about finding references in namespaces… (d335731)
    - Release git-commitgraph v0.4.3 (7dfe16b)
    - [ref #190] more assetions to understand 'find(…)' for namespaced refs… (f58a0ff)
    - [various #184] configure docs.rs build features (cc50249)
    - [repository #190] transparent namespace support (d14f073)
    - Release git-repository v0.8.1 (b269a12)
    - [#190] run tests faster (at the cost of compile time) (a22c95b)
    - [repository #164] make EasyArcExclusive available (2fa3dcb)
    - [#190] faster builds with debug=false and dependency caching (0b0fea4)
    - Release cargo-smart-release v0.3.0 (82f0cec)
    - [ref #190] Make References sortable (16b2232)
    - Release git-repository v0.8.0 (15ae2b8)
    - [repository #190] turns out we need bstr with unicode support (3d8796e)
    - [repository #174] keep assets (e0fca77)
    - [repository #190] public bstr re-export (3b7ffde)
    - [repository #174] remove arc_lock code entirely (dcbe742)
    - [repository #190] cleanup usage of bstr… (e4411ff)
    - Release git-repository v0.8.0 (1c9c5f1)
    - [ref #190] more conversion trait impls (1795a33)
    - Release git-protocol v0.10.1 (cec8ee3)
    - [repository #190] prefixed reference iteration (a6e19c9)
    - [repository #174] conditionally compile future parking_lot version… (5375fc8)
    - [repository #190] implementation of reference iteration (all() for now)… (2c0939a)
    - [protocol #174] fix tests… (cdc16fc)
    - [repository #190] refactor (8c532a4)
    - [smart-release #174] add asciinema recording of failed release (6668527)
    - [repository #190] prepare reference iteration (427f146)
    - Release git-repository v0.8.0 (e191eab)
    - Bump git-hash v0.6.0 (6efd90d)
    - Release git-repository v0.8.0 (403ef0a)
    - [repository #190] obtain the kind fo hash used in a repo (a985491)
</details>

## v0.5.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

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
 - 0 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

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

## v0.3.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 16 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

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

 - 21 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

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
    - Use V1 resolver as publishes seem to fail with… (3f4be8e)
    - (cargo-release) version 0.2.0 (4ec09f4)
</details>

## v0.1.2 (2021-01-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

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
 - 0 unique issues were worked on

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
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - first incarnation of git-hash to separate concerns and resolve cycle (9803041)
</details>

