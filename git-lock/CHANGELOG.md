# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 33 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - Sort all commits by time, descending… (7c37a3d)
    - greatly reduce changelog size now that the traversal fix is applied (3924c03)
    - Fixup remaining changelogs… (0ac488a)
 * **Uncategorized**
    - Merge pull request #172 from mellowagain/main (61aebbf)
</details>

## v1.0.0 (2021-08-25)

- initial release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-lock v1.0.0 (f38f72c)
    - Release git-tempfile v1.0.0 (1238535)
    - [stability #171] prepare git-lock and git-tempfile release (3a1cf4d)
    - [stability #171] Prime git-tempfile and git-lock for release (01278fe)
</details>

## v0.3.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-lock v0.3.2 (a5ea2e7)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.3.1 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.1 (168f5a0)
    - [lock #154] add io impls for `File` (be62a8b)
</details>

## v0.3.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (263088b)
    - (cargo-release) version 0.6.0 (d58f37e)
</details>

## v0.2.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 40 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (20d8e27)
    - (cargo-release) version 0.5.0 (0e11e98)
    - Bump fastrand from 1.4.1 to 1.5.0 (b138b43)
    - [ref] fix docs (536555d)
    - [ref] fix build (b4dcdfc)
    - [lock] support recoverable commits (b2217e7)
    - [lock] refactor (48861b2)
    - [lock] FAIL: trying to make peristence recoverable… (1fcdd1e)
    - [ref] try fix windows, once again (95e74dd)
    - [lock] access to the locked resource path (797bafa)
    - [lock] allow accessing the lock file path more easily (b808b00)
    - [lock] Fix handling of .lock extension on files without extension (64ac60d)
    - [lock] close file lock and commit markers (f700821)
    - [lock] Marker commit with runtime check for protection (b747814)
</details>

## v0.1.0 (2021-06-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.0 (60d48b0)
    - (cargo-release) version 0.4.0 (4512798)
    - [lock] capture amount of attempts taken when obtaining a lock (7fafa3e)
    - [lock] validate error message when waiting for some tim (34d3c5a)
    - [lock] the first test for lock failure (immediate mode) (2d67a0e)
    - [lock] add [must_use = "reason"] attribute where it matters (813c46b)
    - thanks clippy (29782e8)
    - [lock] lock acquire with backoff, but without test for now (bb2ba81)
    - [lock] prevent flakyness due to rounding or something (6f8fbcc)
    - [lock] refactor (ddc2170)
    - [lock] remaining test for everything proper exponential backoff needs (368d994)
    - [lock] support for randomization (220eb99)
    - [lock] better overshoot test for exponential backoff (62c17d8)
    - [lock] a sketch of exponential backoff, without rnadomization (55670b4)
    - [lock] refactor, remaining docs (956e69f)
    - [lock] tests green (3706b26)
    - [lock] creation of lockfiles, with immediate failure mode (fda7da8)
    - [lock] first tests and a lot of refactoring (3c34194)
    - [lock] even more sketched out API (0dc88c9)
    - [lock] refactor; Marker is definitely not necessary… (6af84c9)
    - [lock] test what happens if multiple tempfiles are created (17942c7)
    - [lock] sketch API (f0e1427)
    - (cargo-release) version 0.3.0 (92f3a83)
    - (cargo-release) version 0.2.0 (7c2eb36)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.0.0 (2021-06-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [lock] frame for git-lock crate (e6bc87d)
</details>

