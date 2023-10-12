# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.13.1 (2023-10-12)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 33 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Merge branch 'improvements' ([`429e7b2`](https://github.com/Byron/gitoxide/commit/429e7b25f93c8a7947db19bafa74babf199a1aa6))
    - Add another doc-alias for a commonly used hash method ([`ed458c0`](https://github.com/Byron/gitoxide/commit/ed458c08133ddf1ea3b05b3c95c6534f80763259))
</details>

## 0.13.0 (2023-09-08)

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.12.0 (2023-08-22)

### New Features (BREAKING)

 - <csr-id-bd86f838a304e154828fb3fa1403ea2b713f95a6/> improve hex-parsing performance.
   This improvementment is measurable in real-world applications.
   
   The reason for the breaking change is that the error type changed
   slightly.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 5 calendar days.
 - 30 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Merge branch 'faster-hex' ([`4a4fa0f`](https://github.com/Byron/gitoxide/commit/4a4fa0fcdaa6e14b51d3f03f5d7c5b65042667bf))
    - Improve hex-parsing performance. ([`bd86f83`](https://github.com/Byron/gitoxide/commit/bd86f838a304e154828fb3fa1403ea2b713f95a6))
</details>

## 0.11.4 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 30 days passed between releases.
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

## 0.11.3 (2023-06-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 5 calendar days.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Prepare changelogs prior to release ([`18b0a37`](https://github.com/Byron/gitoxide/commit/18b0a371941aa2d4d62512437d5daa351ba99ffd))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
</details>

## 0.11.2 (2023-06-06)

### New Features

 - <csr-id-8d645b0494bb0376358fcf6e57a37b865ca91a96/> add `ObjectId::is_empty_tree()` as sybling of `::is_empty_blob()`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 12 calendar days.
 - 41 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Add `ObjectId::is_empty_tree()` as sybling of `::is_empty_blob()`. ([`8d645b0`](https://github.com/Byron/gitoxide/commit/8d645b0494bb0376358fcf6e57a37b865ca91a96))
    - Merge branch 'fix-docs' ([`420553a`](https://github.com/Byron/gitoxide/commit/420553a10d780e0b2dc466cac120989298a5f187))
    - Cleaning up documentation ([`2578e57`](https://github.com/Byron/gitoxide/commit/2578e576bfa365d194a23a1fb0bf09be230873de))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Auto-fix as many 'range-plus-one' lints as possible ([`4795fcf`](https://github.com/Byron/gitoxide/commit/4795fcf6adb06b792592a1b11a3f071e9d47b1ac))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
</details>

## 0.11.1 (2023-04-26)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 4 calendar days.
 - 7 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'fix-823' ([`6ebd61e`](https://github.com/Byron/gitoxide/commit/6ebd61e548a36a04e413ac725a03e607a3588334))
    - Thanks clippy ([`14e64e7`](https://github.com/Byron/gitoxide/commit/14e64e74649cfb1f2f99da87015939af98fae5c8))
</details>

## 0.11.0 (2023-04-19)

### New Features (BREAKING)

 - <csr-id-b83ee366a3c65c717beb587ad809268f1c54b8ad/> Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability.
   With it it's possible to not automatically declare all optional dependencies externally visible
   features, and thus re-use feature names that oterwise are also a crate name.
   
   Previously I thought that `serde1` is for future-proofing and supporting multiple serde versions
   at the same time. However, it's most definitely a burden I wouldn't want anyway, so using
   `serde` seems to be the way to go into the future.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
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
    - Refactor ([`691758a`](https://github.com/Byron/gitoxide/commit/691758a4491f8430b61e418dad33d8d901f89361))
    - Streamline status API ([`0f747f3`](https://github.com/Byron/gitoxide/commit/0f747f303089fd862c24d4ad93b75d3064c9328b))
</details>

## 0.10.4 (2023-04-12)

### Bug Fixes

 - <csr-id-28168eee8ea5926793ef509ad56ee1a00c230ca6/> make `oid` `Hash` work on 32-bit targets

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 50 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.10.4, gix-hashtable v0.1.3 ([`b574a39`](https://github.com/Byron/gitoxide/commit/b574a3904203762a6b9e475e16a7c358d7616599))
    - Prepare changelogs prior to release ([`a0a938f`](https://github.com/Byron/gitoxide/commit/a0a938f8fd3442259ad3008778738f91bbc6e1a9))
    - Merge branch 'patch-1' ([`b02bf24`](https://github.com/Byron/gitoxide/commit/b02bf247890c873184e58f734e0912eac6c6bbae))
    - Add comment explaining `oid::hash` ([`cfd93f6`](https://github.com/Byron/gitoxide/commit/cfd93f686dac752a7dc9b7fef3b7a706f265c6b0))
    - Clippy ([`b2a5b9d`](https://github.com/Byron/gitoxide/commit/b2a5b9dc50ae2a26672c2ad8a586beca3e659361))
    - Make `oid` `Hash` work on 32-bit targets ([`28168ee`](https://github.com/Byron/gitoxide/commit/28168eee8ea5926793ef509ad56ee1a00c230ca6))
</details>

## 0.10.3 (2023-02-20)

### New Features

 - <csr-id-c817626a501dd3c8edd444c6e31ba04d9da31776/> add `ObjectId::empty_blob()` to obtain the empty blob object.

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.4.3, gix-hash v0.10.3, gix-features v0.26.5, gix-actor v0.17.2, gix-glob v0.5.5, gix-path v0.7.2, gix-quote v0.4.2, gix-attributes v0.8.3, gix-validate v0.7.3, gix-object v0.26.2, gix-ref v0.24.1, gix-config v0.16.2, gix-command v0.2.4, gix-url v0.13.3, gix-credentials v0.9.2, gix-discover v0.13.1, gix-index v0.12.4, gix-mailmap v0.9.3, gix-pack v0.30.3, gix-packetline v0.14.3, gix-transport v0.25.6, gix-protocol v0.26.4, gix-revision v0.10.4, gix-refspec v0.7.3, gix-worktree v0.12.3, gix v0.36.1 ([`9604783`](https://github.com/Byron/gitoxide/commit/96047839a20a657a559376b0b14c65aeab96acbd))
    - Merge branch 'empty-blob' ([`796f298`](https://github.com/Byron/gitoxide/commit/796f2982ebc0b7682d0ee8987221fd3397e631aa))
    - Refactor ([`231b268`](https://github.com/Byron/gitoxide/commit/231b268964c8e066aa58132978068be7c50b6d63))
    - Add `ObjectId::empty_blob()` to obtain the empty blob object. ([`c817626`](https://github.com/Byron/gitoxide/commit/c817626a501dd3c8edd444c6e31ba04d9da31776))
</details>

## 0.10.2 (2023-02-17)

<csr-id-8be4036dce4a857cc14a8b9467aaf2fc0fc2e827/>
<csr-id-7926f47ebc34e11c769acfd3441ab391fc1b9b36/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Refactor (BREAKING)

 - <csr-id-8be4036dce4a857cc14a8b9467aaf2fc0fc2e827/> rename `oid::short_hex()` to `oid::to_hex()`

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
 - <csr-id-c5213d2b701ca71af5f3c987647e2a0c5c4d42dd/> break delete me

### Changed (BREAKING)

<csr-id-67652cb5cf01c45291d6e117c31290c585bab9d1/>
<csr-id-3363f1e61295810964ddb0c255eed87a87fe6539/>
<csr-id-75b901eff177dade43a28e770920a2b2206ded69/>
<csr-id-b596fa0dbbb3cc1d3ac386458ef52e2db9bca55c/>
<csr-id-3373946d27c91169172e62a637a305ef1e5fbb9e/>

 - <csr-id-79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded/> rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()`
   This change was done in the name of consistency, as `from_bytes()` is
   used in many other git-* crates
 - <csr-id-1b75541c00b8a18000336a8a7eceae5beba1058d/> Remove `Kind:Efrom_len_in_bytes()` from public API
   It shouldn't be encouraged to assume the hash can be deduced from its
   length, also git doesn't assume this.
   
   If that would happen, we would have other problems though, so let's hope
   it doesn't happen nonetheless.
 - <csr-id-b12ee8a97904e6e90b6c08ad9e6804ee969bff41/> Remove `ObjectId::null_sha1()` from public API
   Use `Kind::Sha1.null()` instead if it's a value where the actual
   repository object hash doesn't matter.
 - <csr-id-eaf48bd75a3b778e31695257aedfbd008769f7bb/> rename `Kind::null()` to `null_ref()` and `Kind::null_owned()` to `null()`
   This naming is consistent with typical Rust APIs and the naming used
   throughout the git-* crates thus far.
 - <csr-id-60a4eb5dd7f50949799c558a225146d442dcf936/> remove `Kind::new_sha1()` from public API
 - <csr-id-c079fbe2099bd0ba43e811e987a80ae14e15e131/> Kind::from_len_in_bytes() is infallible
 - <csr-id-2a799e662aa172c243b54d1df0dfc78501cb024f/> remove `ObjectId::from_20_bytes()` from public API
   Use `ObjectId::from()` or `ObjectId::try_from()` instead.
 - <csr-id-53c748d7f438f57e8119cdf04402bfeaa9f2a286/> remove various SHA1 specific hex utilities in favor of unspecific new ones
   - removed `to_sha1_hex()`, use `oid::hex_to_buf()` and
   `oid::hex_to_buf()` instead.

### Refactor

 - <csr-id-7926f47ebc34e11c769acfd3441ab391fc1b9b36/> replace `quickerror` with `thiserror`

### Bug Fixes

 - <csr-id-aaed7eaf4887d5e499437d45c8284bc8941da2ac/> don't assume hex-only characters in `ObjectId::from_hex(…)`.
 - <csr-id-d2e2ea0a9b9c5f756d8b02b4872e6950faa03b3e/> don't use panicking const fn just yet to not require rust 1.57

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-05794383cb7c903ab30b5d6ef0178dffdf66feee/> `Prefix::from(ObjectId)`
   This conversion will never fail and is useful as fallback to handle
   failed hash shortenings, which can now default to a prefix that
   represents the original and thus unique hash.
 - <csr-id-652f228bb7ec880856d4e6ee1c171b0b85a735e2/> expose `Prefix::MIN_HEX_LEN`.
   That way other crates can know which candidates to discard off the bat
   instead of having to match on an error. It's mere convenience.
 - <csr-id-535411f94dcab7e7d9cab6324ac30a4c70298bb2/> `Prefix::from_hex()`
 - <csr-id-89f1b27af9acf46744501f4d31cd1298aeff039b/> Implement `TryFrom<&str>` for `Prefix`
   Currently there is no easy way to create a `struct Prefix` from a hex
   string. The method `Parser::from_hex()` is NIY.
 - <csr-id-1be00cf9e00ce9428ffddb2c79b2373926069b13/> `Commit::short_id()`
 - <csr-id-cb83beedd1aa389f6774e2296f79273e8c8f14f4/> git-hash::Prefix::from_id()
   A way to obtain a prefix of an object id, with all non-prefix
   bytes set to zero.
 - <csr-id-bc89fc77354f7d8af6628364be18550c4a45c789/> Implement Display for hash kind
   This helps 'clap' and allows for a little more type-safety during
   declaration.
 - <csr-id-84e26a7f3cbae31210e100880a48d3b3e6d04013/> Assign version numbers to `Kind` and implement `TryFrom<u8>`
   This makes reading and writing the hash number easier for newer file
   formats.
 - <csr-id-ce673bfd9afee4a7872c6bcae1c39006b1747be7/> add `Kind::from_len_in_bytes()` const fn
 - <csr-id-9a0d8b810050f2acabca988c5ab24ebe93a5d260/> `Kind::len_in_bytes()` method
   It yields the amount of bytes needed to store the hash.
 - <csr-id-ed16bce97c235e7e188444afd7a0d3f7e04a6c72/> oid::short_hex(len) for truncated hex representations

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 237 commits contributed to the release over the course of 793 calendar days.
 - 34 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 16 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#222](https://github.com/Byron/gitoxide/issues/222), [#279](https://github.com/Byron/gitoxide/issues/279), [#287](https://github.com/Byron/gitoxide/issues/287), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#331](https://github.com/Byron/gitoxide/issues/331), [#413](https://github.com/Byron/gitoxide/issues/413), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#522](https://github.com/Byron/gitoxide/issues/522), [#63](https://github.com/Byron/gitoxide/issues/63), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 6 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - Deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - Regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Format links for commit ids ([`9426db5`](https://github.com/Byron/gitoxide/commit/9426db53537162d58a65648f3f3a3a3b65f621dc))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - Greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Rename `oid::short_hex()` to `oid::to_hex()` ([`8be4036`](https://github.com/Byron/gitoxide/commit/8be4036dce4a857cc14a8b9467aaf2fc0fc2e827))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Oid::short_hex(len) for truncated hex representations ([`ed16bce`](https://github.com/Byron/gitoxide/commit/ed16bce97c235e7e188444afd7a0d3f7e04a6c72))
 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - Update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - Stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Basic multi-pack index creation ([`89428b2`](https://github.com/Byron/gitoxide/commit/89428b2936fb0169606a543cf531bddaacb8187c))
    - Multi-pack index writing complete with large-offset support ([`f7d5c7f`](https://github.com/Byron/gitoxide/commit/f7d5c7f815dbf52c668444b316ae2e1485463bcb))
    - Assign version numbers to `Kind` and implement `TryFrom<u8>` ([`84e26a7`](https://github.com/Byron/gitoxide/commit/84e26a7f3cbae31210e100880a48d3b3e6d04013))
    - Rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()` ([`79dc0d5`](https://github.com/Byron/gitoxide/commit/79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded))
    - Remove `Kind:Efrom_len_in_bytes()` from public API ([`1b75541`](https://github.com/Byron/gitoxide/commit/1b75541c00b8a18000336a8a7eceae5beba1058d))
    - Remove `ObjectId::null_sha1()` from public API ([`b12ee8a`](https://github.com/Byron/gitoxide/commit/b12ee8a97904e6e90b6c08ad9e6804ee969bff41))
    - Rename `Kind::null()` to `null_ref()` and `Kind::null_owned()` to `null()` ([`eaf48bd`](https://github.com/Byron/gitoxide/commit/eaf48bd75a3b778e31695257aedfbd008769f7bb))
    - Remove `Kind::new_sha1()` from public API ([`60a4eb5`](https://github.com/Byron/gitoxide/commit/60a4eb5dd7f50949799c558a225146d442dcf936))
    - Kind::from_len_in_bytes() is infallible ([`c079fbe`](https://github.com/Byron/gitoxide/commit/c079fbe2099bd0ba43e811e987a80ae14e15e131))
    - Refactor ([`7331e99`](https://github.com/Byron/gitoxide/commit/7331e99cb88df19f7b1e04b1468584e9c7c79913))
    - Remove `ObjectId::from_20_bytes()` from public API ([`2a799e6`](https://github.com/Byron/gitoxide/commit/2a799e662aa172c243b54d1df0dfc78501cb024f))
    - Fix docs ([`cd981e2`](https://github.com/Byron/gitoxide/commit/cd981e222af237c47fcfb74258de8fdfc04dfc1b))
    - Remove various SHA1 specific hex utilities in favor of unspecific new ones ([`53c748d`](https://github.com/Byron/gitoxide/commit/53c748d7f438f57e8119cdf04402bfeaa9f2a286))
    - `oid::null_sha1()` replaced with `Kind::null()` ([`67652cb`](https://github.com/Byron/gitoxide/commit/67652cb5cf01c45291d6e117c31290c585bab9d1))
    - Remove `ObjectId::from_borrowed_sha1()` ([`3363f1e`](https://github.com/Byron/gitoxide/commit/3363f1e61295810964ddb0c255eed87a87fe6539))
    - Remove `ObjectId::to_sha1_hex_string()` ([`75b901e`](https://github.com/Byron/gitoxide/commit/75b901eff177dade43a28e770920a2b2206ded69))
    - SIZE_OF_SHA1_DIGEST is now private ([`b596fa0`](https://github.com/Byron/gitoxide/commit/b596fa0dbbb3cc1d3ac386458ef52e2db9bca55c))
    - Rename `Kind::to_hex()` to `Kind::to_hex_with_len()`; add `Kind::to_hex()` ([`3373946`](https://github.com/Byron/gitoxide/commit/3373946d27c91169172e62a637a305ef1e5fbb9e))
    - Add `Kind::from_len_in_bytes()` const fn ([`ce673bf`](https://github.com/Byron/gitoxide/commit/ce673bfd9afee4a7872c6bcae1c39006b1747be7))
    - `Kind::len_in_bytes()` method ([`9a0d8b8`](https://github.com/Byron/gitoxide/commit/9a0d8b810050f2acabca988c5ab24ebe93a5d260))
 * **[#287](https://github.com/Byron/gitoxide/issues/287)**
    - Very rough version of repository verification ([`80a4a7a`](https://github.com/Byron/gitoxide/commit/80a4a7add688d16376b9bf2ed7f1c7f655b7c912))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - Prepare changelogs for git-index and dependencies ([`f54bf4b`](https://github.com/Byron/gitoxide/commit/f54bf4bde92b892b6d425987a6a37e10319c4635))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Docs ([`a45f378`](https://github.com/Byron/gitoxide/commit/a45f3789696078848e2e96ddb8a55570c941dd53))
    - Implement ODB::disambiguate_prefix(…) ([`7d4d281`](https://github.com/Byron/gitoxide/commit/7d4d2818395cfe0c31117f8736471d4a707e3feb))
    - Support MSRV ([`d09fd9b`](https://github.com/Byron/gitoxide/commit/d09fd9b37557f2dc199e8a4651c56b3b63423136))
    - Add documentation for lookup_prefix along with missing test ([`927b2ac`](https://github.com/Byron/gitoxide/commit/927b2ace875cdda63ce312eb7ad5329f2159608d))
    - Lookup_prefix() seems to work now ([`b558f11`](https://github.com/Byron/gitoxide/commit/b558f111520381e25a9500d3b2401fdd337db6f6))
    - A stab at implementing lookup_prefix - to no avail ([`69cb6d1`](https://github.com/Byron/gitoxide/commit/69cb6d1dd6b8df74fee1ead1ce15bcf0b51d7232))
    - Refactor ([`cff6f9f`](https://github.com/Byron/gitoxide/commit/cff6f9fc90e58c409e367912d0b38860fae9a205))
    - Refactor ([`5bc548e`](https://github.com/Byron/gitoxide/commit/5bc548ed500045491012ab0a93bcbe13e78b0dc8))
    - Prefix now validates all constraints and errors on violation ([`75efa79`](https://github.com/Byron/gitoxide/commit/75efa79f62efc29b343d2d2f53eaf001eef176df))
    - Refactor; add docs ([`837db62`](https://github.com/Byron/gitoxide/commit/837db626b88b08567c059f9f6687ad3124117ed3))
    - Git-hash::Prefix::from_id() ([`cb83bee`](https://github.com/Byron/gitoxide/commit/cb83beedd1aa389f6774e2296f79273e8c8f14f4))
    - Implement Display for hash kind ([`bc89fc7`](https://github.com/Byron/gitoxide/commit/bc89fc77354f7d8af6628364be18550c4a45c789))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - `Commit::short_id()` ([`1be00cf`](https://github.com/Byron/gitoxide/commit/1be00cf9e00ce9428ffddb2c79b2373926069b13))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Update changelog prior to release ([`1d07934`](https://github.com/Byron/gitoxide/commit/1d079346e789b0acc9a4bdf7577b21c1c37b6106))
 * **[#413](https://github.com/Byron/gitoxide/issues/413)**
    - Don't hardcode Sha1 ([`521c894`](https://github.com/Byron/gitoxide/commit/521c894faf8b1875f449c04aa87003066d4c04ff))
    - Refactor ([`85b9f13`](https://github.com/Byron/gitoxide/commit/85b9f13eb29359a34597fb615805d0fa5aac075b))
    - Refactor ([`073d3a1`](https://github.com/Byron/gitoxide/commit/073d3a104725b06279dbfca6d1a35531fa9cb5c5))
    - `Prefix::from_hex()` ([`535411f`](https://github.com/Byron/gitoxide/commit/535411f94dcab7e7d9cab6324ac30a4c70298bb2))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - `Prefix::from(ObjectId)` ([`0579438`](https://github.com/Byron/gitoxide/commit/05794383cb7c903ab30b5d6ef0178dffdf66feee))
    - Expose `Prefix::MIN_HEX_LEN`. ([`652f228`](https://github.com/Byron/gitoxide/commit/652f228bb7ec880856d4e6ee1c171b0b85a735e2))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Refactor ([`93ac4c3`](https://github.com/Byron/gitoxide/commit/93ac4c38e5837250e158613820a6ac1bb7119ba0))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#522](https://github.com/Byron/gitoxide/issues/522)**
    - Don't assume hex-only characters in `ObjectId::from_hex(…)`. ([`aaed7ea`](https://github.com/Byron/gitoxide/commit/aaed7eaf4887d5e499437d45c8284bc8941da2ac))
    - Refactor ([`f3bcddf`](https://github.com/Byron/gitoxide/commit/f3bcddff931e50472e7a3e8c2c60f3cd565bfa56))
    - Refactor ([`0f0de2b`](https://github.com/Byron/gitoxide/commit/0f0de2ba7837e2044e22a16b6d5e20b3137e9691))
 * **[#63](https://github.com/Byron/gitoxide/issues/63)**
    - Revert "Add additional variant for Sha256 in ObjectId" ([`bb24dc4`](https://github.com/Byron/gitoxide/commit/bb24dc44beb6354fe2d96d2318d4d3219f06ae85))
    - Add additional variant for Sha256 in ObjectId ([`3dd7c43`](https://github.com/Byron/gitoxide/commit/3dd7c4350e140b72c21598f95a4557e6115d3124))
    - Make ObjectId into an enum to soon hold more bytes (and type) ([`4bf0c1a`](https://github.com/Byron/gitoxide/commit/4bf0c1a5a5c23bb0c0836ab8cea41eb06a232906))
    - Impl == and != for common combinations of ObjectId/oid ([`2455178`](https://github.com/Byron/gitoxide/commit/24551781cee4fcf312567ca9270d54a95bc4d7ae))
    - Remove now unused gith-hash::borrowed::Id ([`59ab1bd`](https://github.com/Byron/gitoxide/commit/59ab1bd9a8ea57e1770caf8841a0af5d38905bec))
    - More general to-hex for ObjectId ([`e2be868`](https://github.com/Byron/gitoxide/commit/e2be868ad4a131682d4aae629ca5b3a5b7ed0d5f))
    - Fix incorrectly implemented display for `oid` ([`c4186b0`](https://github.com/Byron/gitoxide/commit/c4186b0a986b4b49f8aa70308b492063bd33285c))
    - Git-commitgraph uses `oid` now ([`0b72966`](https://github.com/Byron/gitoxide/commit/0b72966249523b97fce1bc7b29082ac68fa86a4f))
    - Notes about future proofing `oid` type… ([`658c896`](https://github.com/Byron/gitoxide/commit/658c896690f9a5b63f08484e90837bd1338420a5))
    - Use new `oid` where possible in git-odb ([`68a709e`](https://github.com/Byron/gitoxide/commit/68a709e0337d4969138d30a5c25d60b7dbe51a73))
    - Oid with even more conversions and better hex-display ([`eecd664`](https://github.com/Byron/gitoxide/commit/eecd6644b10ba1e2e8481287db85c67ea6268674))
    - Refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com/Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Add quality-of-life parse() support for hex input ([`6f97063`](https://github.com/Byron/gitoxide/commit/6f97063b14eb3b38a36e418657fd50f80db7f905))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com/Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - A seemingly complete implementation of a referenced borrowed Id ([`b3fc365`](https://github.com/Byron/gitoxide/commit/b3fc36565157a7f9d2fc9cf1a3c009a20c66e661))
    - Fix doc string naming ([`59c3d45`](https://github.com/Byron/gitoxide/commit/59c3d454b61e6932aee0fce0f709ac214db08633))
    - Move git-hash::owned::Id into git-hash::Id ([`fdbe704`](https://github.com/Byron/gitoxide/commit/fdbe704b6c9ace2b8f629f681a0580b24749a238))
    - Make git-hash Error usage explicit (it's for decoding only) ([`4805cfc`](https://github.com/Byron/gitoxide/commit/4805cfc8d837bb111424b5e32f46d0fb9b12365a))
    - Rename `git_hash::*::Digest` to `Id` ([`188d90a`](https://github.com/Byron/gitoxide/commit/188d90ad463d342d715af701b03f0ed392c977fc))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
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
    - Adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - Adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - Adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - Adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
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
    - Rename `git-hash` to `gix-hash` ([`416d1dc`](https://github.com/Byron/gitoxide/commit/416d1dcb71b40ca8a23e8793af4d44bbe9847c27))
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
    - Optimize usage of `hex_to_id()` ([`6fa950d`](https://github.com/Byron/gitoxide/commit/6fa950d0ab1991a5577c06385169be1b390dd88a))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Prepare changelogs prior to git-hashtable release ([`3bafb79`](https://github.com/Byron/gitoxide/commit/3bafb795afb901768ac0f3db99c9d2341a3e170f))
    - Make fmt ([`747008d`](https://github.com/Byron/gitoxide/commit/747008d9d370844574dda94e5bec1648c4deb57e))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Merge branch 'optimize_hashtables' ([`95ad56c`](https://github.com/Byron/gitoxide/commit/95ad56c11489bc46d6eb2b2f48cf0bf01e954c58))
    - Switch to custom Hasher implementation ([`269d59e`](https://github.com/Byron/gitoxide/commit/269d59e0bee1f072096667b143800a0d85b18403))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'main' into fetch-pack ([`d686020`](https://github.com/Byron/gitoxide/commit/d6860205db847b8a474756e92578195e1022481c))
    - Thanks clippy ([`b9937ad`](https://github.com/Byron/gitoxide/commit/b9937adc2c31095dde63397be7d56f1ea559b0f7))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Fix docs ([`71cb9ea`](https://github.com/Byron/gitoxide/commit/71cb9eaefb792656b6380fdcc760c2234f9b9fa7))
    - Replace `quickerror` with `thiserror` ([`7926f47`](https://github.com/Byron/gitoxide/commit/7926f47ebc34e11c769acfd3441ab391fc1b9b36))
    - Some more tests ([`400c6cb`](https://github.com/Byron/gitoxide/commit/400c6cb14fe24b5e947f0c7f3a105deecd3b2f84))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - First step towards everything being documented ([`919923c`](https://github.com/Byron/gitoxide/commit/919923c08b641ca148c2f25d193d65bb068cc787))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - Thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'revspec-parsing' ([`a2c8969`](https://github.com/Byron/gitoxide/commit/a2c8969ba821fd387c39b14248074767f54749c8))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Merge branch 'main' into refs-and-worktrees ([`9cf0c7b`](https://github.com/Byron/gitoxide/commit/9cf0c7bd0cc5419137db5796f3a5b91bdf3dcc94))
    - Merge branch 'kalkin-improve-prefix' ([`0866e89`](https://github.com/Byron/gitoxide/commit/0866e89ad498f85478dccfabeb3b3f0b75d65442))
    - Implement `TryFrom<&str>` for `Prefix` ([`89f1b27`](https://github.com/Byron/gitoxide/commit/89f1b27af9acf46744501f4d31cd1298aeff039b))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Update changelogs prior to git-pack release ([`b7e3a4a`](https://github.com/Byron/gitoxide/commit/b7e3a4afdd6417a38aadad35c7f584617e7b47fa))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Don't use panicking const fn just yet to not require rust 1.57 ([`d2e2ea0`](https://github.com/Byron/gitoxide/commit/d2e2ea0a9b9c5f756d8b02b4872e6950faa03b3e))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Better not have items within items in changelogs ([`6946125`](https://github.com/Byron/gitoxide/commit/69461254b1bfda5e60911164096e4a061e241296))
    - Thanks clippy ([`d8925f5`](https://github.com/Byron/gitoxide/commit/d8925f5bd7ac8ef2c98f0e57a1373e5ffba8ce23))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - Break delete me ([`c5213d2`](https://github.com/Byron/gitoxide/commit/c5213d2b701ca71af5f3c987647e2a0c5c4d42dd))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #190] obtain the kind fo hash used in a repo ([`a985491`](https://github.com/Byron/gitoxide/commit/a985491bcea5f76942b863de8a9a89dd235dd0c9))
    - Release git-hash v0.5.1 ([`d826370`](https://github.com/Byron/gitoxide/commit/d826370b88d45fd2a421d3a59c232ed1504c6b0c))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - Thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref] flexible and simple support for different hash lengths ([`9c2edd5`](https://github.com/Byron/gitoxide/commit/9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e))
    - Revert "[ref] parameterize all uses of hash length…" ([`21f187e`](https://github.com/Byron/gitoxide/commit/21f187e6b7011bb59ed935fc1a2d0a5557890ffe))
    - [ref] parameterize all uses of hash length… ([`5c7285e`](https://github.com/Byron/gitoxide/commit/5c7285e7233390fd7589188084fcd05febcbbac2))
    - [ref] handle create-or-append when writing valid reflog files… ([`9175085`](https://github.com/Byron/gitoxide/commit/9175085248855a7ffa0d4e006740eafc0f4e1c92))
    - [ref] another deletion test succeeds ([`6037900`](https://github.com/Byron/gitoxide/commit/60379001d2729627c042f304217d6459f99f01bf))
    - Thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com/Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com/Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - [traversal] trying to get things done with gitoxide shows some teeth… ([`3fee661`](https://github.com/Byron/gitoxide/commit/3fee661af8d67e277e8657606383a670f17e7825))
    - Nicer debug printing for oids, too ([`b4f94f8`](https://github.com/Byron/gitoxide/commit/b4f94f8af662bf6cdc001ca7b59478c701a40e36))
    - A new failing test ([`86b6c24`](https://github.com/Byron/gitoxide/commit/86b6c2497cfa17bf3f822792e3afe406f7968ee7))
    - Fix git-hash docs ([`327a107`](https://github.com/Byron/gitoxide/commit/327a107afd696f7496e04bd6285c217cd8cdc136))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com/Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
    - (cargo-release) version 0.1.2 ([`d1b4436`](https://github.com/Byron/gitoxide/commit/d1b44369bcca34516c8bf86a540a4591d64ec9ba))
    - Update tasks and dependencies ([`96938be`](https://github.com/Byron/gitoxide/commit/96938be512efd6d6ad26238f258865d7488098f4))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
    - (cargo-release) version 0.1.1 ([`4224c5b`](https://github.com/Byron/gitoxide/commit/4224c5b5ceeb6bd1dbe4aac46018be5cc82b77df))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com/Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - First incarnation of git-hash to separate concerns and resolve cycle ([`9803041`](https://github.com/Byron/gitoxide/commit/9803041c29c18f2976531c9b487e63cd90fa3e72))
</details>

## 0.10.1 (2022-12-01)

A maintenance release without user-facing changes.

## 0.10.0 (2022-11-21)

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

## 0.9.11 (2022-10-10)

Maintenance release without user-facing changes.

## 0.9.10 (2022-09-20)

Maintenance release without observable changes.

## 0.9.9 (2022-09-02)

<csr-id-7926f47ebc34e11c769acfd3441ab391fc1b9b36/>

### Bug Fixes

 - <csr-id-aaed7eaf4887d5e499437d45c8284bc8941da2ac/> don't assume hex-only characters in `ObjectId::from_hex(…)`.

### Refactor

 - <csr-id-7926f47ebc34e11c769acfd3441ab391fc1b9b36/> replace `quickerror` with `thiserror`

## 0.9.8 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

## 0.9.7 (2022-08-15)

### New Features

 - <csr-id-05794383cb7c903ab30b5d6ef0178dffdf66feee/> `Prefix::from(ObjectId)`
   This conversion will never fail and is useful as fallback to handle
   failed hash shortenings, which can now default to a prefix that
   represents the original and thus unique hash.

## 0.9.6 (2022-07-22)

This is a maintenance release with no functional changes.

## 0.9.5 (2022-06-13)

### New Features

 - <csr-id-652f228bb7ec880856d4e6ee1c171b0b85a735e2/> expose `Prefix::MIN_HEX_LEN`.
   That way other crates can know which candidates to discard off the bat
   instead of having to match on an error. It's mere convenience.

## 0.9.4 (2022-05-18)

### New Features

 - <csr-id-535411f94dcab7e7d9cab6324ac30a4c70298bb2/> `Prefix::from_hex()`
 - <csr-id-89f1b27af9acf46744501f4d31cd1298aeff039b/> Implement `TryFrom<&str>` for `Prefix`
   Currently there is no easy way to create a `struct Prefix` from a hex
   string. The method `Parser::from_hex()` is NIY.

## 0.9.3 (2022-04-02)

### New Features

 - <csr-id-1be00cf9e00ce9428ffddb2c79b2373926069b13/> `Commit::short_id()`
 - <csr-id-cb83beedd1aa389f6774e2296f79273e8c8f14f4/> gix-hash::Prefix::from_id()
   A way to obtain a prefix of an object id, with all non-prefix
   bytes set to zero.

### Bug Fixes

 - <csr-id-d2e2ea0a9b9c5f756d8b02b4872e6950faa03b3e/> don't use panicking const fn just yet to not require rust 1.57

## 0.9.2 (2022-02-01)

A automated maintenance release without impact to the public API.

### New Features

 - <csr-id-bc89fc77354f7d8af6628364be18550c4a45c789/> Implement Display for hash kind
   This helps 'clap' and allows for a little more type-safety during
   declaration.

## 0.9.1 (2022-01-23)

### Changed (BREAKING)

<csr-id-67652cb5cf01c45291d6e117c31290c585bab9d1/>
<csr-id-3363f1e61295810964ddb0c255eed87a87fe6539/>
<csr-id-75b901eff177dade43a28e770920a2b2206ded69/>
<csr-id-b596fa0dbbb3cc1d3ac386458ef52e2db9bca55c/>
<csr-id-3373946d27c91169172e62a637a305ef1e5fbb9e/>

 - <csr-id-79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded/> rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()`
   This change was done in the name of consistency, as `from_bytes()` is
   used in many other git-* crates
 - <csr-id-1b75541c00b8a18000336a8a7eceae5beba1058d/> Remove `Kind:Efrom_len_in_bytes()` from public API
   It shouldn't be encouraged to assume the hash can be deduced from its
   length, also git doesn't assume this.
   
   If that would happen, we would have other problems though, so let's hope
   it doesn't happen nonetheless.
 - <csr-id-b12ee8a97904e6e90b6c08ad9e6804ee969bff41/> Remove `ObjectId::null_sha1()` from public API
   Use `Kind::Sha1.null()` instead if it's a value where the actual
   repository object hash doesn't matter.
 - <csr-id-eaf48bd75a3b778e31695257aedfbd008769f7bb/> rename `Kind::null()` to `null_ref()` and `Kind::null_owned()` to `null()`
   This naming is consistent with typical Rust APIs and the naming used
   throughout the git-* crates thus far.
 - <csr-id-60a4eb5dd7f50949799c558a225146d442dcf936/> remove `Kind::new_sha1()` from public API
 - <csr-id-c079fbe2099bd0ba43e811e987a80ae14e15e131/> Kind::from_len_in_bytes() is infallible
 - <csr-id-2a799e662aa172c243b54d1df0dfc78501cb024f/> remove `ObjectId::from_20_bytes()` from public API
   Use `ObjectId::from()` or `ObjectId::try_from()` instead.
 - <csr-id-53c748d7f438f57e8119cdf04402bfeaa9f2a286/> remove various SHA1 specific hex utilities in favor of unspecific new ones
   - removed `to_sha1_hex()`, use `oid::hex_to_buf()` and
   `oid::hex_to_buf()` instead.

### New Features

 - <csr-id-bc89fc77354f7d8af6628364be18550c4a45c789/> Implement Display for hash kind
   This helps 'clap' and allows for a little more type-safety during
   declaration.
 - <csr-id-84e26a7f3cbae31210e100880a48d3b3e6d04013/> Assign version numbers to `Kind` and implement `TryFrom<u8>`
   This makes reading and writing the hash number easier for newer file
   formats.
 - <csr-id-ce673bfd9afee4a7872c6bcae1c39006b1747be7/> add `Kind::from_len_in_bytes()` const fn
 - <csr-id-9a0d8b810050f2acabca988c5ab24ebe93a5d260/> `Kind::len_in_bytes()` method
   It yields the amount of bytes needed to store the hash.

### Bug Fixes

 - <csr-id-d2e2ea0a9b9c5f756d8b02b4872e6950faa03b3e/> don't use panicking const fn just yet to not require rust 1.57

## 0.9.0 (2022-01-19)

### New Features

 - <csr-id-84e26a7f3cbae31210e100880a48d3b3e6d04013/> Assign version numbers to `Kind` and implement `TryFrom<u8>`
   This makes reading and writing the hash number easier for newer file
   formats.
 - <csr-id-ce673bfd9afee4a7872c6bcae1c39006b1747be7/> add `Kind::from_len_in_bytes()` const fn
 - <csr-id-9a0d8b810050f2acabca988c5ab24ebe93a5d260/> `Kind::len_in_bytes()` method
   It yields the amount of bytes needed to store the hash.

### Changed (BREAKING)

 - <csr-id-79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded/> rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()`
   This change was done in the name of consistency, as `from_bytes()` is
   used in many other git-* crates
 - <csr-id-1b75541c00b8a18000336a8a7eceae5beba1058d/> Remove `Kind:Efrom_len_in_bytes()` from public API
   It shouldn't be encouraged to assume the hash can be deduced from its
   length, also git doesn't assume this.
   
   If that would happen, we would have other problems though, so let's hope
   it doesn't happen nonetheless.
 - <csr-id-b12ee8a97904e6e90b6c08ad9e6804ee969bff41/> Remove `ObjectId::null_sha1()` from public API
   Use `Kind::Sha1.null()` instead if it's a value where the actual
   repository object hash doesn't matter.
 - <csr-id-eaf48bd75a3b778e31695257aedfbd008769f7bb/> rename `Kind::null()` to `null_ref()` and `Kind::null_owned()` to `null()`
   This naming is consistent with typical Rust APIs and the naming used
   throughout the git-* crates thus far.
 - <csr-id-60a4eb5dd7f50949799c558a225146d442dcf936/> remove `Kind::new_sha1()` from public API
 - <csr-id-c079fbe2099bd0ba43e811e987a80ae14e15e131/> Kind::from_len_in_bytes() is infallible
 - <csr-id-2a799e662aa172c243b54d1df0dfc78501cb024f/> remove `ObjectId::from_20_bytes()` from public API
   Use `ObjectId::from()` or `ObjectId::try_from()` instead.
 - <csr-id-53c748d7f438f57e8119cdf04402bfeaa9f2a286/> remove various SHA1 specific hex utilities in favor of unspecific new ones.
   
   removed `to_sha1_hex()`, use `oid::hex_to_buf()` and `oid::hex_to_buf()` instead.
   remove `ObjectId::write_hex_to()` in favor of `oid::write_hex_to()`
 - <csr-id-67652cb5cf01c45291d6e117c31290c585bab9d1/> `oid::null_sha1()` replaced with `Kind::null()`
 - <csr-id-3363f1e61295810964ddb0c255eed87a87fe6539/> remove `ObjectId::from_borrowed_sha1()`
 - <csr-id-75b901eff177dade43a28e770920a2b2206ded69/> remove `ObjectId::to_sha1_hex_string()`
   Use `.to_hex().to_string()` instead.
 - <csr-id-b596fa0dbbb3cc1d3ac386458ef52e2db9bca55c/> SIZE_OF_SHA1_DIGEST is now private
   Replace it with your own constant derived from
 - <csr-id-3373946d27c91169172e62a637a305ef1e5fbb9e/> rename `Kind::to_hex()` to `Kind::to_hex_with_len()`; add `Kind::to_hex()`
   The latter prints the oid in full.

## v0.8.0 (2021-10-19)

<csr-id-c5213d2b701ca71af5f3c987647e2a0c5c4d42dd/>

A maintenance release due to reset the entire crate graph to new minor releases.

## v0.7.0 (2021-10-15)

<csr-id-8be4036dce4a857cc14a8b9467aaf2fc0fc2e827/>
<csr-id-ed16bce97c235e7e188444afd7a0d3f7e04a6c72/>

### BREAKING Changes

 - rename `oid::short_hex()` to `oid::to_hex()`
 - `oid::short_hex(len)` for truncated hex representations

## v0.6.0 (2021-09-07)

### Breaking

- `ObjectId::empty_tree()` now has a parameter: `Kind`
- `ObjectId::null_sha(…)` -> `ObjectId::null(…)`

## v0.5.1 (2021-08-17)

## v0.5.0 (2021-08-10)

## v0.3.0 (2021-04-30)

## v0.2.0 (2021-04-08)

## v0.1.2 (2021-01-12)

## v0.1.1 (2020-12-16)

## v0.1.0 (2020-12-16)

