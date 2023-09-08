# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.8.0 (2023-09-08)

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 6 calendar days.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'adjustments-for-cargo' ([`b7560a2`](https://github.com/Byron/gitoxide/commit/b7560a2445b62f888bf5aa2ba4c5a47ae037cb23))
</details>

## 0.7.4 (2023-09-01)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.4, gix-index v0.23.0, safety bump 5 crates ([`3be2b1c`](https://github.com/Byron/gitoxide/commit/3be2b1ccfe30eeae45711c64b88efc522a2b51b7))
    - Prepare `gix-index` release ([`6fdbc66`](https://github.com/Byron/gitoxide/commit/6fdbc667c20f10734390341b435c15c73b7cd227))
    - Thanks clippy ([`5044c3b`](https://github.com/Byron/gitoxide/commit/5044c3b87456cf58ebfbbd00f23c9ba671cb290c))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.7.3 (2023-08-22)

<csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/>

### Chore

 - <csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/> don't call crate 'WIP' in manifest anymore.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 4 calendar days.
 - 15 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
</details>

## 0.7.2 (2023-08-07)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 4 calendar days.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#961](https://github.com/Byron/gitoxide/issues/961)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#961](https://github.com/Byron/gitoxide/issues/961)**
    - Add another fuzz-failure that doesn't fail ([`90ec770`](https://github.com/Byron/gitoxide/commit/90ec77057fa8a7d191c959e2f3a3c958270803eb))
 * **Uncategorized**
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Prepare changelogs prior to release of `gix-submodule` ([`f3c4311`](https://github.com/Byron/gitoxide/commit/f3c43110e8d5f16cf87e50821044d8b3edbae235))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
    - Update `time` crate explicitly in Cargo.toml to latest version ([`e145a74`](https://github.com/Byron/gitoxide/commit/e145a7489dd5e1a7c3458428ecbd101e7b53536b))
</details>

## 0.7.1 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 23 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.7.0 (2023-06-29)

### New Features (BREAKING)

 - <csr-id-09fbfd2ac3100947c8ea96fdf6425032354aee1b/> support dates prior to the UNIX epoch.
   This means such dates can now be parsed and represented.
   We do this by switching the time representation from `u64` which
   is used in `git` to `i64` which is used in `libgit2`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Prepare changelogs prior to release ([`00f96fb`](https://github.com/Byron/gitoxide/commit/00f96fb3110a8f81a1bd0d74c757c15b8773c6f6))
    - Merge branch 'i64-times' ([`b407461`](https://github.com/Byron/gitoxide/commit/b407461d8991db67a5bdb2ab13f518f78a85ed40))
    - Adapt to changes in `gix-date` ([`fba45c6`](https://github.com/Byron/gitoxide/commit/fba45c68d57d5f73070a6949556a04187d42e427))
    - Support dates prior to the UNIX epoch. ([`09fbfd2`](https://github.com/Byron/gitoxide/commit/09fbfd2ac3100947c8ea96fdf6425032354aee1b))
</details>

## 0.6.0 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### New Features (BREAKING)

 - <csr-id-4bc0ae1eb313ad83c793f397af9ca791b9b171e5/> Represent time as 64 bit integer.
   That way, dates beyond 2038 are supported.
   
   Further, we rename `Time::seconds_since_unix_epoch` to `seconds`, and remove `seconds()`
   as there now is a new type, `SecondsSinceUnixEpoch`. In the same vein, we rename
   `Time::offset_in_seconds` to `offset` as it now has at type `OffsetInSeconds`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 11 calendar days.
 - 15 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Prepare changelogs prior to release ([`18b0a37`](https://github.com/Byron/gitoxide/commit/18b0a371941aa2d4d62512437d5daa351ba99ffd))
    - `just fmt` ([`871dd0b`](https://github.com/Byron/gitoxide/commit/871dd0b977caf17159092a4739ba5408403cdb2c))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Merge branch 'help-874-redundant-closures' ([`fe59956`](https://github.com/Byron/gitoxide/commit/fe59956ad667303a923d7cfd9ffd72283df41d78))
    - Add `clippy::redundant-closure-for-method-calls` lint ([`bcad5c2`](https://github.com/Byron/gitoxide/commit/bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d))
    - Merge branch 'future-dates' ([`8d2e6a9`](https://github.com/Byron/gitoxide/commit/8d2e6a91ac92a033e9e3daad5cffa90263075536))
    - Represent time as 64 bit integer. ([`4bc0ae1`](https://github.com/Byron/gitoxide/commit/4bc0ae1eb313ad83c793f397af9ca791b9b171e5))
</details>

## 0.5.1 (2023-06-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 12 calendar days.
 - 48 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
</details>

## 0.5.0 (2023-04-19)

### New Features (BREAKING)

 - <csr-id-b83ee366a3c65c717beb587ad809268f1c54b8ad/> Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability.
   With it it's possible to not automatically declare all optional dependencies externally visible
   features, and thus re-use feature names that oterwise are also a crate name.
   
   Previously I thought that `serde1` is for future-proofing and supporting multiple serde versions
   at the same time. However, it's most definitely a burden I wouldn't want anyway, so using
   `serde` seems to be the way to go into the future.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 57 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#814](https://github.com/Byron/gitoxide/issues/814)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#814](https://github.com/Byron/gitoxide/issues/814)**
    - Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability. ([`b83ee36`](https://github.com/Byron/gitoxide/commit/b83ee366a3c65c717beb587ad809268f1c54b8ad))
 * **Uncategorized**
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Prepare changelog prior to release ([`7f06458`](https://github.com/Byron/gitoxide/commit/7f064583bd0e1b078df89a7750f5a25deb70f516))
    - Merge branch 'main' into dev ([`cdef398`](https://github.com/Byron/gitoxide/commit/cdef398c4a3bd01baf0be2c27a3f77a400172b0d))
    - Rename the serde1 feature to serde ([`19338d9`](https://github.com/Byron/gitoxide/commit/19338d934b6712b7d6bd3fa3b2e4189bf7e6c8a1))
</details>

## 0.4.3 (2023-02-20)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.
 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.4.3, gix-hash v0.10.3, gix-features v0.26.5, gix-actor v0.17.2, gix-glob v0.5.5, gix-path v0.7.2, gix-quote v0.4.2, gix-attributes v0.8.3, gix-validate v0.7.3, gix-object v0.26.2, gix-ref v0.24.1, gix-config v0.16.2, gix-command v0.2.4, gix-url v0.13.3, gix-credentials v0.9.2, gix-discover v0.13.1, gix-index v0.12.4, gix-mailmap v0.9.3, gix-pack v0.30.3, gix-packetline v0.14.3, gix-transport v0.25.6, gix-protocol v0.26.4, gix-revision v0.10.4, gix-refspec v0.7.3, gix-worktree v0.12.3, gix v0.36.1 ([`9604783`](https://github.com/Byron/gitoxide/commit/96047839a20a657a559376b0b14c65aeab96acbd))
    - Compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
</details>

## 0.4.2 (2023-02-17)

<csr-id-41fc2bb20e6a926ffc3638c0fac21d733fdc2e3c/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Other (BREAKING)

 - <csr-id-41fc2bb20e6a926ffc3638c0fac21d733fdc2e3c/> `time::format::GIT_DEFAULT` -> `*::DEFAULT` and `*::DEFAULT` -> `*::GITOXIDE`.
   That way we properly indicate what we are doing and don't try to somewhat
   sneakily suggest that the default for git dates is something else
   due to personal preference.

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-c24ea67f84aa48953949682672114715bee67432/> parse now takes the current time `parse(…, Option<time>)` as parameter.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### New Features

 - <csr-id-fe04934d783e4c53a79fae2e0d3b0c5802ea1809/> Adds fuzzer for date parser
 - <csr-id-f5c9aa827e3b9ffb82a52ad7f840c58aa0d654ed/> return the time that failed to parse in the error
 - <csr-id-4066ac7367d8e870522746429513fb7a357a2cc6/> Support git default date format
   This is the format output by default by `git log` or when using
   `--pretty=%ad`.
   
   The new git_date::time::format::GIT_DEFAULT format description may be used
   to output date strings in this format. It is also now used by
   git_date::parse() to accept date strings that may be in this format.
 - <csr-id-8094351fe547a0f6756b0ed29dc87a0e6b9ceec1/> Format git-style RFC 2822 date strings
   Git outputs the day-of-month field as a non-padded number whereas strict
   RFC 2822 date strings are supposed to use a zero-padded two-digit number.
   
   The new git_date::time::format::GIT_RFC2822 format description allows Time
   to be formatted in git's RFC 2822 style. (Whereas the existing RFC2822
   format description produces a strict RFC 2822 date string).
 - <csr-id-034c8dc4437c06dc8af702fbe9b86ec903c73a18/> bump version to 1.0 to prevent accidental inclusions downstream
   For some reason, cargo considers different patch releases breaking, so
   creating a new patch can break installation of gitoxide entirely.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-8f7f9ce2b06ec884220b8cd5010b3df04b1ff0bc/> Raw and Unix formats.
 - <csr-id-4b0c2198f9d5b28584c717123c7cfb1b27724605/> Add ISO-strict format
 - <csr-id-c76fde7de278b49ded13b655d5345e4eb8c1b134/> initialize `Time` from `now_utc` and `now_local`
   Localtime support depends on some other factors now, but that
   will only get better over time.
   
   We might have to document `unsound_local_time` at some point.
 - <csr-id-aeda76ed500d2edba62747d667227f2664edd267/> `Time::is_set()` to see if the time is more than just the default.
 - <csr-id-cfb6a726ddb763f7c22688f8ef309e719c2dfce4/> Add `Time` type.
   It was originally from the `git-actor` crate.

### Bug Fixes

<csr-id-dff0aa0be600b9cd9518184fefa9b3c8fdb510f2/>
<csr-id-39655f5f6fa39a55c4420f672e866c483f9b85ed/>
<csr-id-f4ea59db0a429801ab40b1294da4bffd9e0f80b3/>
<csr-id-be603f593055309b74685bc2aebb8e35e6de2d59/>

 - <csr-id-786f6dc5c1f765b9598cd55ca8fb1714ad177e46/> prevent panics from dates which cannot be represented by the `time` crate
 - <csr-id-3d6c81000559df91b17834ec5e9830b085277af8/> panic in `parse_raw()` (as found by fuzzer)
 - <csr-id-046af94f005a6e095f0d3616c0b57ef1f556f734/> Stricter raw date parsing
   The raw date parser (git_date::parse::function::parse_raw()) accepted some
   inputs that it should not have. Specifically, it would accept:
   
   - Any character for the timezone offset's sign

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 186 commits contributed to the release over the course of 309 calendar days.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#331](https://github.com/Byron/gitoxide/issues/331), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691), [#711](https://github.com/Byron/gitoxide/issues/711), [#720](https://github.com/Byron/gitoxide/issues/720)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Initialize `Time` from `now_utc` and `now_local` ([`c76fde7`](https://github.com/Byron/gitoxide/commit/c76fde7de278b49ded13b655d5345e4eb8c1b134))
    - `Time::is_set()` to see if the time is more than just the default. ([`aeda76e`](https://github.com/Byron/gitoxide/commit/aeda76ed500d2edba62747d667227f2664edd267))
    - Frame for git-date ([`37e8ef8`](https://github.com/Byron/gitoxide/commit/37e8ef890305db0798059919290a0d27a9a39194))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - Git-style disambiguation errors ([`5717194`](https://github.com/Byron/gitoxide/commit/57171946081c03da98f3d33f5b963c3bc4b2d6d9))
    - Reflog lookup by date is complete ([`b3d009e`](https://github.com/Byron/gitoxide/commit/b3d009e80e3e81afd3d095fa2d7b5fc737d585c7))
    - Add `Time` type. ([`cfb6a72`](https://github.com/Byron/gitoxide/commit/cfb6a726ddb763f7c22688f8ef309e719c2dfce4))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **[#711](https://github.com/Byron/gitoxide/issues/711)**
    - Assure we get the latest version of the `time` crate ([`cb31cd1`](https://github.com/Byron/gitoxide/commit/cb31cd16bc4a6e749c298cfbc7e0dad05b11b96c))
 * **[#720](https://github.com/Byron/gitoxide/issues/720)**
    - Prevent panics from dates which cannot be represented by the `time` crate ([`786f6dc`](https://github.com/Byron/gitoxide/commit/786f6dc5c1f765b9598cd55ca8fb1714ad177e46))
 * **Uncategorized**
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - Rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - Adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - Adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - Adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - Adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - Adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - Adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - Adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - Adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - Adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - Adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - Adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - Adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - Adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - Adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - Adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - Adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - Adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - Adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - Adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - Adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - Adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - Rename `git-date` to `gix-date` ([`2b0a966`](https://github.com/Byron/gitoxide/commit/2b0a9662acf7c74f594138963e2efd8f576799f3))
    - Adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - Adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - Adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - Adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - Adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - Adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - Adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - Adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - Adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - Adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - Adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - Adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - Adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - Adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - Adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Merge branch 'adjustments-for-cargo' ([`7bba270`](https://github.com/Byron/gitoxide/commit/7bba2709488b7eb999b8136dbab03af977241678))
    - Merge branch 'fix-git-date-panics' ([`56f5593`](https://github.com/Byron/gitoxide/commit/56f5593b25e300d21c380c5fb5a184445ff26183))
    - Panic in `parse_raw()` (as found by fuzzer) ([`3d6c810`](https://github.com/Byron/gitoxide/commit/3d6c81000559df91b17834ec5e9830b085277af8))
    - Fix warnings, don't track Cargo.lock to use compatible latest dependencies ([`96a56a9`](https://github.com/Byron/gitoxide/commit/96a56a9d1d76e5832a4bf505152985a74c6c7357))
    - Merge pull request #714 from silvergasp/fuzz-git-date ([`a52c54e`](https://github.com/Byron/gitoxide/commit/a52c54e97698c1b61ff70884378338f63b4d1a27))
    - Adds fuzzer for date parser ([`fe04934`](https://github.com/Byron/gitoxide/commit/fe04934d783e4c53a79fae2e0d3b0c5802ea1809))
    - Optimize usage of `hex_to_id()` ([`6fa950d`](https://github.com/Byron/gitoxide/commit/6fa950d0ab1991a5577c06385169be1b390dd88a))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - Return the time that failed to parse in the error ([`f5c9aa8`](https://github.com/Byron/gitoxide/commit/f5c9aa827e3b9ffb82a52ad7f840c58aa0d654ed))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - Thanks clippy ([`b34c9fe`](https://github.com/Byron/gitoxide/commit/b34c9fe58223862712eacc1cb7353e497a4b9778))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Prepare changelogs prior to release ([`d679f5b`](https://github.com/Byron/gitoxide/commit/d679f5b6f018633e858d3ebbdaf1cd5098bbc5e7))
    - `time::format::GIT_DEFAULT` -> `*::DEFAULT` and `*::DEFAULT` -> `*::GITOXIDE`. ([`41fc2bb`](https://github.com/Byron/gitoxide/commit/41fc2bb20e6a926ffc3638c0fac21d733fdc2e3c))
    - Merge branch 'strict-raw-dates' ([`c65ce7e`](https://github.com/Byron/gitoxide/commit/c65ce7e3031b036d3a76b6e8a6c9ead39390261c))
    - Stricter raw date parsing ([`046af94`](https://github.com/Byron/gitoxide/commit/046af94f005a6e095f0d3616c0b57ef1f556f734))
    - Merge branch 'issue-679' ([`a910d9e`](https://github.com/Byron/gitoxide/commit/a910d9e7dcb2ba1979660165fa5b8cb0a2dce594))
    - Refactor ([`26597b9`](https://github.com/Byron/gitoxide/commit/26597b983d401a1efcd13b3e69aad6a39581ec0b))
    - Support git default date format ([`4066ac7`](https://github.com/Byron/gitoxide/commit/4066ac7367d8e870522746429513fb7a357a2cc6))
    - Format git-style RFC 2822 date strings ([`8094351`](https://github.com/Byron/gitoxide/commit/8094351fe547a0f6756b0ed29dc87a0e6b9ceec1))
    - Parse git-styled RFC 2822 date strings ([`dff0aa0`](https://github.com/Byron/gitoxide/commit/dff0aa0be600b9cd9518184fefa9b3c8fdb510f2))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Merge branch 'bugfix/system-time-correct-offset-sign' ([`6e40433`](https://github.com/Byron/gitoxide/commit/6e40433f6f607888e8f8a6c36e53a68b91fcf671))
    - Add non-isolated test that, depending on region, would catch the invalid-sign bug. ([`b649965`](https://github.com/Byron/gitoxide/commit/b6499653b71e79f17a7304c6e83d2e1776ff5d5e))
    - Negative system timezone offsets should be serialized as such ([`39655f5`](https://github.com/Byron/gitoxide/commit/39655f5f6fa39a55c4420f672e866c483f9b85ed))
    - Adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'bugfix/signed-raw-time' ([`f50b9f5`](https://github.com/Byron/gitoxide/commit/f50b9f54425e64461a31d00e082470aa5042be74))
    - Thanks clippy ([`75d6e88`](https://github.com/Byron/gitoxide/commit/75d6e882cea823100f2ad5bf26a4f1082287d80b))
    - Refactor ([`f4e8051`](https://github.com/Byron/gitoxide/commit/f4e8051fbc8cde9ba25fb1185c9e32f6aed4c0fb))
    - Correctly parse raw dates with negative timezone offsets ([`f4ea59d`](https://github.com/Byron/gitoxide/commit/f4ea59db0a429801ab40b1294da4bffd9e0f80b3))
    - Extend git-date's baseline tests to also re-format the parsed dates ([`9f95f7f`](https://github.com/Byron/gitoxide/commit/9f95f7fbbe5b56e65c00c26f580bf67a4001e146))
    - Merge branch 'bugfix/timestamp-to-datetime-conversion' ([`be0bbf5`](https://github.com/Byron/gitoxide/commit/be0bbf519c4a6687c305717ec0c12215a5836f58))
    - Always consider timestamps as UTC when loading from commits ([`be603f5`](https://github.com/Byron/gitoxide/commit/be603f593055309b74685bc2aebb8e35e6de2d59))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'git_date_relative' ([`83a3832`](https://github.com/Byron/gitoxide/commit/83a38329c59e9ebc057221da832fd8320bbeddb1))
    - Refactor ([`c5c6bf6`](https://github.com/Byron/gitoxide/commit/c5c6bf6ef3f0c9c12389bb638ab4d32b61839dec))
    - Refactor ([`956613f`](https://github.com/Byron/gitoxide/commit/956613fcdb33a845526fa9743aa0e7f80b3badfa))
    - Refactor ([`1026b7c`](https://github.com/Byron/gitoxide/commit/1026b7c613a3a8b46a27dd7cd5e3520043b21ab7))
    - WIP. ([`79d82d4`](https://github.com/Byron/gitoxide/commit/79d82d46613c83280d2401ef4d72a35010a70b87))
    - Parse the output while parsing the baseline file. ([`70fe59f`](https://github.com/Byron/gitoxide/commit/70fe59f4a1cad25f687397206ee2cbe50e643181))
    - Make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Parse now takes the current time `parse(…, Option<time>)` as parameter. ([`c24ea67`](https://github.com/Byron/gitoxide/commit/c24ea67f84aa48953949682672114715bee67432))
    - Merge branch 'git_date_parse' ([`75591fb`](https://github.com/Byron/gitoxide/commit/75591fb108ce440ba2f920bebf99158b407e3046))
    - Thanks clippy ([`590fcc9`](https://github.com/Byron/gitoxide/commit/590fcc9f3fc768802fd09f4e73036e225ec5c363))
    - A sample on how to more easily test relative date parsing ([`c585c9b`](https://github.com/Byron/gitoxide/commit/c585c9b2b0e628914169bbfba55aa5130da51a83))
    - Add test to check times before unix epoch ([`eb304ea`](https://github.com/Byron/gitoxide/commit/eb304ea91f857070380ef4e4dbdff6b8ab89cee1))
    - Refactor ([`0e231eb`](https://github.com/Byron/gitoxide/commit/0e231ebeeb41306b1075bc06c78b45dc65ded5fa))
    - Refactor ([`5793465`](https://github.com/Byron/gitoxide/commit/5793465b0f19b284a4615290e2b08203e969e9bb))
    - Refactor; add failing test to see invalid date error in action ([`90008aa`](https://github.com/Byron/gitoxide/commit/90008aa4e59f78cfe2ecd5a7ee851bb56f7b0d33))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - PR comments. ([`1eac4de`](https://github.com/Byron/gitoxide/commit/1eac4de99de250e770b48054fd5bc6b806418e4d))
    - `parse` is pure function. ([`9ad1a5f`](https://github.com/Byron/gitoxide/commit/9ad1a5fa2ce54e978396ff3eaa7061a8edd10d4a))
    - Fallible timestamp cast i64 -> u32. ([`cce7616`](https://github.com/Byron/gitoxide/commit/cce7616835f1dff29b2c74bd96b5238d55167d5f))
    - `parse()` returns Result. ([`206f392`](https://github.com/Byron/gitoxide/commit/206f3923f5da2e9e26677e917550e6e5baa2913a))
    - Add output to baseline. ([`5c3b733`](https://github.com/Byron/gitoxide/commit/5c3b7334289ef71a9dbcc021f1989eb8e021fc7f))
    - `parse` returns Result. ([`67c8c6a`](https://github.com/Byron/gitoxide/commit/67c8c6a9666161a3a0cd01203e0fb1da22336939))
    - Add fixtures. ([`6c40ac1`](https://github.com/Byron/gitoxide/commit/6c40ac1e39fbefba640fc9cb4fd3ef419a149e99))
    - Add git baseline. ([`b747a60`](https://github.com/Byron/gitoxide/commit/b747a60b62c533b3919124e628f31e6bbef9c838))
    - Refactor ([`3e6e0f9`](https://github.com/Byron/gitoxide/commit/3e6e0f9ebdf7ab87c689be074ae1eecfea4485f0))
    - Draft. ([`95b4902`](https://github.com/Byron/gitoxide/commit/95b490230ebfbd4c2f04edd7074d5a0f5e2429ec))
    - Draft. ([`43b6c06`](https://github.com/Byron/gitoxide/commit/43b6c064bfa407d1a2cabf09aad1cc8334dec651))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Merge branch 'fix-ci-installation' ([`9245083`](https://github.com/Byron/gitoxide/commit/92450839621a4d99cb22d08cbf9f9a89ff6b9e3f))
    - Release git-date v0.1.0, git-actor v0.11.4, git-revision v0.4.3, git-repository v0.22.1, cargo-smart-release v0.11.0, git-commitgraph v0.8.2, gitoxide-core v0.17.0, gitoxide v0.15.0 ([`1fb931a`](https://github.com/Byron/gitoxide/commit/1fb931a7ea59f1cf895a6c1392fd8615b723c743))
    - Update changelogs prior to release ([`23cb58f`](https://github.com/Byron/gitoxide/commit/23cb58f02043e0e5027136fd6e8e724c03a2efbe))
    - Bump version to 1.0 to prevent accidental inclusions downstream ([`034c8dc`](https://github.com/Byron/gitoxide/commit/034c8dc4437c06dc8af702fbe9b86ec903c73a18))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge pull request #497 from svetli-n/patch-2 ([`bd02b39`](https://github.com/Byron/gitoxide/commit/bd02b392734d8074adedc504a2cf69952d6fa980))
    - Fix doc comment ([`51cd9ce`](https://github.com/Byron/gitoxide/commit/51cd9ceda6a8a0127a18802dc2cc49861013a65d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`c82bbfa`](https://github.com/Byron/gitoxide/commit/c82bbfaddc45bf9b5b55f056613046d977d9ef09))
    - Release git-date v0.0.4, git-actor v0.11.2, git-revision v0.4.1, git-repository v0.21.1 ([`2f9dc84`](https://github.com/Byron/gitoxide/commit/2f9dc847e0d54f4181ce35ddadd9286ba80ca01f))
    - Update changelogs prior to release ([`1b5fd86`](https://github.com/Byron/gitoxide/commit/1b5fd86d121634f8567e8442f125377e460032c6))
    - Prepare for release of git-repository ([`8aa5389`](https://github.com/Byron/gitoxide/commit/8aa5389d5a1bdd3a07f1caa1c2f55c8af4f9844a))
    - Merge branch 'add_common_git_formats' ([`c53e5a4`](https://github.com/Byron/gitoxide/commit/c53e5a4c521fbae7d74ad8323f79ced4dfe4f037))
    - Raw and Unix formats. ([`8f7f9ce`](https://github.com/Byron/gitoxide/commit/8f7f9ce2b06ec884220b8cd5010b3df04b1ff0bc))
    - Foundation for custom formats that aren't easily done with `time` formatting ([`b74eaf8`](https://github.com/Byron/gitoxide/commit/b74eaf85d41e1ec67d8c84cc8484702514c3e7cd))
    - Add ISO-strict format ([`4b0c219`](https://github.com/Byron/gitoxide/commit/4b0c2198f9d5b28584c717123c7cfb1b27724605))
    - Refinements ([`b1fea0f`](https://github.com/Byron/gitoxide/commit/b1fea0fe76bd94850c7da34ee9504525ad667748))
    - Add common git date formats. ([`090795b`](https://github.com/Byron/gitoxide/commit/090795b4040e2dad995390e502f87c2ced8045f8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - Adjust `git_date::parsea(str)` to use a str ([`0f8680a`](https://github.com/Byron/gitoxide/commit/0f8680a60913556b7fbd7543fda6a409ac05b121))
    - Refactor ([`11a5fa2`](https://github.com/Byron/gitoxide/commit/11a5fa29615d47c24f78446a1c3f5d3b8acf2f93))
    - Refactor ([`8e6f4a9`](https://github.com/Byron/gitoxide/commit/8e6f4a921b6b45945e711aaf5858b7714371fb41))
    - Merge branch 'format_git_date_time' ([`99e12be`](https://github.com/Byron/gitoxide/commit/99e12bee16ab3f344c71818bfd1c95cf50e1721b))
    - Thanks clipppy ([`b139d70`](https://github.com/Byron/gitoxide/commit/b139d7043fbbbe5b933d96e83544059fe2a7bdd8))
    - Refactor ([`bd64387`](https://github.com/Byron/gitoxide/commit/bd64387d8ad3377571755dff14577cc3c53ee9cc))
    - Use time format strings. ([`f84e8f5`](https://github.com/Byron/gitoxide/commit/f84e8f5f16ec2197d1967fb1cc06e9609ea52c16))
    - Refactor ([`556dd8c`](https://github.com/Byron/gitoxide/commit/556dd8cb78ea9321031984e2c6b4f9bc415f1be5))
    - Refactor ([`5bbcbcd`](https://github.com/Byron/gitoxide/commit/5bbcbcd75d1ab26746da7a927390ff3b6cc19a85))
    - Format `git-date::Time` with `time::format_description`. ([`d4243bc`](https://github.com/Byron/gitoxide/commit/d4243bc4feb994bde99156ba77fff63bc9c875e9))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge branch 'config-cascade' ([`f144eaf`](https://github.com/Byron/gitoxide/commit/f144eaf5863ae5cac63103f0db51c35fcf03a948))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'revspec-parsing' ([`a2c8969`](https://github.com/Byron/gitoxide/commit/a2c8969ba821fd387c39b14248074767f54749c8))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Release git-date v0.0.0 ([`2bc2f76`](https://github.com/Byron/gitoxide/commit/2bc2f765dc4f8a4779c132f7729fb782c66c0d99))
</details>

## 0.4.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.4.0 (2023-01-06)

<csr-id-41fc2bb20e6a926ffc3638c0fac21d733fdc2e3c/>

### New Features

 - <csr-id-4066ac7367d8e870522746429513fb7a357a2cc6/> Support git default date format
   This is the format output by default by `git log` or when using
   `--pretty=%ad`.
   
   The new gix_date::time::format::GIT_DEFAULT format description may be used
   to output date strings in this format. It is also now used by
   gix_date::parse() to accept date strings that may be in this format.
 - <csr-id-8094351fe547a0f6756b0ed29dc87a0e6b9ceec1/> Format git-style RFC 2822 date strings
   Git outputs the day-of-month field as a non-padded number whereas strict
   RFC 2822 date strings are supposed to use a zero-padded two-digit number.
   
   The new gix_date::time::format::GIT_RFC2822 format description allows Time
   to be formatted in git's RFC 2822 style. (Whereas the existing RFC2822
   format description produces a strict RFC 2822 date string).

### Bug Fixes

<csr-id-dff0aa0be600b9cd9518184fefa9b3c8fdb510f2/>

 - <csr-id-046af94f005a6e095f0d3616c0b57ef1f556f734/> Stricter raw date parsing
   The raw date parser (gix_date::parse::function::parse_raw()) accepted some
   inputs that it should not have. Specifically, it would accept:
   
   - Any character for the timezone offset's sign

### Other (BREAKING)

 - <csr-id-41fc2bb20e6a926ffc3638c0fac21d733fdc2e3c/> `time::format::GIT_DEFAULT` -> `*::DEFAULT` and `*::DEFAULT` -> `*::GITOXIDE`.
   That way we properly indicate what we are doing and don't try to somewhat
   sneakily suggest that the default for git dates is something else
   due to personal preference.

## 0.3.1 (2022-12-19)

### Bug Fixes

 - <csr-id-39655f5f6fa39a55c4420f672e866c483f9b85ed/> Negative system timezone offsets should be serialized as such
 - <csr-id-f4ea59db0a429801ab40b1294da4bffd9e0f80b3/> correctly parse raw dates with negative timezone offsets
 - <csr-id-be603f593055309b74685bc2aebb8e35e6de2d59/> always consider timestamps as UTC when loading from commits

## 0.3.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.2.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-c24ea67f84aa48953949682672114715bee67432/> parse now takes the current time `parse(…, Option<time>)` as parameter.

## 0.1.0 (2022-08-24)

### New Features

 - <csr-id-034c8dc4437c06dc8af702fbe9b86ec903c73a18/> bump version to 1.0 to prevent accidental inclusions downstream
   For some reason, cargo considers different patch releases breaking, so
   creating a new patch can break installation of gitoxide entirely.

## 0.0.5 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

## 0.0.4 (2022-08-19)

### New Features

 - <csr-id-8f7f9ce2b06ec884220b8cd5010b3df04b1ff0bc/> Raw and Unix formats.
 - <csr-id-4b0c2198f9d5b28584c717123c7cfb1b27724605/> Add ISO-strict format

## 0.0.3 (2022-08-17)

A maintenance release without user facing changes.

## 0.0.2 (2022-07-22)

### New Features

 - <csr-id-c76fde7de278b49ded13b655d5345e4eb8c1b134/> initialize `Time` from `now_utc` and `now_local`
   Localtime support depends on some other factors now, but that
   will only get better over time.
   
   We might have to document `unsound_local_time` at some point.
 - <csr-id-aeda76ed500d2edba62747d667227f2664edd267/> `Time::is_set()` to see if the time is more than just the default.

## 0.0.1 (2022-06-13)

### New Features

 - <csr-id-cfb6a726ddb763f7c22688f8ef309e719c2dfce4/> Add `Time` type.
   It was originally from the `gix-actor` crate.

## 0.0.0 (2022-04-14)

An empty crate without any content to reserve the name for the gitoxide project.

