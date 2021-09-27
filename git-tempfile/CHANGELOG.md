# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## v1.0.2 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on in its tests.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release.
 - 3 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - loose reference iteration with non-dir prefixes… (293bfc0)
    - Add 'references().all().peeled().'… (6502412)
    - smart-release: filter refs correctly, but… (2b4a615)
    - smart-release: find tag references by name… (72e1752)
    - commit traversal along the first parent… (7bce49c)
    - git-ref(docs): improve changelog format (90e6128)
    - smart-release: sketch first step of info generation (ff894e5)
    - smart-release: changelog gets crates to work on (78d31d9)
 * **Uncategorized**
    - Release git-tempfile v1.0.2 (310ea73)
    - Update changelogs once more… (d57d279)
    - thanks clippy (68ea77d)
    - [repository] don't enforce feature flags that may fail on windows by default (afdec2e)
    - Dependency update (d2f23f8)
    - thanks clippy (7899ef1)
    - Update changelogs retro-actively… (78cfe0a)
    - Release gitoxide v0.8.4 (effb2a6)
    - Release gitoxide-core v0.10.5 (590e59b)
    - Release git-ref v0.7.2 (e940e9a)
    - Release git-protocol v0.10.4 (898ee08)
    - Release git-odb v0.21.3 (223f930)
</details>

## v1.0.1 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release over the course of 17 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - smart-release: handle unborn heads (0e02831)
    - smart-release: fmt (d66c5ae)
    - smart-release: refactor (d4ffb4f)
    - smart-release: refactor (9fc15f9)
    - smart-release: refactor (9e430df)
    - smart-release: initial test for changelog (a33dd5d)
    - smart-release: very basic support for changelog command… (1a683a9)
    - smart-release: add 'cargo changelog' sub-command binary (3677b78)
    - smart-release(test): add changelog to most tests (cdf4199)
 * **Uncategorized**
    - Release git-tempfile v1.0.1 (295eb37)
    - [smart-release] auto-detect changes in production crates as well (24bc1bd)
    - [smart-release #195] update test output to match CI… (f864386)
    - [smart-release #195] better error for untracked files. (f5266f9)
    - [#195] Provide nix-shell target for macos… (5e75e8c)
    - [tempfile #195] adapt to Rust 1.55 (d9e71ac)
    - [#195] Fix previously incorrect usage of io::Kind::Other… (4dae07d)
    - thanks clippy (4701296)
    - [smart-release #195] fix docs (8d7e132)
    - improved changelog… (8b82f7d)
    - [smart-release #195] assure dependent packages are not packages to be published (6792ebc)
    - Fix formatting of performance-tasks.md (917967e)
    - Merge branch 'Byron:main' into main (dc58eca)
    - Release git-actor v0.4.0 (16358c9)
    - Release git-testtools v0.5.0 (574ede9)
    - [actor #173] fix docs (2d7956a)
    - Release git-testtools v0.5.0 (86e0a92)
    - [actor #173] refactor (08a1849)
    - Upgrade to nom-7 (f0aa3e1)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - some helpful remarks; be more specific about fixing breakage (7783965)
    - [stability #171] Another question to ask before stabilizing a crate… (c2bc1a6)
    - Update COLLABORATING.md (e1a04cf)
    - Release git-lock v1.0.0 (f38f72c)
    - First draft of collaboration guide (ec3f0ec)
</details>

## v1.0.0 (2021-08-25)

- initial release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.0 (1238535)
    - Adjust contribution recommendation (3aae2e2)
    - [smart-release #171] it's about time we get some tests (48a489b)
    - [pack #170] there can only be one (dce4f97)
    - [stability #171] prepare git-lock and git-tempfile release (3a1cf4d)
    - [pack #170] clru allows for free lists, reducing allocation pressure... (4d820d2)
    - [stability #171] Prime git-tempfile and git-lock for release (01278fe)
</details>

## v0.6.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v0.6.1 (eda952f)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (d58f37e)
    - [utils #154] refactor: bool.then(||this) - neat (1dec1c4)
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 40 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (0e11e98)
    - [pack #153] implement io traits for tempfiles (59d03d6)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - [ref] basic lookup rule impl; needs more test cases (3226f77)
    - Remove unnecessary unsafe code (83e207a)
    - [ref] fix compile warning on windows (c328774)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8)
    - [ref] a test case specifically for lookup rules (ab3a34f)
    - Implement Parser::into_iter without extra allocation (aa79924)
    - dependency update (059fa33)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - [lock] support recoverable commits (b2217e7)
    - [ref] support for persistence with recovery (d8b2d66)
    - [ref] refactor (a261b82)
    - [ref] allow reflogs to be created in place of empty directory trees (80a6e0e)
    - [tempfile] a way to delete empty dirs recursively (6025aa0)
    - Bump libc from 0.2.97 to 0.2.98 (caf6495)
    - [tempfile] close a tempfile while keeping track of it (aa96ed1)
</details>

## v0.4.0 (2021-06-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (4512798)
    - [lock] add [must_use = "reason"] attribute where it matters (813c46b)
    - [lock] refactor, remaining docs (956e69f)
    - [lock] tests green (3706b26)
    - [lock] cleanup signal handling even more… (9fb13d2)
    - [lock] first tests and a lot of refactoring (3c34194)
    - [lock] refactor; Marker is definitely not necessary… (6af84c9)
    - [lock] test what happens if multiple tempfiles are created (17942c7)
    - [lock] sketch API (f0e1427)
</details>

## v0.3.0 (2021-06-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (92f3a83)
    - [tempfile] refactor (f3144a8)
    - [tempfile] remaining tests (450db66)
    - [tempfile] refactor (3bafa7b)
    - [tempfile] implement 'closed' version of tempfile (d4bb61d)
    - [tempfile] refactor (4788222)
    - [tempfile] fix docs (3cd6712)
    - [tempfile] sketch of a closed registration with different types (db9bb11)
    - [tempfile] refactor (8a0ce64)
    - [tempfile] typesafe diffentiation between writable tempfiles and closed ones (3b424b1)
    - [tempfile] refactor (913f301)
    - [tempfile] refactor (9384617)
    - [tempfile] implement 'map' on tempfile to realize that 'close()' can't be done… (f4a1d33)
</details>

## v0.2.0 (2021-06-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (7c2eb36)
    - [tempfile] improve docs (d311b08)
    - thanks clippy (a0f9803)
    - [tempfile] refactor (3a0f1ad)
    - [tempfile] refactor (9b8abd0)
    - [tempfile] cleanup control for named and unnamed tempfiles (0ef85a2)
    - [tempfile] all remaining remove_dir tests I can think of (3e45e5f)
    - [tempfile] first bunch of tests for error handling and basic function of rmdir (ba41a15)
    - [tempfile] quick impl of remove-dir iter without tests (bf48913)
    - [tempfile] refactor (9226dbe)
    - [tempfile] refactor (730b733)
    - [tempfile] refactor (1da35ce)
    - [tempfile] improve Retries documentation; retries docs for remove_dir (e665a5f)
    - [tempfile] sketch how tempfile cleanup should be configured… (71acede)
    - [tempfile] logic fixed, it's working (6ad4946)
    - [tempfile] better counting, but… (972113f)
    - [tempfile] better retry counts (c7a35ca)
    - [tempfile] refactor (273d722)
    - [tempfile] a better way to count retries… (e110b97)
    - [tempfile] trying to implement remove_dir really shows that… (1319b90)
    - [tempfile] docs for create_dir; frame for remove_dir; (aa6b6d1)
    - [tempfile] tests for automatic directory creation (3bd5cee)
    - [tempfile] refactor (d441312)
    - [tempfile] use create_dir::all based on configuration… (156c021)
    - [tempfile] remove todo (5a14ab6)
    - [tempfile] more information about error cases, too (7415141)
    - [tempfile] refactor (ae0c97a)
    - [tempfile] refactor (7c7658d)
    - [tempfile] test for racy directory creation… (c9073bf)
    - [tempfile] verify existing files are handled correctly (28fee55)
    - [tempfile] a test for directory creation limits (584b4b7)
    - [tempfile] limit retries for directory creation… (1536c7a)
    - [tempfile] refactor (fa7b8e9)
    - [tempfile] handle interrupts and assure there is an end to it (dc0afbd)
    - [tempfile] first recursive directory creation (b01faa9)
    - [tempfile] refactor (7b59392)
    - [tempfile] another test (9e4834d)
    - [tempfile] first sketch of iterator based create directory all… (314693c)
    - [lock] frame for git-lock crate (e6bc87d)
    - [tempfile] add journey test to validate operation on process level (2d1efd4)
    - [tempfile] more docs (d0c5e6b)
    - refactor (e0b7f69)
    - [tempfile] clean cargo manifest (d43f514)
    - [tempfile] fix windows for good (3192a47)
    - [tempfile] fix windows build (hopefully) (6c1df66)
    - [tempfile] refactor (4a45df0)
</details>

## v0.1.0 (2021-06-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [tempfile] prepare release (c0f7fde)
    - [tempfile] an example to show off signal handlers (f325e69)
    - [tempfile] remaining docs (d334dc0)
    - [tempfile] restore original signal handler behaviour. (9f91dd8)
    - [tempfile] process-id filter on deletion to support forks (611056f)
    - [tempfile] implement handler correctly, probably. (8cb0bbc)
    - [tempfile] remove tempfiles on shutdown, but… (954b760)
    - [tempfile] switch to dashmap as slab (6164719)
    - [tempfile] a more realistic slab test shows the index can get quite high. (915f14c)
    - [tempfile] first step towards clearing tempfiles… (b0e0cee)
    - [tempfile] precisely named tempfiles (edc74f0)
    - [tempfile] `take()` method (d377397)
    - [tempfile] basic operation of a tempfile (a692950)
    - [tempfile] show that slabs can store a lot actually (0cc5b33)
    - [tempfile] initial docs as there is a lot to consider (9dffc2b)
    - [tempfile] crate frame (1b04c39)
</details>

## v1.0.0 (2021-08-25)

