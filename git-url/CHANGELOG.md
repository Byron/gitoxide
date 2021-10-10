# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - Only write headlines that we can parse back… (d44369a)
    - Rebuild all changelogs to assure properly ordered headlines (4a9a05f)
    - Sort all commits by time, descending… (f536bad)
    - greatly reduce changelog size now that the traversal fix is applied (a0bc98c)
    - Fixup remaining changelogs… (2f75db2)
 * **Uncategorized**
    - make fmt, but now it picked up some parts that usually don't get altered… (01f7b72)
    - Update changelogs just for fun (21541b3)
</details>

## v0.3.3 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-url v0.3.3 (fdd5bdb)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.3.2 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 94 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.2 (03de99e)
    - (cargo-release) version 0.3.1 (4deef67)
    - Merge branch 'patch-2' (f01dc54)
    - Merge branch 'patch-1' (5edc076)
    - Fix compile warnings (42fd77b)
</details>

## v0.3.0 (2021-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 12 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (d5c6643)
    - thanks clippy (e13adb2)
    - [gitoxide-core] Use git-config for remote url parsing (c45feed)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.2.0 (2021-01-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (0c39373)
    - support for radicle urls (2c5b955)
</details>

## v0.1.1 (2020-12-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 93 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 (e94fefa)
    - finish git-url docs (4099508)
    - begin of documenting git-url crate (c891901)
    - remove dash in all repository links (98c1360)
    - Finish removal of rust 2018 idioms (0d1699e)
</details>

## v0.1.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 48 commits contributed to the release over the course of 28 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - refactor (e07fbd6)
    - [clone] encode message for git credentials helper (143549e)
    - [clone] make URL available in transport layer (6778447)
    - [clone] Finish round-trip testing (df617fd)
    - refactor (aea52fe)
    - [clone] first sketch of roundtripping URLs (23678f8)
    - [clone] first steps towards launching git-upload-pack while… (41f05f1)
    - [clone] Better error handling for generalized `connect(…)` (713808c)
    - [clone] expand-path should be server-side (8a38856)
    - thanks clippy (0506fd9)
    - [url] more specific 'missing user home' error (ec5721a)
    - refactor (e54681a)
    - [url] Actually the is_relative() case should never be triggered (ac89d38)
    - [url] try again, maybe this works on windows… (f14fdd1)
    - [url] Once more with feeling (2ea4a8c)
    - [url] all debug output there is… (3df5b41)
    - [url] yikes, more debugging for windows on CI (9a430e7)
    - [url] Another try to make this work on windows - tests probably (a51647f)
    - [url] See if this fixes the windows tests (534c6a6)
    - [url]  add standard conversions (27e3bdc)
    - refactor (73e2b1b)
    - [url] BString in public interface (745662d)
    - [url] Commit to 'bstr' (3d26ae1)
    - [url] remove feature toggle, 'home' dependency is small enough (a5a6f0f)
    - [url] Add user expansion support (behind feature toggle) (a684cfe)
    - [url] first stab at expanding paths with user names (37459dc)
    - thanks clippy (50acab7)
    - [url] Support for git and http urls, as well as user expansion parsing (5ef201d)
    - refactor (6ab7cc6)
    - [url] first stab at implementing username expansion reasonably (86d17a3)
    - [url] fix serde (569014d)
    - [url] Now with support for non-utf8 byte strings (81f01fd)
    - [url] more tests and additional limitations (3c2811f)
    - [url] handle trivial file protocol URLs better (18eb512)
    - [url] Disable URL parsing for things that look like paths (03b0de9)
    - [url] turns out that relative URLs and windows paths are killing it (0bee58e)
    - [url] Switch to 'url' crate, as correctness certainly is more important than compile times (da6ad48)
    - thanks clippy (a37c7a3)
    - [url] user and IPv4 parsing/simple validation (d1929ac)
    - [url] parse port number (bc8bd99)
    - try for leaner tests, but it does the opposite kind of :D (098f802)
    - refactor (4499a08)
    - refactor (42a1b51)
    - [url] the first green tests (a501bc1)
    - refactor (9c5fb91)
    - [url] infrastructure for nom errors, taken from git-object (0ae38ed)
    - [url] basic frame and first failing test (60aacf0)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.0.0 (2020-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add git-url crate (fd2e5ba)
</details>

