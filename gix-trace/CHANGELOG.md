# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.1 (2023-06-22)

### New Features

 - <csr-id-83895f964c8308902d083f2fd98a457926db24b3/> `Span::record()` to allow recording a value after the span was created.
 - <csr-id-aa9dc8f09826790d1259140f6026ff116d943ac1/> add `Span::into_scope()`
   A way to conveniently auto-drop a span after executing a closure.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 5 calendar days.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

