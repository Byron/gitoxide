# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - make fmt, but now it picked up some parts that usually don't get altered… (01f7b72)
</details>

## v0.16.4 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.16.4 (fd189c7)
    - Bump git-hash v0.6.0 (6efd90d)
    - [features #190] be more explicit about why sha1-asm is disabled (507d710)
    - [various #190] rename 'local-offset' to 'local-time-support' (3a7d379)
    - [actor #190] methods to get an actor signature at the current time (6d0bedd)
    - [features #189] simple UTC-offset support for git-features (b58134b)
    - [features #???] WIP local time (1388ebf)
    - [#189] Upgrade to prodash 16… (8e98418)
</details>

## v0.16.3 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [pack #67] Optimize caches based on cache debugging (1271c01)
    - [pack #67] Add cache debugging capabilities to git-features (8776c98)
    - thanks clippy (d689599)
    - [features] refactor (0958fc8)
    - [features] refactor (d4605cd)
</details>

## v0.16.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.16.2 (42861ca)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.16.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.1 (e10e55c)
    - (cargo-release) version 0.5.0 (ae02dab)
</details>

## v0.16.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 57 commits contributed to the release over the course of 78 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.0 (1231dbd)
    - upgrade prodash/crosstermion (f109409)
    - clippy on tests and thanks clippy (a77a71c)
    - [pack] fix build (98dd557)
    - [pack] all tests running for now, but… (aec8439)
    - refactor sha-1 specification to avoid duplication (e23d19c)
    - resolver = 2: works! (6dc8779)
    - try windows one more time: resolver = "2" (69d52b8)
    - Fix windows, leave todo, move on (2de9e78)
    - See if turning off "asm" support entirely fixes windows (b804ef2)
    - Try to fix build, again (c616627)
    - Don't use ASM on windows for Sha1 as it fails to build there. (ba1fb7a)
    - [features] enable ASM for everyone… (7a1128f)
    - [ref] reproducible loose ref iteration with built-in sorting (e138748)
    - [features] fix docs in the absence of sha1 related features (6ca02ac)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c1)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - [ref] first rough implementation of loose ref iteration (918af42)
    - refactor (2174513)
    - fix docs (e68d460)
    - Remove mentions of interrupt handling feature toggles (833ac04)
    - Fix everything up so that… (5930563)
    - A first attempt to make intrerupt tools work, but… (8fb8d37)
    - First step towards moving git-features::interrupt… (8a741d0)
    - fix build (ea2bfac)
    - refactor (7f9be36)
    - And one less usage of the global interrupt handler… (5da57a3)
    - Make most interrupts local to the method or function (4588993)
    - fix build (04d919f)
    - refactor (e0b7f69)
    - [features] sketch of iterator to auto-check for interruptions (61d3a15)
    - [tempfile] integrate with git-features to have a single top-level interrupt… (6e9400d)
    - [features] protect interrupt handler from multi-initialization (592404c)
    - [interrupt] remove any user mesasages as it can't be done in a handler. (8a10af7)
    - [tempfile] a first somewhat working version of signal-hooks for interrupt handling (07b3242)
    - Update to latest prodash to get rid of ctrlc (c070d6f)
    - refactor (2e86723)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - Bump crossbeam-utils from 0.8.4 to 0.8.5 (fce4d10)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-pack] fix docs (efd20d4)
    - [git-features] fix compilation (38c7961)
    - [git-pack] move hash-writer to git-features as it's quite general purpose (80e5640)
    - [git-features] Remove feature that would break licensing agreements (cd6ce67)
    - [git-features] fix typo (c6f342f)
    - [git-features] Finally zlib with feature toggles is working… (057016e)
    - [git-features] And now zlib finally works! (6d887d5)
    - [git-features] simplify even more (ca54d97)
    - [git-features] refactor to help understand a zlib-related logic bug (ae826e8)
    - [git-features] a first step towards supporting a pure rust zlib backend (040cab7)
    - [git-features] Add zlib module to allow changing implementation on the fly (4bdf783)
    - (cargo-release) version 0.15.0 (d69d9fb)
    - Put prodash behind a feature toggle, too (966058d)
    - Put 'walkdir' behind a feature flag/make it optional. (1a3cc5b)
    - Put 'sha1' behind a feature toggle (4f326bc)
    - Use crc32fast instead of `crc` (11955f9)
    - Put crc functionality behind a feature toggle (458fa6e)
</details>

## v0.14.0 (2021-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.14.0 (a760f8c)
    - upgrade to prodash 13/tui 0.15 (1c99f51)
</details>

## v0.13.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 12 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (e9665c7)
    - Allow calling 'finalize()' on the entries iterator (3c617bc)
    - git-odb without cargo warnings due to using the same test twice (8945f95)
    - Fix compile warning for git-features (d457faa)
    - fix doc links (870af2a)
    - run git-odb tests in parallel, too; improved threaded error handling (40802fd)
    - refactor (82c2f42)
    - refactor (7a6b514)
    - refactor (5ef1f22)
    - fix docs #(67) (01db10a)
    - refactor (3e908bd)
    - refactor (409d763)
    - refactor (896ab94)
    - Remove unused dependency (26beb2a)
    - Don't finish the computation on drop of SteppedReduce (6453633)
    - thanks clippy (c320761)
    - Remove unsafe interface for stepped computation #(67) (c856613)
    - A first working version of a static parallel iterator #(67) (d7d5c68)
    - A way iteration won't work with 'static #(67) (6fda1f2)
    - Sketch of machinery for producing pack entries #(67) (ac8e7fb)
    - Less restrictive requirements: Clone instead of Copy #(67) (410e7d6)
    - Improve Safety docs #(67) (15e4748)
    - A test to assure referenced input and references in 'consume' work #(67) (4526d82)
    - Make iterator creation unsafe and document why #(67) (593d5df)
    - First seemingly working version of an iterator which allows controlling threaded work #(67) (4a7ef7d)
    - Make the parallel SteppedReduce compile #(67) (017fdf4)
    - More docs to differentiate SteppedReduce from in_parallel() #(67) (153c083)
    - serial version of SteppedReduce seems to be working #(67) (779542e)
    - Only store thread state #(67) (0bf8a9b)
    - sketch instantiation of iterator adapter #(67) (a3083ad)
    - A reducer test in preparation for allow it to be used as iterator #(67) (1c2adf4)
    - (cargo-release) version 0.13.0 (ac2eddb)
    - Allow parallel reducers to produce something during 'feed()' #(67) (6c04fcd)
</details>

## v0.12.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 10 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - git-protocol uses `oid` type (3930a6f)
    - Make ObjectId/oid happen! (ca78d15)
    - Move git-hash::owned::Id into git-hash::Id (fdbe704)
    - Rename `git_hash::*::Digest` to `Id` (188d90a)
 * **Uncategorized**
    - (cargo-release) version 0.12.0 (3b71e7e)
    - (cargo-release) version 0.2.0 (4ec09f4)
    - refactor (dee8c66)
</details>

## v0.11.0 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.11.0 (1aa1f5e)
</details>

## v0.10.1 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 38 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.10.1 (0dcdfd7)
    - Remove usage of gitfeatures::fs in organize subcommand (b567d37)
    - Assure basic 'organize' operation is working as expected (deb6073)
    - A first stab at finding git repositories (e4dc964)
    - upgrade 'jwalk' (cba048f)
    - upgrade 'bytes' (3934392)
    - upgrade prodash and friends (50755bc)
    - Add missing '.' at end of doc comments (7136854)
</details>

## v0.10.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - use git-hash in git-features (5b307e0)
</details>

## v0.9.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 18 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.9.0 (a89fdb9)
    - (cargo-release) version 0.5.0 (fc7d600)
    - more docs for owned git-object (b79101d)
    - fix io::pipe tests (9604154)
    - uograde everything else (0cd79d0)
    - upgrade prodash and tui (b5eadca)
    - add remaining docs to git-features using the missing_docs directive (f8aafd6)
</details>

## v0.8.0 (2020-11-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (47c00c2)
    - finish git-features documentation (934a26c)
    - refactor (b3a8bb5)
    - refactor (f9e8d29)
    - docs for the git-features::pipe module (67a950a)
    - Document git-features::parallel (b899227)
    - dependency update (fb077f9)
    - finish git_features::interrupt docs (471a1bf)
    - dependency update (b3b4aba)
    - docs for git-features::hash (a3fdecc)
    - first sketch of filesystem docs for git-features (1a8141c)
</details>

## v0.7.0 (2020-11-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 63 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (7fa7bae)
    - specify the hash to create with 'hash::bytes_of_file' (c000294)
    - move 'git_odb::hash::bytes_of_file' into git_features::hash (c5f6b45)
    - remove dash in all repository links (98c1360)
    - Use parallel walkdir (via jwalk) when parallel feature is enabled (f444c85)
    - refactor (e4bcfe6)
</details>

## v0.6.0 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (9ef184e)
    - Switch to prodash 10 and safe a lot of trait bounds in the process (e2fb1d9)
</details>

## v0.5.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (82b7313)
    - [clone] This actually works: first MVP of retrieving packs via clone (c06d819)
    - [clone] test (and fix) for piped line reading (afe2996)
    - [clone] Send headers with BufReaders (6a95aaa)
    - [clone] pipe allows to send errors as well (69286ec)
    - [clone] BufRead for Reader… (bf1d40f)
    - [clone] a piped iterator (5148c85)
    - [clone] pipe probably shouldn't abort on empty writes (9cfa9b7)
    - thanks clippy (c4f570f)
    - [clone] more pipe tests (1652a74)
    - [clone] first working pipe implementation (490a9b9)
    - [clone] frame for implementing 'pipe' support (c555681)
    - Fix git-features hash tests (35e8809)
</details>

## v0.4.0 (2020-08-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 4 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump git-features to 0.4 to allow publishes after breaking changes (9d6b879)
    - [protocol] properly implement remote progress reporting (a81954a)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
    - add 'disable-interrupts' feature flag (ccd9c3e)
    - refactor (b4a6e16)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 16 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 (4351e28)
    - thanks clippy (6725104)
    - first step towards parallelizing file hashes and traversal! (9573836)
    - better usability for units (b226253)
    - update dependencie (ade06b4)
    - Make obvious that interrupt request was received (34b2373)
    - Remove once_cell dependency as it is really not required anymore (5ac9538)
    - make interrupt handler work reliably (e71da0f)
    - Conditionally use an eager iterator… (e9b5511)
    - refactor (d14f0f6)
    - Allow eager iterator to behave properly when used with index writing (66ebc5f)
    - first successful test of moving the streaming iterator into its own thread (c9fcb68)
    - now it's order preserving (4c8711e)
    - first sketch of order-destroying eager iterator (20fca45)
    - Print read throughput automatically (0a71b48)
    - Make sure interrupt logic works even without an interrupt handler… (66b1644)
    - Add percentage and throughput to tasks that matter (763d7ca)
    - Upgrade to latest iteration of prodash (3a4faec)
    - First part of migration to prodash 8.0, but… (6901a09)
    - thanks clippy (ed5882d)
    - Write about user interfaces and the use/non-use of async (91ba045)
    - interrupt support for pretty plumbing (bca7ce2)
    - support for interruptible operations (a025593)
    - refactor (413968d)
    - receive progress information when reading packs in bundle (759091d)
    - initial batch of progress usage for index creation… (b10e5c6)
    - first stab at streaming pack header encoding (3c6e78b)
    - We can now restore (possibly half-written) packs (b1daa46)
    - See how big a Sha1 hasher really is (26b271d)
    - First sketch of new verify expressed in terms of traversal (4cb570f)
</details>

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 10 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (0bb8314)
    - incorporate dynamic chunking into 'less-time' algorithm (295aa2f)
    - integrate new chunk size code into lookup code (a8422cf)
    - first round of number tuning done (a647b2d)
    - Somehow handle chunk size in absence of known chunk amount (acfccad)
    - Chunk computation seems alright, what about realistic values (973e6bb)
    - getting there… (a1b5d56)
    - first step towards computing better chunk sizes and thread limits (1cdde7d)
    - Add 'inc()' convenience methods to progress (2e46c9b)
    - (more) graceful shutdown of failing parallel tasks (163f50f)
    - Respect thread limit in 'in_parallel' (babfd84)
</details>

## v0.1.0 (2020-07-12)

### other

 - <csr-id-ab6f98b905f13ed2a7c0c483f34fab63141fbc5b/> try-join with static typing works, but…

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 12 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable (5688a34)
    - Flume isn't actually needed for that… (c750022)
    - Don't just ignore send errors - we should panic for now (f128117)
    - Proper implementation of line renderer into 'lean' CLI (e98e7c2)
    - upgrade to prodash version 7 (af02b46)
    - update prodash to verion 6.0 (a4731a3)
    - Add metadata to allow docs.rs build all featueres (10f9386)
    - Switch to prodash 5.0 for windows support (88542e1)
    - Allow to limit the logging depth for less cluttered output (fce7035)
    - finally speed up logging progress properly - needs input throttling (1a550c6)
    - Avoid calling system time too often in logs, it reduced performance (b17bd76)
    - Revert "ABORT: try-join with static typing works, but…" (b8b979b)
    - try-join with static typing works, but… (ab6f98b)
    - Remove dependency to git-object from git-features - it better remains free (67c3a6a)
    - \#[forbid(unsafe)] for all crates (afda803)
    - Allow for more screen space when formatting (6794300)
    - refactor (7add82c)
    - Automatically close the TUI when there is no progress anymore. (c416152)
    - pretty progress in a generalized form (caa883b)
    - express DoOrDiscard in terms of Either (progress) (cb29a45)
    - Provide 'either' type with implementation for Progress (237bb5e)
    - better trait bounds of `in_parallel_if`… (6264f2f)
    - First implementation of logging per thread (477dd90)
    - Support for providing progress to threads (2815858)
    - first very basic progress implementation (b820717)
    - Pass progress everywhere, for now just to discard it (da3ae1c)
    - Implement `Progress` trait for prodash::tree::Item (0eeb6d7)
    - implement progress trait for logs with throttling (287eca9)
    - Add 'fast-sha1' to git-features (b22541f)
    - A new crate to represent features that can toggle from the top-level (23c420c)
</details>

