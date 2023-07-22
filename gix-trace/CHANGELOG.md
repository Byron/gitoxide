# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.3 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 23 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.1.2 (2023-06-29)

### New Features

 - <csr-id-d5eba46afeaadb9ff113421bdb43466d67cfeac6/> Support for events (e.g. warn!()) via macro.
   Thanks to these, one can now emit warnings which could be visible
   if the application compiled in support for it.
   
   This is a nice alternative compared to having to have a `progress` around.
 - <csr-id-ba777f3978e5c7f0275ad48a54d85ec63609aead/> panic on `record()` if field wasn't found.
   Even though this is a breaking change, it happens at runtime and should be helpful.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Prepare changelogs prior to release ([`00f96fb`](https://github.com/Byron/gitoxide/commit/00f96fb3110a8f81a1bd0d74c757c15b8773c6f6))
    - Merge branch 'basic-filtering' ([`3fd5e16`](https://github.com/Byron/gitoxide/commit/3fd5e16e205db18edc21341fb4c2a75d0726f5a5))
    - Support for events (e.g. warn!()) via macro. ([`d5eba46`](https://github.com/Byron/gitoxide/commit/d5eba46afeaadb9ff113421bdb43466d67cfeac6))
    - Panic on `record()` if field wasn't found. ([`ba777f3`](https://github.com/Byron/gitoxide/commit/ba777f3978e5c7f0275ad48a54d85ec63609aead))
</details>

## 0.1.1 (2023-06-22)

### New Features

 - <csr-id-83895f964c8308902d083f2fd98a457926db24b3/> `Span::record()` to allow recording a value after the span was created.
 - <csr-id-aa9dc8f09826790d1259140f6026ff116d943ac1/> add `Span::into_scope()`
   A way to conveniently auto-drop a span after executing a closure.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 5 calendar days.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Prepare changelogs prior to release ([`18b0a37`](https://github.com/Byron/gitoxide/commit/18b0a371941aa2d4d62512437d5daa351ba99ffd))
    - `just fmt` ([`871dd0b`](https://github.com/Byron/gitoxide/commit/871dd0b977caf17159092a4739ba5408403cdb2c))
    - Merge branch 'gix-corpus' ([`5861afb`](https://github.com/Byron/gitoxide/commit/5861afb45f32c16eefcd8e7b7480309bf44b6edc))
    - `Span::record()` to allow recording a value after the span was created. ([`83895f9`](https://github.com/Byron/gitoxide/commit/83895f964c8308902d083f2fd98a457926db24b3))
    - Add `Span::into_scope()` ([`aa9dc8f`](https://github.com/Byron/gitoxide/commit/aa9dc8f09826790d1259140f6026ff116d943ac1))
    - Refactor ([`2b37e25`](https://github.com/Byron/gitoxide/commit/2b37e25f7bb8d5be9803e876771d3adf807fbe0e))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
</details>

## v0.1.0 (2023-06-16)

### New Features

 - <csr-id-093efafa7c39aa03bfef4894779cca6e3716f471/> add `tracing` feature toggle to provide minimal tracing API
   This API is based on `tracing-core`, not on tracing, and provides a limited
   API that is always available, while being a no-op if `tracing` isn't enabled.
   
   That way, plumbing crates can add instrumentation at will.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-trace v0.1.0 ([`2ab69c6`](https://github.com/Byron/gitoxide/commit/2ab69c6fd142fcbbf6df3f981b1550b0c8167a04))
    - Add changelog prior to release of `gix-trace` ([`e1305c3`](https://github.com/Byron/gitoxide/commit/e1305c34092cdb067fbe597050bba4927cff491c))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Add `tracing` feature toggle to provide minimal tracing API ([`093efaf`](https://github.com/Byron/gitoxide/commit/093efafa7c39aa03bfef4894779cca6e3716f471))
</details>

